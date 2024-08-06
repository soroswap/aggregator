import { invokeContract, invokeCustomContract } from "../utils/contract.js";
import {setTrustline, payment, deployStellarAsset} from "./utils.js";
import { AddressBook } from '../utils/address_book.js';
import { config } from '../utils/env_config.js';
import { Address, Asset, nativeToScVal, scValToNative, xdr } from "@stellar/stellar-sdk";
import { AxiosClient } from "@stellar/stellar-sdk/rpc";
import { LiquidityPoolInitInfo as phoenixLPInterface } from "../protocols/phoenix/bindgins/factory_bindings.js";
import { getCurrentTimePlusOneHour, signWithKeypair } from "../utils/tx.js";
import * as PhoenixFactoryContract from '../protocols/phoenix/bindgins/factory_bindings.js';

const network = process.argv[2];
const addressBook = AddressBook.loadFromFile(network);
const loadedConfig = config(network);

  //Old code may not work
/*   console.log("-------------------------------------------------------");
  console.log("Starting Balances");
  console.log("-------------------------------------------------------");
  let usdcUserBalance = await invokeCustomContract(
    usdc_address,
    "balance",
    [new Address(loadedConfig.admin.publicKey()).toScVal()],
    loadedConfig.admin,
    true
  );
  console.log(
    "USDC USER BALANCE:",
    scValToNative(usdcUserBalance.result.retval)
  );
  let xtarUserBalance = await invokeCustomContract(
    xtar_address,
    "balance",
    [new Address(loadedConfig.admin.publicKey()).toScVal()],
    loadedConfig.admin,
    true
  );
  console.log("XTAR USER BALANCE:", scValToNative(xtarUserBalance.result.retval)); */

  const aggregatorManualTest = async ()=>{
  const networkPassphrase = loadedConfig.passphrase
  //Issue #57 Create tokens
  console.log("-------------------------------------------------------");
  console.log("Creating new tokens");
  console.log("-------------------------------------------------------");
  const tokenAdmin = loadedConfig.tokenAdmin
  const phoenixAdmin = loadedConfig.phoenixAdmin
  const aggregatorAdmin = loadedConfig.admin
  const testUser = loadedConfig.testUser
  const assetA = new Asset('PLT1', tokenAdmin.publicKey())
  const assetB = new Asset('PLT2', tokenAdmin.publicKey())
  const assetC = new Asset('PLT3', tokenAdmin.publicKey())
  const cID_A = assetA.contractId(networkPassphrase)
  const cID_B = assetB.contractId(networkPassphrase)
  const cID_C = assetC.contractId(networkPassphrase)
  console.log('----------------------')
  console.log("----Contract ID's----")
  console.log('----------------------')
  console.log(cID_A)
  console.log(cID_B)
  console.log(cID_C)

  console.log("-------------------------------------------------------");
  console.log("Setting trustlines");
  console.log("-------------------------------------------------------");
  const assets = [assetA, assetB, assetC]
  for(let asset of assets){
    console.log(`Setting trustline for ${asset.code}`)
    try{
      await setTrustline(asset, testUser, loadedConfig.horizonRpc, loadedConfig.passphrase)
      console.log(`Minting ${asset.code}`)
      await payment(testUser.publicKey(), asset, "150000000", loadedConfig.tokenAdmin, loadedConfig.horizonRpc, loadedConfig.passphrase)
      await deployStellarAsset(asset, loadedConfig.tokenAdmin)
      console.log(`🚀Deploying contract for ${asset.code}...`)
    } catch(e:any){
      if(e.toString().includes('ExistingValue')){
        console.log('Contract alredy deployed')
      } else {
        console.log(`❌Error setting trustline for ${asset.code}`)
        console.log(e)
      }
    }
  } 
 
  //Issue #58 Add liquidity in Phoenix and Soroswap
  const soroswapRouterAddress = await (await AxiosClient.get('https://api.soroswap.finance/api/testnet/router')).data.address
  
  console.log("-------------------------------------------------------");
  console.log("Creating pairs in Soroswap");
  console.log("-------------------------------------------------------");
  console.log('To:', testUser.publicKey())
  const addLiquidityParams: xdr.ScVal[] = [
    new Address(cID_A).toScVal(),
    new Address(cID_B).toScVal(),
    nativeToScVal(150000000n, { type: "i128" }),
    nativeToScVal(150000000n, { type: "i128" }),
    nativeToScVal(0, { type: "i128" }),
    nativeToScVal(0, { type: "i128" }),
    new Address(testUser.publicKey()).toScVal(),
    nativeToScVal(getCurrentTimePlusOneHour(), { type: "u64" }),
  ];
  const soroswapInvoke = await invokeCustomContract(soroswapRouterAddress, 'add_liquidity', addLiquidityParams, testUser)
  console.log('Soroswap Pair:', soroswapInvoke)
  console.log("-------------------------------------------------------");
  console.log("Creating pairs in Phoenix");
  console.log("-------------------------------------------------------");
  const factory_contract = new PhoenixFactoryContract.Client({
    publicKey: phoenixAdmin.publicKey()!,
    contractId: addressBook.getContractId("phoenix_factory"),
    networkPassphrase: loadedConfig.passphrase,
    rpcUrl: "https://soroban-testnet.stellar.org/",
    signTransaction: (tx: string) => signWithKeypair(tx, loadedConfig.passphrase, phoenixAdmin),
  });

  const tx = await factory_contract.create_liquidity_pool({
    sender: phoenixAdmin.publicKey(),
    lp_init_info: {
      admin: aggregatorAdmin.publicKey(),
      fee_recipient: testUser.publicKey(),
      max_allowed_slippage_bps: 4000n,
      max_allowed_spread_bps: 400n,
      max_referral_bps: 5000n,
      swap_fee_bps: 0n,
      stake_init_info: {
        manager: testUser.publicKey(),
        max_complexity: 10,
        min_bond: 6n,
        min_reward: 3n
      },
      token_init_info: {
        token_b: cID_A,
        token_a: cID_B,
      }
    },
    share_token_name: `TOKEN-LP-${assetA.code}/${assetB.code}`,
    share_token_symbol: `PLP-${assetA.code}/${assetB.code}`,
  });
  try {
    const result = await tx.signAndSend();
    console.log('🚀 « result:', result);
  } catch (error) {
    console.log('🚀 « error:', error);
  }

  console.log('-------------------------------------------------------');
  console.log('Testing Soroswap Aggregator');
  console.log('-------------------------------------------------------');

  const soroswapAdapter =  addressBook.getContractId('soroswap_adapter');
  console.log('soroswapAdapter:', soroswapAdapter)
  const phoenixAdapter =  addressBook.getContractId('phoenix_adapter');
  console.log('phoenixAdapter:', phoenixAdapter)

  const dexDistributionRaw = [
    {
      protocol_id: "soroswap",
      path: [cID_A, cID_B],
      parts: 50,
      is_exact_in: true,
    },
    {
      protocol_id: "phoenix",
      path: [cID_A, cID_B],
      parts: 50,
      is_exact_in: true,
    },
  ];

//  pub struct DexDistribution {
//    pub protocol_id: String,
//    pub path: Vec<Address>,
//    pub parts: u32,
//  }

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

//  fn swap_exact_tokens_for_tokens(
//    token_in: Address,
//    token_out: Address,
//    amount_in: i128,
//    amount_out_min: i128,
//    distribution: Vec<DexDistribution>,
//    to: Address,
//    deadline: u64,
//) 
  const aggregatorSwapParams: xdr.ScVal[] = [
    new Address(cID_A).toScVal(),
    new Address(cID_B).toScVal(), 
    nativeToScVal(10000000n, {type: "i128"}),
    nativeToScVal(0n, {type: "i128"}),
    dexDistributionScValVec, 
    new Address(loadedConfig.testUser.publicKey()).toScVal(), 
    nativeToScVal(getCurrentTimePlusOneHour(), {type:'u64'}),
  ];

  console.log("Initializing Aggregator")
  await invokeContract(
    'aggregator',
    addressBook,
    'swap_exact_tokens_for_tokens',
    aggregatorSwapParams,
    loadedConfig.admin
  );
  //Issue #59 Get the optimal route in the aggregator
  
  //Issue #60 Swap tokens using the aggregator
}

aggregatorManualTest()