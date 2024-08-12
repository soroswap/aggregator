import { invokeCustomContract } from "../utils/contract.js";
import { 
  setTrustline,
  mintToken,
  deployStellarAsset,
  create_soroswap_liquidity_pool,
  create_phoenix_liquidity_pool,
  provide_phoenix_liquidity,
  fetchAssetBalance,
  fetchContractBalance,
  createDexDistribution,
  callAggregatorSwap,
  SwapMethod
} from "./utils.js";
import { AddressBook } from '../utils/address_book.js';
import { config } from '../utils/env_config.js';
import { Address, Asset, scValToNative, xdr } from "@stellar/stellar-sdk";
import { AxiosClient } from "@stellar/stellar-sdk/rpc";


const network = process.argv[2];
const addressBook = AddressBook.loadFromFile(network);
const loadedConfig = config(network);


const aggregatorManualTest = async ()=>{
  const networkPassphrase = loadedConfig.passphrase;

  console.log("-------------------------------------------------------");
  console.log("Creating new tokens");
  console.log("-------------------------------------------------------");
  const tokenAdmin = loadedConfig.tokenAdmin;
  const phoenixAdmin = loadedConfig.phoenixAdmin;
  const aggregatorAdmin = loadedConfig.admin;
  const testUser = loadedConfig.testUser;
  const assetA = new Asset('AAAA', tokenAdmin.publicKey());
  const assetB = new Asset('AAAB', tokenAdmin.publicKey());
  const assetC = new Asset('AABB', tokenAdmin.publicKey());
  const cID_A = assetA.contractId(networkPassphrase);
  const cID_B = assetB.contractId(networkPassphrase);
  const cID_C = assetC.contractId(networkPassphrase);
  console.log('------------------------')
  console.log("----Using addresses:----")
  console.log('------------------------')
  console.log('🔎 Contract id for AAAB: ',cID_B);
  console.log('🔎 Contract id for AAAA: ',cID_A);
  console.log('🔎 Contract id for AABB: ',cID_C);
  
  console.log(`🔎 Test user: ${testUser.publicKey()}`);
  console.log(`🔎 Phoenix admin: ${phoenixAdmin.publicKey()}`);
  console.log(`🔎 Token admin: ${tokenAdmin.publicKey()}`);
  console.log(`🔎 Aggregator admin: ${aggregatorAdmin.publicKey()}`);
  console.log("-------------------------------------------------------");
  console.log("Setting trustlines");
  console.log("-------------------------------------------------------");
  
  const assets = [assetA, assetB, assetC] ;
  
  for(let asset of assets){
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

    const userHasTrustline = await fetchAssetBalance(asset, testUser);
    if(!userHasTrustline){
      console.log(`Missing trustline for ${asset.code} in ${testUser.publicKey()}`);
      try{
        await setTrustline(asset, testUser, loadedConfig.horizonRpc);
      } catch(e:any){
        console.error(e);
      }  
    } else {
      console.log(`🟢 Trustline for ${asset.code} already exists in ${testUser.publicKey()}`);;
      console.log(`🟢 Balance: ${userHasTrustline}`);;
    }
    const phoenixAdminHasTrustline = await fetchAssetBalance(asset, phoenixAdmin);;
    if(!phoenixAdminHasTrustline){
      console.log(`Missing trustline for ${asset.code} in ${phoenixAdmin.publicKey()}`);;
      try{
        await setTrustline(asset, phoenixAdmin, loadedConfig.horizonRpc);;
      } catch(e:any){
        console.error(e);;
      }  
    } else {
      console.log(`🟢 Trustline for ${asset.code} already exists in ${phoenixAdmin.publicKey()}`);;
      console.log(`🟢 Balance: ${phoenixAdminHasTrustline}`);;
    }

    await mintToken(testUser.publicKey(), asset, "1500000000", tokenAdmin, loadedConfig.horizonRpc, loadedConfig.passphrase);;
    const newUserBalance = await fetchAssetBalance(asset, testUser);;
    console.log(`🟢 Test user balance of ${asset.code}: ${newUserBalance}`);;


    await mintToken(phoenixAdmin.publicKey(), asset, "150000000000", tokenAdmin, loadedConfig.horizonRpc, loadedConfig.passphrase);;
    const newPhoenixBalance = await fetchAssetBalance(asset, phoenixAdmin);;
    console.log(`🟢 Phoenix balance of ${asset.code}: ${newPhoenixBalance}`);;
    
  }
  //Issue #58 Add liquidity in Phoenix and Soroswap
  const soroswapRouterAddress = await (await AxiosClient.get('https://api.soroswap.finance/api/testnet/router')).data.address;;
 
  //To-do: Add liquidity to all pools
  console.log("-------------------------------------------------------");
  console.log("Creating Soroswap liquidity pool");
  console.log("-------------------------------------------------------");
  const poolParams = {
    contractID_A: cID_A,
    contractID_B: cID_B,
    user: testUser,
    amount_A: 15000000,
    amount_B: 15000000,
  };;

  await create_soroswap_liquidity_pool(soroswapRouterAddress, poolParams);;

  const fetchPoolParams: xdr.ScVal[] = [
    new Address(cID_A).toScVal(),
    new Address(cID_B).toScVal(),
  ];;

  console.log('🟡 Fetching Soroswap pair address');;
  const soroswapPool = await invokeCustomContract(soroswapRouterAddress, 'router_pair_for', fetchPoolParams, testUser, true);;
  const soroswapPoolCID = scValToNative(soroswapPool.result.retval);;
  console.log('🟢 Soroswap pair address:', soroswapPoolCID);;

  console.log('🟡 Fetching liquidity pool balance');;
  const soroswapPoolBalance = await fetchContractBalance(soroswapPoolCID, testUser);;
  console.log(`🟢 Soroswap pair balance: ${(soroswapPoolBalance)}`);;

  //To-do: Add liquidity to all pools
  console.log("-------------------------------------------------------");
  console.log("Creating pairs in Phoenix");
  console.log("-------------------------------------------------------");
  
  const pairAddress: string = await create_phoenix_liquidity_pool(phoenixAdmin, aggregatorAdmin, testUser, assetA, assetB);
  console.log('🟢 Phoenix pair address:', pairAddress);

  const initialPhoenixPoolBalance = await invokeCustomContract(pairAddress, 'query_pool_info', [], phoenixAdmin, true);
  console.log('🔎 Current Phoenix liquidity pool balances:',scValToNative(initialPhoenixPoolBalance.result.retval));
  
  console.log('🟡 Adding liquidity');
  await provide_phoenix_liquidity(phoenixAdmin, pairAddress, 100000000000, 100000000000);
  const phoenixPoolBalance = await invokeCustomContract(pairAddress, 'query_pool_info', [], phoenixAdmin, true);
  console.log('🔎 New Phoenix liquidity pool balances:',scValToNative(phoenixPoolBalance.result.retval));
  
  //To-do: refactor agregator swap, add swapMethod (exact-tokens/tokens-exact)
  console.log('-------------------------------------------------------');
  console.log('Testing Soroswap Aggregator');
  console.log('-------------------------------------------------------');

  const soroswapAdapter =  addressBook.getContractId('soroswap_adapter');
  console.log('soroswapAdapter:', soroswapAdapter);
  const phoenixAdapter =  addressBook.getContractId('phoenix_adapter');
  console.log('phoenixAdapter:', phoenixAdapter);

  const dexDistributionRaw = [
    {
      protocol_id: "soroswap",
      path: [cID_A, cID_B],
      parts: 50,
    },
    {
      protocol_id: "phoenix",
      path: [cID_A, cID_B],
      parts: 50,
    },
  ];

  const dexDistributionVec = await createDexDistribution(dexDistributionRaw);

  const asset_A_first_balance = await fetchAssetBalance(assetA, testUser);
  const asset_B_first_balance = await fetchAssetBalance(assetB, testUser);

  console.log('🟡 Initial balances');
  console.log('🔎 Asset A:', asset_A_first_balance);
  console.log('🔎 Asset B:', asset_B_first_balance);
  
  const swapExactIn = await callAggregatorSwap(cID_A, cID_B, 150000, dexDistributionVec, testUser, SwapMethod.EXACT_INPUT);
  console.log('🟡 Swap exact in:', swapExactIn);

  const asset_A_second_balance = await fetchAssetBalance(assetA, testUser);
  const asset_B_second_balance = await fetchAssetBalance(assetB, testUser);

  console.log('🟡 Test user balances after exact input swap');
  console.log('🔎 Asset A:', asset_A_second_balance);
  console.log('🔎 Asset B:', asset_B_second_balance);

  const swapExactOut = await callAggregatorSwap(cID_A, cID_B, 150000, dexDistributionVec, testUser, SwapMethod.EXACT_OUTPUT);
  console.log('🟡 Swap exact out:', swapExactOut);

  const asset_A_third_balance = await fetchAssetBalance(assetA, testUser);
  const asset_B_third_balance = await fetchAssetBalance(assetB, testUser);
  
  console.log('🟡 Test user balances after exact output swap');
  console.log('🔎 Asset A:', asset_A_third_balance);
  console.log('🔎 Asset B:', asset_B_third_balance);
}

aggregatorManualTest();