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
  xdr 
} from "@stellar/stellar-sdk";
import type {
  Option,
} from "@stellar/stellar-sdk/contract";
import { deployStellarAsset } from "../utils/contract.js";
import { getCurrentTimePlusOneHour, signWithKeypair } from "../utils/tx.js";
import * as PhoenixFactoryContract from '../protocols/phoenix/bindgins/factory_bindings.js';
import { AddressBook } from '../utils/address_book.js';
import { config } from "../utils/env_config.js";
import { randomBytes } from "crypto";


const network = process.argv[2];
const addressBook = AddressBook.loadFromFile(network);
const loadedConfig = config(network);

const name_parts = [
  "zi", "ay", "vo", "ak", "rd", "pi", "nv", "ku", "or", "fx",
  "ba", "un", "ve", "uy", "pr", "ot", "ml", "kr", "ix", "gx",
  "de", "xi", "to", "iv", "ra", "ne", "lz", "ke", "am", "fz",
  "cy", "ax", "ui", "ez", "rg", "pe", "nl", "lo", "ib", "jh",
  "gu", "ep", "ww", "tv", "su", "rx", "nu", "ox", "kx",
];

const generateRandomCode = () => {
  const part1 = name_parts[Math.floor(Math.random() * name_parts.length)];
  const part2 = name_parts[Math.floor(Math.random() * name_parts.length)];
  const formatCode = (part1+part2).toUpperCase().substring(0, 4);
  return formatCode; 
}

const generateRandomAsset = () => {
  const asset = new Asset(generateRandomCode(), loadedConfig.tokenAdmin.publicKey());
  return asset;
}

const fetchAssetBalance = async (asset: Asset, account: Keypair) => {
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
    //return (parsedBalance / BigInt(10000000)).toString();
    return parsedBalance;
  }
}

const fetchContractBalance = async (contractID: string, account: Keypair) => {
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
    //return parsedBalance / BigInt(10000000);
    return parsedBalance;
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
    console.log(`🟢 Trustline for ${asset.code} set`)
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
    console.log(`🟢 Payment of ${amount} ${asset.code} to ${destination} successful`)
  }
  return transactionResult;
}

const deployAndMint = async (asset: Asset, user: Keypair, amount:string)=>{
  try {
    console.log(`🟡 Deploying contract for ${asset.code}`);
    await deployStellarAsset(asset, loadedConfig.tokenAdmin);

  } catch (error:any) {
    if(error.toString().includes('ExistingValue')){
      console.log(`🟢 Contract for ${asset.code} already exists`);
    } else {
      console.error(error);
    }
  };

  const userHasTrustline = await fetchAssetBalance(asset, user);
  if(!userHasTrustline){
    console.log(`Missing trustline for ${asset.code} in ${user.publicKey()}`);
    try{
      await setTrustline(asset, user, loadedConfig.horizonRpc);
    } catch(e:any){
      console.error(e);
    }  
  } else {
    console.log(`🟢 Trustline for ${asset.code} already exists in ${user.publicKey()}`);
    console.log(`🟢 Balance: ${userHasTrustline}`);
  }
  

  await mintToken(user.publicKey(), asset, amount, loadedConfig.tokenAdmin, loadedConfig.horizonRpc, loadedConfig.passphrase);
  const newUserBalance = await fetchAssetBalance(asset, user);
  console.log(`🟢 Test user balance of ${asset.code}: ${newUserBalance}`);
}

