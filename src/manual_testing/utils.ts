import { invokeContract, invokeCustomContract } from "../utils/contract.js";
import { 
  Address, 
  Asset, 
  BASE_FEE, 
  Horizon, 
  Keypair, 
  nativeToScVal, 
  Operation, 
  scValToNative, 
  TransactionBuilder,
  xdr, 
} from "@stellar/stellar-sdk";
import { deployStellarAsset } from "../utils/contract.js";
import { getCurrentTimePlusOneHour, signWithKeypair } from "../utils/tx.js";
import * as PhoenixFactoryContract from '../protocols/phoenix/bindgins/factory_bindings.js';
import { AddressBook } from '../utils/address_book.js';
import { config } from "../utils/env_config.js";

const network = process.argv[2];
const addressBook = AddressBook.loadFromFile(network);
const loadedConfig = config(network);

const fetchAssetBalance = async (asset: Asset, account: Keypair) => {
  console.log("-------------------------------------------------------");
  console.log(`Fetching ${asset.code} balance in ${account.publicKey()}`);
  console.log("-------------------------------------------------------");
  let balance;
  try {
    balance = await invokeCustomContract(
      asset.contractId(loadedConfig.passphrase),
      "balance",
      [new Address(account.publicKey()).toScVal()],
      account,
      true
    );
  } catch (error:any) {
    if(error.toString().includes('#13')){
      console.log(`🔴 Missing ${asset.code} trustline in ${account.publicKey}`)
      return undefined;
    } else {
      throw new Error("🔴 Can't set trustline", error)
    }
  } 
  if(balance != undefined){
    const parsedBalance: bigint = scValToNative(balance.result.retval);
    return parsedBalance / BigInt(10000000);
  }
}

const fetchContractBalance = async (contractID: string, account: Keypair) => {
  console.log("-------------------------------------------------------");
  console.log(`Fetching ${contractID} balance in ${account.publicKey()}`);
  console.log("-------------------------------------------------------");
  let balance;
  try {
    balance = await invokeCustomContract(
      contractID,
      "balance",
      [new Address(account.publicKey()).toScVal()],
      account,
      true
    );
  } catch (error:any) {
    if(error.toString().includes('MissingValue')){
      console.log(`🔴 contract not deployed or balance not found`)
      return 0;
    } else {
      throw new Error("🔴 Can't set trustline", error)
    }
  } 
  if(balance != undefined){
    const parsedBalance: bigint = scValToNative(balance.result.retval);
    return parsedBalance / BigInt(10000000);
  }
}

const setTrustline = async (asset: Asset, account: Keypair, rpc: Horizon.Server, limit?: string,) => {
  console.log(`🟡 Setting trustline for ${asset.code} with account ${account.publicKey()}`)
  const loadedAccount: Horizon.AccountResponse = await rpc.loadAccount(account.publicKey());
  const operation =  Operation.changeTrust({
    asset: asset,
    limit: limit || undefined
  })
  const transaction = new TransactionBuilder(loadedAccount, {
    fee: Number(BASE_FEE).toString(),
    timebounds: { minTime: 0, maxTime: 0 },
    networkPassphrase: loadedConfig.passphrase,
  })
    .addOperation(operation)
    .setTimeout(300)
    .build();
  await transaction.sign(account);
  const transactionResult = await rpc.submitTransaction(transaction);
  if(transactionResult.successful) {
    console.log(`✨Trustline for ${asset.code} set`)
  }
  return transactionResult;
}

const mintToken = async (destination: string, asset: Asset, amount: string, source: Keypair, rpc: Horizon.Server, passphrase: string,) => {
  console.log('-------------------------------------------------------');
  console.log(`Minting ${amount} ${asset.code} to ${destination}`);
  console.log('-------------------------------------------------------');
  const loadedSource = await rpc.loadAccount(source.publicKey());
  const operation = Operation.payment({
    destination: destination,
    asset: asset,
    amount: amount
  })

  const transaction = new TransactionBuilder(loadedSource, {
    fee: BASE_FEE,
    networkPassphrase: passphrase
  })
    .addOperation(operation)
    .setTimeout(300)
    .build();
    await transaction.sign(source);
  const transactionResult = await rpc.submitTransaction(transaction);
  if(transactionResult.successful) {
    console.log(`✨Payment of ${amount} ${asset.code} to ${destination} successful`)
  }
  return transactionResult;
}

const formatAmmount = (amount: number) => {
  const formattedAmmount = BigInt(amount * 10000000).toString();
  return formattedAmmount
}
interface CreatePoolParams {
  contractID_A: string,
  contractID_B: string,
  user: Keypair,
  amount_A: number,
  amount_B: number,
}

