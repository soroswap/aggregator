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

const setTrustline = async (asset: Asset, account: Keypair, rpc: Horizon.Server, limit?: string,) => {
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

  const keyPair = account;
  await transaction.sign(keyPair);
  const transactionResult = await rpc.submitTransaction(transaction);
  if(transactionResult.successful) {
    console.log(`✨Trustline for ${asset.code} set`)
  }
  return transactionResult;
}

const payment = async (destination: string, asset: Asset, amount: string, source: Keypair, rpc: Horizon.Server, passphrase: string,) => {
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

interface CreatePoolParams {
  contractID_A: string,
  contractID_B: string,
  user: Keypair,
  amount_A: number,
  amount_B: number,
}
const formatAmmount = (amount: number) => {
  const formattedAmmount = BigInt(amount * 1000000).toString();
  return formattedAmmount
}

const create_soroswap_liquidity_pool = async (
    router:string,  
    poolParams: CreatePoolParams,
  ) => {
  console.log('🟡 Creating soroswap liquidity pool')
  console.log('🚀 « poolParams:', poolParams);  
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
  testUser: Keypair, 
  aggregatorAdmin:Keypair, 
  assetA:Asset, 
  assetB:Asset)=>{
  const tx = await factory_contract.create_liquidity_pool({
    sender: phoenixAdmin.publicKey(),
    lp_init_info: {
      admin: phoenixAdmin.publicKey(),
      fee_recipient: testUser.publicKey(),
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
        token_a: assetA.contractId(loadedConfig.passphrase),
        token_b: assetB.contractId(loadedConfig.passphrase),
      }
    },
    share_token_name: `TOKEN-LP-${assetA.code}/${assetB.code}`,
    share_token_symbol: `PLP-${assetA.code}/${assetB.code}`,
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
  let shouldRetry = false;
  try {
    console.log('creating phoenix pool transaction')
    await create_phoenix_pool_transaction(factory_contract, phoenixAdmin, testUser, aggregatorAdmin, assetA, assetB)
  } catch (error:any) {
    if(error.toString().includes('#5')){
      console.log('Asset A greater than asset B, should retry')
      shouldRetry = true;  
    } else if(error.toString().includes('ExistingValue')){
      console.log('Pool already exists')
      console.log('🚀 « error:', error)
    }
  }
  if(shouldRetry){
    console.log('Retrying to create phoenix pool')
    try {
      await create_phoenix_pool_transaction(factory_contract, phoenixAdmin, testUser, aggregatorAdmin, assetB, assetA)
    }
     catch(error:any) {
      if(error.toString().includes('ExistingValue')){
        console.log('Pool already exists, continuing')
      } else {
        console.log('🚀 « error:', error)
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
    nativeToScVal(null)
  ]

  const provide_liquidity = await invokeCustomContract(pairAddress, 'provide_liquidity', addPhoenixLiquidityParams, phoenixAdmin)

  if(provide_liquidity.status === 'SUCCESS'){
    console.log('✨🟢 Successfully added liquidity to phoenix pool')
    return provide_liquidity;
  } else {
    console.log('🔴 error providing liquidity:', provide_liquidity)
    return provide_liquidity;
  }
}

export {
  setTrustline,
  payment, 
  deployStellarAsset,
  create_soroswap_liquidity_pool,
  create_phoenix_liquidity_pool,
  provide_phoenix_liquidity
}