const formatAmmount = (amount: number) => {
  //const formattedAmmount = BigInt(amount * 10000000).toString();
  //return formattedAmmount
  return amount
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
  console.log('🔎 poolParams:', parsedPoolParams);  
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
    console.log('🔎 soroswapInvoke:', soroswapInvoke);
  }
  return soroswapInvoke;
}
export interface SoroswapPool {
  address: string,
  asset_0: string,
  asset_0_balance: number,
  asset_1: string,
  asset_1_balance: number,
}
const createSoroswapLP = async (addresses: string[], amount_A: number, amount_B: number, router: string, user: Keypair)=>{
  let soroswapPools = [];
  const poolParams = {
    contractID_A: addresses[0],
    contractID_B: addresses[1],
    user: user,
    amount_A: amount_A,
    amount_B: amount_B,
  };

  await create_soroswap_liquidity_pool(router, poolParams);
  const fetchPoolParams: xdr.ScVal[] = [
    new Address(addresses[0]).toScVal(),
    new Address(addresses[1]).toScVal(),
  ];
  console.log('🟡 Fetching Soroswap pair address');
  try {
    const soroswapPool = await invokeCustomContract(router, 'router_pair_for', fetchPoolParams, user, true);
    const soroswapPoolCID = scValToNative(soroswapPool.result.retval);
    console.log('🟡 Fetching liquidity pool balance');
    const soroswapPoolBalance = await invokeCustomContract(soroswapPoolCID, 'get_reserves', [], user, true);
    const parsedPoolBalance = scValToNative(soroswapPoolBalance.result.retval);
    console.log(`🟢 Soroswap pair balance: ${scValToNative(soroswapPoolBalance.result.retval)}`);
    console.log('🟢 Soroswap pair address:', soroswapPoolCID)
    const soroswap_pool_result = {
      address: soroswapPoolCID,
      asset_0: addresses[0],
      asset_0_balance: parsedPoolBalance[0],
      asset_1: addresses[1],
      asset_1_balance: parsedPoolBalance[1],
    }
    return soroswap_pool_result;
  } catch (e) {
    console.error(e)
  }
}