const create_soroswap_liquidity_pool = async (
    router:string,  
    poolParams: CreatePoolParams,
  ) => {
  console.log('🟡 Creating soroswap liquidity pool')
  const parsedPoolParams = {
    ...poolParams,
    user: poolParams.user.publicKey()
  }
  console.log('🚀 « poolParams:', parsedPoolParams);  
  const addSoroswapLiquidityParams: xdr.ScVal[] = [
    new Address(poolParams.contractID_A).toScVal(),
    new Address(poolParams.contractID_B).toScVal(),
    nativeToScVal(formatAmmount(poolParams.amount_A), { type: "i128" }),
    nativeToScVal(formatAmmount(poolParams.amount_B), { type: "i128" }),
    nativeToScVal(0, { type: "i128" }),
    nativeToScVal(0, { type: "i128" }),
    new Address(poolParams.user.publicKey()).toScVal(),
    nativeToScVal(getCurrentTimePlusOneHour(), { type: "u64" }),
  ];
  const soroswapInvoke = await invokeCustomContract(router, 'add_liquidity', addSoroswapLiquidityParams, poolParams.user)
  if(soroswapInvoke.status === 'SUCCESS'){
    console.log('🟢 Soroswap pool created successfully')
  } else {
    console.log('🚀 « soroswapInvoke:', soroswapInvoke);
  }
  return soroswapInvoke;
}

const create_phoenix_pool_transaction = async (
  factory_contract: PhoenixFactoryContract.Client, 
  phoenixAdmin: Keypair, 
  aggregatorAdmin:Keypair, 
  assetA:Asset, 
  assetB:Asset)=>{
 
  let firstAsset: Asset;
  let secondAsset: Asset;
  if(assetA.contractId(loadedConfig.passphrase) > assetB.contractId(loadedConfig.passphrase)){
    firstAsset = assetB;
    secondAsset = assetA;
  } else {
    firstAsset = assetA;
    secondAsset = assetB;
  }
  const tx = await factory_contract.create_liquidity_pool({
    sender: phoenixAdmin.publicKey(),
    lp_init_info: {
      admin: phoenixAdmin.publicKey(),
      fee_recipient: loadedConfig.testUser.publicKey(),
      max_allowed_slippage_bps: 4000n,
      max_allowed_spread_bps: 400n,
      max_referral_bps: 5000n,
      swap_fee_bps: 0n,
      stake_init_info: {
        manager: aggregatorAdmin.publicKey(),
        max_complexity: 10,
        min_bond: 6n,
        min_reward: 3n
      },
      token_init_info: {
        token_a: secondAsset.contractId(loadedConfig.passphrase),
        token_b: firstAsset.contractId(loadedConfig.passphrase),
      }
    },
    share_token_name: `TOKEN-LP-${firstAsset.code}/${secondAsset.code}`,
    share_token_symbol: `PLP-${firstAsset.code}/${secondAsset.code}`,
  });
  return await tx.signAndSend().catch((error:any)=>{
    throw new Error(error)
  })
}

const create_phoenix_liquidity_pool = async (phoenixAdmin: Keypair, aggregatorAdmin:Keypair, testUser:Keypair, assetA:Asset, assetB:Asset)=>{
  const factory_contract = new PhoenixFactoryContract.Client({
    publicKey: phoenixAdmin.publicKey()!,
    contractId: addressBook.getContractId("phoenix_factory"),
    networkPassphrase: loadedConfig.passphrase,
    rpcUrl: "https://soroban-testnet.stellar.org/",
    signTransaction: (tx: string) => signWithKeypair(tx, loadedConfig.passphrase, phoenixAdmin),
  });
  try {
    console.log('creating phoenix pool transaction')
    await create_phoenix_pool_transaction(factory_contract, phoenixAdmin, aggregatorAdmin, assetA, assetB)
  } catch (error:any) {
    if(error.toString().includes('ExistingValue')){
      console.log('Pool already exists')
    } else {
      console.log('🚀 « error:', error)
    }
  }
  
  console.log("Getting pair address")
  const getPairParams: xdr.ScVal[] = [
    new Address(assetA.contractId(loadedConfig.passphrase)).toScVal(),
    new Address(assetB.contractId(loadedConfig.passphrase)).toScVal()
  ]
  const pairAddress = await invokeContract('phoenix_factory', addressBook, 'query_for_pool_by_token_pair', getPairParams, phoenixAdmin, true)
  console.log('🚀 « pairAddress:', scValToNative(pairAddress.result.retval));
  if(pairAddress.result){
    return scValToNative(pairAddress.result.retval);
  } else {
    return pairAddress;
  }
}

