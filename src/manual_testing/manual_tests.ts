import { invokeContract, invokeCustomContract } from "../utils/contract.js";
import {setTrustline, payment, deployStellarAsset} from "./utils.js";
import { AddressBook } from '../utils/address_book.js';
import { config } from '../utils/env_config.js';
import { Address, Asset, nativeToScVal, scValToNative, xdr } from "@stellar/stellar-sdk";
import { AxiosClient } from "@stellar/stellar-sdk/rpc";
import { LiquidityPoolInitInfo as phoenixLPInterface } from "../protocols/phoenix/bindgins/factory_bindings.js";
import { getCurrentTimePlusOneHour } from "../utils/tx.js";

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

  console.log('-------------------------------------------------------');
  console.log('Testing Soroswap Aggregator');
  console.log('-------------------------------------------------------');

  console.log("Getting protocols")
  const {result} = await invokeContract(
    'aggregator',
    addressBook,
    'get_adapters',
    [],
    loadedConfig.admin,
    true
  );
  console.log(scValToNative(result.retval))

  //Issue #57 Create tokens
  console.log("-------------------------------------------------------");
  console.log("Creating new tokens");
  console.log("-------------------------------------------------------");
  const tokenAdmin = loadedConfig.tokenAdmin.publicKey()
  const phoenixAdmin = loadedConfig.admin.publicKey()
  const assetA = new Asset('PLT1', tokenAdmin)
  const assetB = new Asset('PLT2', tokenAdmin)
  const assetC = new Asset('PLT3', tokenAdmin)
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
      await setTrustline(asset, loadedConfig.admin, loadedConfig.horizonRpc, loadedConfig.passphrase)
      console.log(`✨Trustline for ${asset.code} set`)
      console.log(`Minting ${asset.code}`)
      await payment(loadedConfig.admin.publicKey(), asset, "15000", loadedConfig.tokenAdmin, loadedConfig.horizonRpc, loadedConfig.passphrase)
      console.log(`✨Minted $1500 ${asset.code}`)
      console.log('deploying assets')
      await deployStellarAsset(asset, loadedConfig.tokenAdmin)
    } catch(e){
      console.log(`❌Error setting trustline for ${asset.code}`)
      console.log(e)
    }
  } 
 
  //Issue #58 Add liquidity in Phoenix and Soroswap
  console.log("-------------------------------------------------------");
  console.log("Getting Factory Contract Addresses");
  console.log("-------------------------------------------------------");
  const phoenixFactoryAddress = addressBook.getContractId('phoenix_factory')
  console.log(phoenixFactoryAddress)
  const soroswapRouterAddress = await (await AxiosClient.get('https://api.soroswap.finance/api/testnet/router')).data.address
  console.log(soroswapRouterAddress)

  console.log("-------------------------------------------------------");
  console.log("Creating pairs in Phoenix");
  console.log("-------------------------------------------------------");
  const phoenixLPArgs = nativeToScVal({
    admin: phoenixAdmin,
    //fee_recipient: phoenixAdmin,
    share_token_name: 'TestLP1',
    share_token_symbol: 'TLP1',
    pool_type: 0n,
    amp: 0n,
    default_slippage_bps: 1000n,
    max_allowed_fee_bps: 1000n,
/*     max_allowed_slippage_bps: 4000n,
    max_allowed_spread_bps: 400n,
    max_referral_bps: 5000n,
    swap_fee_bps: 0n,
    stake_init_info: {
      manager: phoenixAdmin,
      max_complexity: 10,
      min_bond: 6n,
      min_reward: 3n
    }, */
    token_init_info: {
      token_a: cID_A,
      token_b: cID_B,
    },
  })
/*   const phoenixInvoke = await invokeContract('phoenix_factory', addressBook, 'create_liquidity_pool', [phoenixLPArgs], loadedConfig.admin)
  console.log('Phoenix Pair:', phoenixInvoke) */

  console.log("-------------------------------------------------------");
  console.log("Creating pairs in Soroswap");
  console.log("-------------------------------------------------------");
  console.log('To:', phoenixAdmin)
  const addLiquidityParams: xdr.ScVal[] = [
    new Address(cID_A).toScVal(),
    new Address(cID_B).toScVal(),
    nativeToScVal(1000, { type: "i128" }),
    nativeToScVal(1000, { type: "i128" }),
    nativeToScVal(0, { type: "i128" }),
    nativeToScVal(0, { type: "i128" }),
    new Address(phoenixAdmin).toScVal(),
    nativeToScVal(getCurrentTimePlusOneHour(), { type: "u64" }),
  ];
  const soroswapInvoke = await invokeCustomContract(soroswapRouterAddress, 'add_liquidity', addLiquidityParams, loadedConfig.admin)
  console.log('Soroswap Pair:', soroswapInvoke)
  //Issue #59 Get the optimal route in the aggregator

  //Issue #60 Swap tokens using the aggregator
}

aggregatorManualTest()