const create_phoenix_pool_transaction = async (
  factory_contract: PhoenixFactoryContract.Client, 
  phoenixAdmin: Keypair, 
  aggregatorAdmin:Keypair, 
  assetA:Asset, 
  assetB:Asset)=>{
 
  let firstAsset: Asset = assetA
  let secondAsset: Asset = assetB
 /*  if(assetA.contractId(loadedConfig.passphrase) > assetB.contractId(loadedConfig.passphrase)){
    firstAsset = assetB;
    secondAsset = assetA;
  } else {
    firstAsset = assetA;
    secondAsset = assetB;
  } */
  const tx = await factory_contract.create_liquidity_pool({
    sender: phoenixAdmin.publicKey(),
    lp_init_info: {
      admin: phoenixAdmin.publicKey(),
      swap_fee_bps: 0n,
      fee_recipient: loadedConfig.testUser.publicKey(),
      max_allowed_slippage_bps: 4000n,
      default_slippage_bps: 2500n,
      max_allowed_spread_bps: 4000n,
      max_referral_bps: 5000n,
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
    pool_type: PhoenixFactoryContract.PoolType.Xyk,
    amp: 0n,
    default_slippage_bps: 100n,
    max_allowed_fee_bps: 2000n,

  });
  return await tx.signAndSend().catch((error:any)=>{
    throw new Error(error)
  })
}

const create_phoenix_liquidity_pool = async (phoenixAdmin: Keypair, aggregatorAdmin:Keypair, assetA:Asset, assetB:Asset)=>{
  const factory_contract = new PhoenixFactoryContract.Client({
    publicKey: phoenixAdmin.publicKey()!,
    contractId: addressBook.getContractId("phoenix_factory"),
    networkPassphrase: loadedConfig.passphrase,
    rpcUrl: "https://soroban-testnet.stellar.org/",
    signTransaction: (tx: string) => signWithKeypair(tx, loadedConfig.passphrase, phoenixAdmin),
  });
  let needRetry: boolean = false;
  try {
    console.log('creating phoenix pool transaction')
    await create_phoenix_pool_transaction(factory_contract, phoenixAdmin, aggregatorAdmin, assetA, assetB)
  } catch (error:any) {
    if(error.toString().includes('ExistingValue')){
      console.log('Pool already exists')
    }
    if(error.toString().includes('#5')){
      console.log('Token A bigger than token B, retrying with token B as first asset')
      needRetry = true;
    } 
    else {
      console.log('🔴 error:', error)
    }
  }

  if(needRetry){
    try {
      await create_phoenix_pool_transaction(factory_contract, phoenixAdmin, aggregatorAdmin, assetB, assetA)
    } catch (error:any) {
      if(error.toString().includes('ExistingValue')){
        console.log('🟡 Pool already exists')
      } else {
        console.log('🔴 error:', error)
      }
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

const getPhoenixBalanceForContract = (contractID:string, balancesObject: any)=>{
  for(let asset in balancesObject)  {
    if(balancesObject[asset].address === contractID){
      return balancesObject[asset].amount;
    }
  }  
}

interface PhoenixPool {
  phoenix_pool_address: string,
  asset_a_address: string,
  asset_a_amount: string,
  asset_b_address: string,
  asset_b_amount: string,
  asset_lp_address: string,
  asset_lp_amount: string,
  stake_address: string,
}

export interface CometPoolParams{
  asset_a: string,
  asset_b: string,
  weight_a: number,
  weight_b: number,
  amount_a: number,
  amount_b: number,
  user: Keypair
}

export interface CometPool {
  address: string,
  asset_0: string,
  asset_0_balance: number,
  asset_1: string,
  asset_1_balance: number,
  adapter_name: string,
}

function generateAdapterIdForComet() {
  const chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
  let result = "";
  for (let i = 0; i < 10; i++) {
    result += chars.charAt(Math.floor(Math.random() * chars.length));
  }
  return result;
}


export async function createCometPool(params: CometPoolParams): Promise<CometPool> {
    console.log('creating comet pair...');
    const createCometPairResponse = await invokeContract(
      'comet_factory',
      addressBook,
      'new_c_pool',
      [
        nativeToScVal(randomBytes(32)),
        new Address(params.user.publicKey()).toScVal(),
        nativeToScVal([new Address(params.asset_a).toScVal(), new Address(params.asset_b).toScVal()]),
        nativeToScVal([
          nativeToScVal(params.weight_a, { type: 'i128' }),
          nativeToScVal(params.weight_b, { type: 'i128' }),
        ]),
        nativeToScVal([
          nativeToScVal(params.amount_a, { type: 'i128' }),
          nativeToScVal(params.amount_b, { type: 'i128' }),
        ]),
        nativeToScVal(30000, { type: 'i128' }),
      ],
      params.user
    );
    const cometPairAddress = scValToNative(createCometPairResponse.returnValue)

    const initArgs = xdr.ScVal.scvVec([
      xdr.ScVal.scvString("comet"),
      new Address(cometPairAddress).toScVal(),
    ]);
  
    const cometAdapterDeployParams: xdr.ScVal[] = [
      new Address(loadedConfig.admin.publicKey()).toScVal(),
      nativeToScVal(Buffer.from(addressBook.getWasmHash('comet_adapter'), 'hex')),
      nativeToScVal(randomBytes(32)),
      xdr.ScVal.scvSymbol('initialize'),
      initArgs,
    ];
  
    const response = await invokeContract(
      'deployer',
      addressBook,
      'deploy',
      cometAdapterDeployParams,
      loadedConfig.admin
    );
  
    const cometAdapterAddress = scValToNative(response.returnValue)[0];
    console.log('🚀 « comet adapter address:', cometAdapterAddress);
    // SAVE ADDRES IN ADDRESS BOOK
    addressBook.setContractId('comet_adapter', cometAdapterAddress);

    const adapter_name = generateAdapterIdForComet()

    const adaptersVec = [
      {
        protocol_id: adapter_name,
        address: new Address(cometAdapterAddress),
        paused: false
      }
    ];

    const adaptersVecScVal = xdr.ScVal.scvVec(adaptersVec.map((adapter) => {
      return xdr.ScVal.scvMap([
        new xdr.ScMapEntry({
          key: xdr.ScVal.scvSymbol('address'),
          val: adapter.address.toScVal(),
        }),
        new xdr.ScMapEntry({
          key: xdr.ScVal.scvSymbol('paused'),
          val: nativeToScVal(adapter.paused),
        }),
        new xdr.ScMapEntry({
          key: xdr.ScVal.scvSymbol('protocol_id'),
          val: xdr.ScVal.scvString(adapter.protocol_id),
        }),
      ]);
    }));

      const aggregatorUpdateAdaptersParams: xdr.ScVal[] = [
        adaptersVecScVal,
      ];
    
      await invokeContract(
        'aggregator',
        addressBook,
        'update_adapters',
        aggregatorUpdateAdaptersParams,
        loadedConfig.admin
      );

    return{
      address: cometPairAddress,
      asset_0: params.asset_a,
      asset_0_balance: params.amount_a,
      asset_1: params.asset_b,
      asset_1_balance: params.amount_b,
      adapter_name
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
// fn swap_exact_tokens_for_tokens(
//   token_in: Address,
//   token_out: Address,
//   amount_in: i128,
//   amount_out_min: i128,
//   distribution: Vec<DexDistribution>,
//   to: Address,
//   deadline: u64,
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
let parsedAggregatorSwapParams: any;
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
    parsedAggregatorSwapParams = {
      token_in: scValToNative(aggregatorSwapParams[0]),
      token_out: scValToNative(aggregatorSwapParams[1]),
      amount_in: scValToNative(aggregatorSwapParams[2]),
      amount_out_min: scValToNative(aggregatorSwapParams[3]),
      distribution: scValToNative(aggregatorSwapParams[4]).map((distribution: any)=>{
        return {
          protocol_id: distribution.protocol_id,
          parts: distribution.parts,
          path: distribution.path.toString()
        }
      }),
      to: scValToNative(aggregatorSwapParams[5]),
      deadline: scValToNative(aggregatorSwapParams[6]),
    }
    break;
  case SwapMethod.EXACT_OUTPUT:
    aggregatorSwapParams = [
      new Address(asset_a).toScVal(),
      new Address(asset_b).toScVal(), 
      nativeToScVal(formatAmmount(max_amount), {type: "i128"}),
      nativeToScVal(formatAmmount(max_amount+200000000), {type: "i128"}),
      dexDistributionScValVec, 
      new Address(user.publicKey()).toScVal(), 
      nativeToScVal(getCurrentTimePlusOneHour(), {type:'u64'}),
    ];
    parsedAggregatorSwapParams = {
      token_in: scValToNative(aggregatorSwapParams[0]),
      token_out: scValToNative(aggregatorSwapParams[1]),
      amount_out: scValToNative(aggregatorSwapParams[2]),
      amount_in_max: scValToNative(aggregatorSwapParams[3]),
      distribution: scValToNative(aggregatorSwapParams[4]).map((distribution: any)=>{
        return {
          protocol_id: distribution.protocol_id,
          parts: distribution.parts,
          path: distribution.path.toString()
        }
      }),
      to: scValToNative(aggregatorSwapParams[5]),
      deadline: scValToNative(aggregatorSwapParams[6]),
    }
    break;
  default:
    throw new Error('Invalid swap method');
}

  console.log(`🟡 Calling aggregator ${method}`)

  console.log('🔎 aggregatorSwapParams:');
  
  console.log(parsedAggregatorSwapParams);
  const aggregatorResponse = await invokeContract(
    'aggregator',
    addressBook,
    method,
    aggregatorSwapParams,
    user
  );
  if(aggregatorResponse.status === 'SUCCESS'){
    console.log(`🟢 Aggregator ${method} successful`)
    const parsedResponse = scValToNative(aggregatorResponse.returnValue)
    return parsedResponse;
  } else {
    console.log('🔴 error calling aggregator:', aggregatorResponse)
  }
}


//Todo: refactor the main script to use preparetestenvironment (DRY!!!!)
/* const prepareTestEnvironment = async (nOfTokens: number)=>{
  const aggregatorAdmin = loadedConfig.admin;
  const networkPassphrase = loadedConfig.passphrase;
  const phoenixAdmin = loadedConfig.phoenixAdmin;
  const testUser = loadedConfig.testUser;
  const tokenAdmin = loadedConfig.tokenAdmin;
  const isTokenAdminFound = await loadedConfig.horizonRpc.loadAccount(tokenAdmin.publicKey()).catch(()=>false)
  const soroswapRouterAddress = await (await AxiosClient.get('https://api.soroswap.finance/api/testnet/router')).data.address;

  console.log('------------------------')
  console.log("----Using addresses:----")
  console.log('------------------------')
  console.log(`🔎 Test user => ${testUser.publicKey()}`);
  console.log(`🔎 Phoenix admin => ${phoenixAdmin.publicKey()}`);
  console.log(`🔎 Token admin => ${tokenAdmin.publicKey()}`);
  console.log(`🔎 Soroswap router => ${soroswapRouterAddress}`);
  
  if(!!!isTokenAdminFound){
    console.log(`🟡 Founding token admin`);
    const friendbot = await loadedConfig.horizonRpc.friendbot(tokenAdmin.publicKey());
    console.log(friendbot)
    await friendbot.call().then(()=>{
      console.log(`🟢 Token admin funded`);
    })
  }
  
  const isTestUserFound = await loadedConfig.horizonRpc.loadAccount(testUser.publicKey()).catch(()=>false)

  if(!!!isTestUserFound){
    console.log(`🟡 Founding test user`);
    const friendbot = await loadedConfig.horizonRpc.friendbot(testUser.publicKey());
    await friendbot.call().then(()=>{
      console.log(`🟢 Test user funded`);
    })
  }

  console.log("-------------------------------------------------------");
  console.log("Creating new tokens");
  console.log("-------------------------------------------------------");
  
  let assets: Asset[] = [];

  for(let i = 0; i < nOfTokens; i++){
    const asset = generateRandomAsset();
    const contractID = asset.contractId(networkPassphrase);
    console.log(`🔎 Contract ID for ${asset.code} => ${contractID}`);
    console.log('🟡Minting and deploying for test user')
    console.log('🟡Minting and deploying for phoenix admin')
    console.log("-------------------------------------------------------");
    console.log("Setting trustlines");
    console.log("-------------------------------------------------------");
    
    //intiaial balance shouldn't be hardcoded
    await deployAndMint(asset, testUser, "120000000000");
    await deployAndMint(asset, phoenixAdmin, "120000000000");
    if(i === 0){
      assets = [asset];
    } else {
      assets.push(asset);
    }
  }
  
  console.log("-------------------------------------------------------");
  console.log("Creating paths");
  console.log("-------------------------------------------------------");
  const paths = assets.map((asset)=>{
    return asset.contractId(networkPassphrase);
  });
  
  const pairs = [];
  for(let i = 0; i < paths.length; i++){
    for(let j = i+1; j < paths.length; j++){
      pairs.push([paths[i], paths[j]]);
    }
  }
  
  console.log("-------------------------------------------------------");
  console.log("Creating liquidity pools");
  console.log("-------------------------------------------------------");

  const soroswap_liquidity_pools: SoroswapPool[] = [];
  for(let i = 0; i < pairs.length; i++){
    const amount_A = 100000000000000000;
    const amount_B = 400000000000000000;
    const addresses = pairs[i];
    const LP = await createSoroswapLP(addresses, amount_A, amount_B, soroswapRouterAddress, testUser);
    console.log(LP)
    if(LP != undefined){
      soroswap_liquidity_pools.push(LP);
    }
  }


  console.log("-------------------------------------------------------");
  console.log("Creating pairs in Phoenix");
  console.log("-------------------------------------------------------");
  const assetPairs = [];
  for(let i = 0; i < assets.length; i++){
    for(let j = i+1; j < assets.length; j++){
      assetPairs.push([assets[i], assets[j]]);
    }
  }
  const phoenixPools = [];
  for(let i = 0; i < assetPairs.length; i++){
    const pairAddress: string = await create_phoenix_liquidity_pool(phoenixAdmin, aggregatorAdmin, assetPairs[i][0], assetPairs[i][1]);
    console.log('🟢 Phoenix pair address:', pairAddress);
    console.log('🟡 Adding liquidity');
    await provide_phoenix_liquidity(phoenixAdmin, pairAddress, 100000000000000000, 100000000000000000);
    const phoenixPoolBalance = await invokeCustomContract(pairAddress, 'query_pool_info', [], phoenixAdmin, true);
    const parsedPhoenixPoolBalance = scValToNative(phoenixPoolBalance.result.retval);
    phoenixPools.push({
      phoenix_pool_address: pairAddress,
      asset_a_address: parsedPhoenixPoolBalance.asset_a.address,
      asset_a_amount: parsedPhoenixPoolBalance.asset_a.amount,
      asset_b_address: parsedPhoenixPoolBalance.asset_b.address,
      asset_b_amount: parsedPhoenixPoolBalance.asset_b.amount,
      asset_lp_address: parsedPhoenixPoolBalance.asset_lp_share.address,
      asset_lp_amount: parsedPhoenixPoolBalance.asset_lp_share.amount,
      stake_address: parsedPhoenixPoolBalance.stake_address,
    });
  }

  


  
  const result = {
    assets: assets,
    contracts: paths,
    testUser: testUser,
    phoenixAdmin: phoenixAdmin,
    soroswap_liquidity_pools: soroswap_liquidity_pools,
    phoenix_pools: phoenixPools,
  }
  return result; 
} */

export {
  fetchAssetBalance,
  fetchContractBalance,
  setTrustline,
  mintToken,
  deployAndMint,
  create_soroswap_liquidity_pool,
  createSoroswapLP,
  create_phoenix_liquidity_pool,
  provide_phoenix_liquidity,
  createDexDistribution,
  callAggregatorSwap,
  generateRandomAsset,
  getPhoenixBalanceForContract
}