const provide_phoenix_liquidity = async (phoenixAdmin: Keypair, pairAddress:string, amount_A: number, amount_B:number)=>{
  const addPhoenixLiquidityParams: xdr.ScVal[] = [
    new Address(phoenixAdmin.publicKey()).toScVal(),
    nativeToScVal(formatAmmount(amount_A), { type: "i128" }),
    nativeToScVal(null),
    nativeToScVal(formatAmmount(amount_B), { type: "i128" }),
    nativeToScVal(null),
    nativeToScVal(null)
  ]

  const provide_liquidity = await invokeCustomContract(pairAddress, 'provide_liquidity', addPhoenixLiquidityParams, phoenixAdmin)

  if(provide_liquidity.status === 'SUCCESS'){
    console.log('🟢 Successfully added liquidity to phoenix pool')
    return provide_liquidity;
  } else {
    console.log('🔴 error providing liquidity:', provide_liquidity)
    return provide_liquidity;
  }
}

interface DexDistributionRaw {
  protocol_id: string,
  path: string[],
  parts: number,
}
const createDexDistribution = async (dexDistributionRaw: DexDistributionRaw[]) => {
  const dexDistributionScVal = dexDistributionRaw.map((distribution) => {
    return xdr.ScVal.scvMap([
      new xdr.ScMapEntry({
        key: xdr.ScVal.scvSymbol('parts'),
        val: nativeToScVal(distribution.parts, {type: "u32"}),
      }),
      new xdr.ScMapEntry({
        key: xdr.ScVal.scvSymbol('path'),
        val: xdr.ScVal.scvVec(distribution.path.map((pathAddress) => new Address(pathAddress).toScVal())),
      }),
      new xdr.ScMapEntry({
        key: xdr.ScVal.scvSymbol('protocol_id'),
        val: nativeToScVal(distribution.protocol_id),
      }),
    ]);
  });

  const dexDistributionScValVec = xdr.ScVal.scvVec(dexDistributionScVal);
  return dexDistributionScValVec;
}

export enum SwapMethod {
  EXACT_INPUT = 'swap_exact_tokens_for_tokens',
  EXACT_OUTPUT = 'swap_tokens_for_exact_tokens',
}
const callAggregatorSwap = async (asset_a:string, asset_b:string, max_amount: number, dexDistributionScValVec: xdr.ScVal, user: Keypair, method: SwapMethod ) => {
//  fn swap_exact_tokens_for_tokens(
//    token_in: Address,
//    token_out: Address,
//    amount_in: i128,
//    amount_out_min: i128,
//    distribution: Vec<DexDistribution>,
//    to: Address,
//    deadline: u64,
//)

//fn swap_tokens_for_exact_tokens(
//  token_in: Address,
//  token_out: Address,
//  amount_out: i128,
//  amount_in_max: i128,
//  distribution: Vec<DexDistribution>,
//  to: Address,
//  deadline: u64,
//)

let aggregatorSwapParams: xdr.ScVal[];
switch (method) {
  case SwapMethod.EXACT_INPUT:
    aggregatorSwapParams = [
      new Address(asset_a).toScVal(),
      new Address(asset_b).toScVal(), 
      nativeToScVal(formatAmmount(max_amount), {type: "i128"}),
      nativeToScVal(formatAmmount(0), {type: "i128"}),
      dexDistributionScValVec, 
      new Address(user.publicKey()).toScVal(), 
      nativeToScVal(getCurrentTimePlusOneHour(), {type:'u64'}),
    ];
    break;
  case SwapMethod.EXACT_OUTPUT:
    aggregatorSwapParams = [
      new Address(asset_a).toScVal(),
      new Address(asset_b).toScVal(), 
      nativeToScVal(formatAmmount(max_amount), {type: "i128"}),
      nativeToScVal(formatAmmount(max_amount+20000000), {type: "i128"}),
      dexDistributionScValVec, 
      new Address(user.publicKey()).toScVal(), 
      nativeToScVal(getCurrentTimePlusOneHour(), {type:'u64'}),
    ];
    break;
  default:
    throw new Error('Invalid swap method');
}

  console.log(`🟡 Calling aggregator ${method}`)
  const aggregatorResponse = await invokeContract(
    'aggregator',
    addressBook,
    method,
    aggregatorSwapParams,
    user
  );
  if(aggregatorResponse.status === 'SUCCESS'){
    console.log(`✨ Aggregator ${method} successful`)
    const parsedResponse = scValToNative(aggregatorResponse.returnValue)
    return parsedResponse;
  } else {
    console.log('🔴 error calling aggregator:', aggregatorResponse)
  }
}

export {
  fetchAssetBalance,
  fetchContractBalance,
  setTrustline,
  mintToken, 
  deployStellarAsset,
  create_soroswap_liquidity_pool,
  create_phoenix_liquidity_pool,
  provide_phoenix_liquidity,
  createDexDistribution,
  callAggregatorSwap
}