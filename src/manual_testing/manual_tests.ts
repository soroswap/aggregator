import { invokeContract, invokeCustomContract } from "../utils/contract.js";
import {setTrustline, payment, deployStellarAsset, create_soroswap_liquidity_pool, create_phoenix_liquidity_pool, provide_phoenix_liquidity} from "./utils.js";
import { AddressBook } from '../utils/address_book.js';
import { config } from '../utils/env_config.js';
import { Address, Asset, nativeToScVal, scValToNative, xdr } from "@stellar/stellar-sdk";
import { AxiosClient } from "@stellar/stellar-sdk/rpc";

import { getCurrentTimePlusOneHour, signWithKeypair } from "../utils/tx.js";


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
  //To-do: Clear console.logs
  const networkPassphrase = loadedConfig.passphrase

  //Issue #57 Create tokens
  console.log("-------------------------------------------------------");
  console.log("Creating new tokens");
  console.log("-------------------------------------------------------");
  const tokenAdmin = loadedConfig.tokenAdmin
  const phoenixAdmin = loadedConfig.phoenixAdmin
  const aggregatorAdmin = loadedConfig.admin
  const testUser = loadedConfig.testUser
  const assetA = new Asset('AAAA', tokenAdmin.publicKey())
  const assetB = new Asset('AAAB', tokenAdmin.publicKey())
  const assetC = new Asset('AABB', tokenAdmin.publicKey())
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
    try {
      console.log(`🟡 Deploying contract for ${asset.code}`)
      await deployStellarAsset(asset, loadedConfig.tokenAdmin)

    } catch (error:any) {
      if(error.toString().includes('ExistingValue')){
        console.log(`🟢 Contract for ${asset.code} already exists`)
      } else {
        console.error(error)
      }
    }

    try {
      console.log('🟡 Checking trustline for user')
      const userHasTrustline  = await invokeCustomContract(
        asset.contractId(networkPassphrase),
        "balance",
        [new Address(testUser.publicKey()).toScVal()],
        testUser,
        true
      );
      if(!!userHasTrustline.result.retval.value()){
        console.log(`🟢 Trustline for ${asset.code} already exists`)
      }
    } catch (error:any) {
      console.log(`🆙 Trustline for ${asset.code} not set`)
      if(error.toString().includes('#13')){
        try{
          console.log('Setting trustline...')
          await setTrustline(asset, testUser, loadedConfig.horizonRpc)
        } catch(e:any){
          console.error(e)
        }      
      }
    }

    try {
      console.log('🟡 Checking trustline for phoenix')
      const phoenixHasTrustline  =  await invokeCustomContract(
        asset.contractId(networkPassphrase),
        "balance",
        [new Address(phoenixAdmin.publicKey()).toScVal()],
        phoenixAdmin,
        true
      )
      if(!!phoenixHasTrustline.result.retval.value()){
        console.log(`🟢 Trustline for ${asset.code} already exists`)
      }
    } catch (error:any){
      console.log(`🆙 Trustline for ${asset.code} not set`)
      if(error.toString().includes('#13')){
        try{
          console.log('Setting trustline...')
          await setTrustline(asset, phoenixAdmin, loadedConfig.horizonRpc)
        } catch(e:any){
          console.error(e)
        }
      }
    }
    
      
    
    console.log(`Minting ${asset.code}`)
    await payment(testUser.publicKey(), asset, "1500000000", tokenAdmin, loadedConfig.horizonRpc, loadedConfig.passphrase)
    await payment(phoenixAdmin.publicKey(), asset, "1500000000", tokenAdmin, loadedConfig.horizonRpc, loadedConfig.passphrase)
    
  }
  //Issue #58 Add liquidity in Phoenix and Soroswap
  const soroswapRouterAddress = await (await AxiosClient.get('https://api.soroswap.finance/api/testnet/router')).data.address
 
  //To-do: Add liquidity to all pools
  console.log("-------------------------------------------------------");
  console.log("Creating Soroswap liquidity pool");
  console.log("-------------------------------------------------------");
  const poolParams = {
    contractID_A: cID_A,
    contractID_B: cID_B,
    user: testUser,
    amount_A: 1500,
    amount_B: 1500,
  }

  await create_soroswap_liquidity_pool(soroswapRouterAddress, poolParams)

  //To-do: Add liquidity to all pools
  console.log("-------------------------------------------------------");
  console.log("Creating pairs in Phoenix");
  console.log("-------------------------------------------------------");

  const pairAddress: any = await create_phoenix_liquidity_pool(phoenixAdmin, aggregatorAdmin, testUser, assetA, assetB)
  console.log('🟢 Phoenix pair address:', pairAddress)

  console.log('🟡 Adding liquidity')
  
  await provide_phoenix_liquidity(phoenixAdmin, pairAddress, 1500, 1500)

  
  //To-do: refactor agregator swap, add swapMethod (exact-tokens/tokens-exact)
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
    nativeToScVal(15000000000n, {type: "i128"}),
    nativeToScVal(0n, {type: "i128"}),
    dexDistributionScValVec, 
    new Address(testUser.publicKey()).toScVal(), 
    nativeToScVal(getCurrentTimePlusOneHour(), {type:'u64'}),
  ];

  console.log("Initializing Aggregator")
  const aggregatorResponse = await invokeContract(
    'aggregator',
    addressBook,
    'swap_exact_tokens_for_tokens',
    aggregatorSwapParams,
    testUser
  );

  //To-do: parse response
  console.log(aggregatorResponse.status)
  console.log(scValToNative(aggregatorResponse.returnValue))
  

}

aggregatorManualTest()