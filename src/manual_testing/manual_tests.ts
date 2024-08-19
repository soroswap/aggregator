import { invokeCustomContract } from "../utils/contract.js";
import { 
  generateRandomAsset,
  deployAndMint,
  createSoroswapLP,
  create_phoenix_liquidity_pool,
  provide_phoenix_liquidity,
  fetchAssetBalance,
  createDexDistribution,
  callAggregatorSwap,
  SwapMethod,
  getPhoenixBalanceForContract,
  SoroswapPool,
  create_soroswap_liquidity_pool
} from "./utils.js";
import { AddressBook } from '../utils/address_book.js';
import { config } from '../utils/env_config.js';
import { Address, Asset, scValToNative, xdr } from "@stellar/stellar-sdk";
import { AxiosClient } from "@stellar/stellar-sdk/rpc";

const network = process.argv[2];
const addressBook = AddressBook.loadFromFile(network);
const loadedConfig = config(network);

const prepareTestEnvironment = async ()=>{
  const networkPassphrase = loadedConfig.passphrase;
  const soroswapRouterAddress = await (await AxiosClient.get('https://api.soroswap.finance/api/testnet/router')).data.address;
  console.log("-------------------------------------------------------");
  console.log("Creating new tokens");
  console.log("-------------------------------------------------------");
  const tokenAdmin = loadedConfig.tokenAdmin;
  const phoenixAdmin = loadedConfig.phoenixAdmin;
  const aggregatorAdmin = loadedConfig.admin;
  const testUser = loadedConfig.testUser;
  
  const assetA = generateRandomAsset();
  const assetB = generateRandomAsset();

  const cID_A = assetA.contractId(networkPassphrase);
  const cID_B = assetB.contractId(networkPassphrase);

  console.log('------------------------')
  console.log("----Using addresses:----")
  console.log('------------------------')
  console.log(`🔎 Contract ID for ${assetA.code} => ${cID_A}`)
  console.log(`🔎 Contract ID for ${assetB.code} => ${cID_B}`)
  
  console.log(`🔎 Test user => ${testUser.publicKey()}`);
  console.log(`🔎 Phoenix admin => ${phoenixAdmin.publicKey()}`);
  console.log(`🔎 Token admin => ${tokenAdmin.publicKey()}`);
  console.log("-------------------------------------------------------");
  console.log("Setting trustlines");
  console.log("-------------------------------------------------------");
  
  const assets = [assetA, assetB];

  const isTokenAdminFound = await loadedConfig.horizonRpc.loadAccount(tokenAdmin.publicKey()).catch(()=>false)

  if(!!!isTokenAdminFound){
    console.log(`🟡 Founding token admin`);
    const friendbot = await loadedConfig.horizonRpc.friendbot(tokenAdmin.publicKey());
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
  for(let asset of assets){
    console.log(asset)
    await deployAndMint(asset, testUser, "40000000000");
    await deployAndMint(asset, phoenixAdmin, "40000000000");
  }

  console.log("-------------------------------------------------------");
  console.log("Creating Soroswap liquidity pool");
  console.log("-------------------------------------------------------");
  const poolParams = {
    contractID_A: cID_A,
    contractID_B: cID_B,
    user: testUser,
    amount_A: 100000000000000000,
    amount_B: 400000000000000000,
  };

  await create_soroswap_liquidity_pool(soroswapRouterAddress, poolParams);

  const fetchPoolSecondParams: xdr.ScVal[] = [
    new Address(cID_A).toScVal(),
    new Address(cID_B).toScVal(),
  ];

  console.log('🟡 Fetching Soroswap pair address');
  const soroswapPool = await invokeCustomContract(soroswapRouterAddress, 'router_pair_for', fetchPoolSecondParams, testUser, true);
  const soroswapPoolCID = scValToNative(soroswapPool.result.retval);
  console.log('🟢 Soroswap pair address:', soroswapPoolCID)

  console.log('🟡 Fetching liquidity pool balance');
  const soroswapPoolBalance = await invokeCustomContract(soroswapPoolCID, 'get_reserves', [], testUser, true);
  console.log(scValToNative(soroswapPoolBalance.result.retval));
  console.log(`🟢 Soroswap pair balance: ${scValToNative(soroswapPoolBalance.result.retval)}`);

  //To-do: Add liquidity to all pools
  console.log("-------------------------------------------------------");
  console.log("Creating pairs in Phoenix");
  console.log("-------------------------------------------------------");
  
  const pairAddress: string = await create_phoenix_liquidity_pool(phoenixAdmin, aggregatorAdmin, assetA, assetB);
  console.log('🟢 Phoenix pair address:', pairAddress);

  const initialPhoenixPoolBalance = await invokeCustomContract(pairAddress, 'query_pool_info', [], phoenixAdmin, true);
  console.log('🔎 Current Phoenix liquidity pool balances:',scValToNative(initialPhoenixPoolBalance.result.retval));
  
  console.log('🟡 Adding liquidity');
  await provide_phoenix_liquidity(phoenixAdmin, pairAddress, 100000000000000000, 100000000000000000);
  const phoenixPoolBalance = await invokeCustomContract(pairAddress, 'query_pool_info', [], phoenixAdmin, true);
  console.log('🔎 New Phoenix liquidity pool balances:',scValToNative(phoenixPoolBalance.result.retval));
  const result = {
    assetA: assetA,
    assetB: assetB,
    cID_A: cID_A,
    cID_B: cID_B,
    testUser: testUser,
    phoenixAdmin: phoenixAdmin,
    soroswapPoolCID: soroswapPoolCID,
    pairAddress: pairAddress,
    soroswapPoolBalance: soroswapPoolBalance,
    phoenixPoolBalance: phoenixPoolBalance,
  }
  return result;
}

const swapExactInputAggregatorTest = async ()=>{
  console.log("-------------------------------------------------------");
  console.log("Testing exact input swap");
  console.log("-------------------------------------------------------");
  const { 
    assetA,
    assetB, 
    cID_A, 
    cID_B, 
    testUser, 
    phoenixAdmin, 
    soroswapPoolCID, 
    pairAddress, 
    soroswapPoolBalance, 
    phoenixPoolBalance 
  } = await prepareTestEnvironment();

  console.log('-------------------------------------------------------');
  console.log('Aggregator exact input swap test');
  console.log('-------------------------------------------------------');

  const soroswapAdapter =  addressBook.getContractId('soroswap_adapter');
  console.log('soroswapAdapter:', soroswapAdapter);
  const phoenixAdapter =  addressBook.getContractId('phoenix_adapter');
  console.log('phoenixAdapter:', phoenixAdapter);

  const dexDistributionRaw = [
    {
      protocol_id: "soroswap",
      path: [cID_A, cID_B],
      parts: 1,
    },
    {
      protocol_id: "phoenix",
      path: [cID_A, cID_B],
      parts: 3,
    },
  ];

  const dexDistributionVec = await createDexDistribution(dexDistributionRaw);

  const asset_A_first_balance = await fetchAssetBalance(assetA, testUser);
  const asset_B_first_balance = await fetchAssetBalance(assetB, testUser);

  console.log(' ------------ Test user balances --------------');
  console.log('🔎 Asset A:', asset_A_first_balance);
  console.log('🔎 Asset B:', asset_B_first_balance);

  console.log(' ------------ Soroswap pool balances --------------');
  console.log('🔎 Soroswap pool balance [A,B]:', scValToNative(soroswapPoolBalance.result.retval));

  console.log(' ------------ Phoenix pool balances --------------')
  console.log('🔎 Phoenix pool balance:', scValToNative(phoenixPoolBalance.result.retval));
  
  const swapExactIn = await callAggregatorSwap(cID_A, cID_B, 123456789, dexDistributionVec, testUser, SwapMethod.EXACT_INPUT);
  console.log('🟡 Swap exact in:', swapExactIn);

  const asset_A_second_balance = await fetchAssetBalance(assetA, testUser);
  const asset_B_second_balance = await fetchAssetBalance(assetB, testUser);

  console.log(' -------------- Test user balances after exact input swap -------------');
  console.log('🔎 Asset A:', asset_A_second_balance);
  console.log('🔎 Asset B:', asset_B_second_balance);

  console.log(' -------------- Soroswap pool balances after exact input swap -------------');
  const soroswapPoolBalanceAfterExactIn = await invokeCustomContract(soroswapPoolCID, 'get_reserves', [], testUser, true);
  console.log('🔎 Soroswap pool balance [A,B]:', scValToNative(soroswapPoolBalanceAfterExactIn.result.retval))
  
  console.log(' -------------- Phoenix pool balances after exact input swap -------------');
  const phoenixPoolBalanceAfterExactIn = await invokeCustomContract(pairAddress, 'query_pool_info', [], phoenixAdmin, true);
  console.log('🔎 Phoenix pool balance:', scValToNative(phoenixPoolBalanceAfterExactIn.result.retval));

  const phoenix_before_assets = scValToNative(phoenixPoolBalance.result.retval);
  const phoenix_after_assets = scValToNative(phoenixPoolBalanceAfterExactIn.result.retval);

  console.log("-------------- Contract ID's -----------------")
  console.table({
    'Contract Asset A': cID_A,
    'Contract Asset B': cID_B,
    'Contract Soroswap': soroswapPoolCID,
    'Contract Phoenix': pairAddress,
  })

  console.log(' -------------- Asset balances table -------------')
  console.table({
    'Initial balance': {
      'User Asset A': asset_A_first_balance,
      'User Asset B': asset_B_first_balance,
      'Soroswap Asset A': scValToNative(soroswapPoolBalance.result.retval)[0],
      'Soroswap Asset B': scValToNative(soroswapPoolBalance.result.retval)[1],
      'Phoenix Asset A': getPhoenixBalanceForContract(cID_A, phoenix_before_assets),
      'Phoenix Asset B': getPhoenixBalanceForContract(cID_B, phoenix_before_assets),
    },
    'Balance after exact input swap': {
      'User Asset A': asset_A_second_balance,
      'User Asset B': asset_B_second_balance,
      'Soroswap Asset A': scValToNative(soroswapPoolBalanceAfterExactIn.result.retval)[0],
      'Soroswap Asset B': scValToNative(soroswapPoolBalanceAfterExactIn.result.retval)[1],
      'Phoenix Asset A': getPhoenixBalanceForContract(cID_A, phoenix_after_assets),
      'Phoenix Asset B': getPhoenixBalanceForContract(cID_B, phoenix_after_assets),
    },
  })
  const expectedAmountIn0 = 30864197n;
  const expectedAmountIn1 = 92592592n;
  const expectedAmountOut0 = 123086415n;
  const expectedAmountOut1 = 92592592n;
  if(
    swapExactIn[0][0] === expectedAmountIn0 && 
    swapExactIn[0][1] === expectedAmountOut0 &&
    swapExactIn[1][0] === expectedAmountIn1 &&
    swapExactIn[1][1] === expectedAmountOut1
  ){
    console.log('🟢 Aggregator test swap exact input passed')
    return true;
  } else {
    console.error('🔴 Aggregator test swap exact input failed')
    return false;
  }
}

const swapExactOutputAggregatorTest = async ()=>{
  console.log("-------------------------------------------------------");
  console.log("Testing exact output swap");
  console.log("-------------------------------------------------------");
  const { 
    assetA,
    assetB, 
    cID_A, 
    cID_B, 
    testUser, 
    phoenixAdmin, 
    soroswapPoolCID, 
    pairAddress, 
    soroswapPoolBalance, 
    phoenixPoolBalance 
  } = await prepareTestEnvironment();

  console.log('-------------------------------------------------------');
  console.log('Aggregator exact output test');
  console.log('-------------------------------------------------------');

  const soroswapAdapter =  addressBook.getContractId('soroswap_adapter');
  console.log('soroswapAdapter:', soroswapAdapter);
  const phoenixAdapter =  addressBook.getContractId('phoenix_adapter');
  console.log('phoenixAdapter:', phoenixAdapter);

  const dexDistributionRaw = [
    {
      protocol_id: "soroswap",
      path: [cID_A, cID_B],
      parts: 1,
    },
    {
      protocol_id: "phoenix",
      path: [cID_A, cID_B],
      parts: 3,
    },
  ];

  const dexDistributionVec = await createDexDistribution(dexDistributionRaw);

  const asset_A_first_balance = await fetchAssetBalance(assetA, testUser);
  const asset_B_first_balance = await fetchAssetBalance(assetB, testUser);

  console.log(' ------------ Test user balances --------------');
  console.log('🔎 Asset A:', asset_A_first_balance);
  console.log('🔎 Asset B:', asset_B_first_balance);

  console.log(' ------------ Soroswap pool balances --------------');
  console.log('🔎 Soroswap pool balance [A,B]:', scValToNative(soroswapPoolBalance.result.retval));

  console.log(' ------------ Phoenix pool balances --------------')
  console.log('🔎 Phoenix pool balance:', scValToNative(phoenixPoolBalance.result.retval));
  
  const swapExactOut = await callAggregatorSwap(cID_A, cID_B, 30000000, dexDistributionVec, testUser, SwapMethod.EXACT_OUTPUT);
  console.log('🟡 Swap exact out:', swapExactOut);

  const asset_A_second_balance = await fetchAssetBalance(assetA, testUser);
  const asset_B_second_balance = await fetchAssetBalance(assetB, testUser);
  
  console.log(' -------------- Test user balances after exact output swap -------------');
  console.log('🔎 Asset A:', asset_A_second_balance);
  console.log('🔎 Asset B:', asset_B_second_balance);

  console.log(' -------------- Soroswap pool balances after exact output swap -------------');
  const soroswapPoolBalanceAfter = await invokeCustomContract(soroswapPoolCID, 'get_reserves', [], testUser, true);
  console.log('🔎 Soroswap pool balance [A,B]:', scValToNative(soroswapPoolBalanceAfter.result.retval));

  console.log(' -------------- Phoenix pool balances after exact output swap -------------');
  const phoenixPoolBalanceAfter = await invokeCustomContract(pairAddress, 'query_pool_info', [], phoenixAdmin, true);
  console.log('🔎 Phoenix pool balance:', scValToNative(phoenixPoolBalanceAfter.result.retval));

  const phoenix_before_assets = scValToNative(phoenixPoolBalance.result.retval);
  const phoenix_after_assets = scValToNative(phoenixPoolBalanceAfter.result.retval);

  console.log("-------------- Contract ID's -----------------")
  console.table({
    'Contract Asset A': cID_A,
    'Contract Asset B': cID_B,
    'Contract Soroswap': soroswapPoolCID,
    'Contract Phoenix': pairAddress,
  })
  const expectedAmountIn0 = 1880643n;
  const expectedAmountIn1 = 22500000n;
  const expectedAmountOut0 = 7500000n;
  const expectedAmountOut1 = 22500000n;

  console.log(' -------------- Asset balances table -------------')
  console.table({
    'Initial balance': {
      'User Asset A': asset_A_first_balance,
      'User Asset B': asset_B_first_balance,
      'Soroswap Asset A': scValToNative(soroswapPoolBalance.result.retval)[0],
      'Soroswap Asset B': scValToNative(soroswapPoolBalance.result.retval)[1],
      'Phoenix Asset A': getPhoenixBalanceForContract(cID_A, phoenix_before_assets),
      'Phoenix Asset B': getPhoenixBalanceForContract(cID_B, phoenix_before_assets),
    },
    'Balance after exact output swap': {
      'User Asset A': asset_A_second_balance,
      'User Asset B': asset_B_second_balance,
      'Soroswap Asset A': scValToNative(soroswapPoolBalanceAfter.result.retval)[0],
      'Soroswap Asset B': scValToNative(soroswapPoolBalanceAfter.result.retval)[1],
      'Phoenix Asset A': getPhoenixBalanceForContract(cID_A, phoenix_after_assets),
      'Phoenix Asset B': getPhoenixBalanceForContract(cID_B, phoenix_after_assets),
    },
    'Expected amounts': {
      'Expected amount in asset A': expectedAmountIn0,
      'Expected amount out asset A': expectedAmountOut0,
      'Expected amount in asset B': expectedAmountIn1,
      'Expected amount out asset B': expectedAmountOut1,
    },
    'Swap result': {
      'Amount in asset A': swapExactOut[0][0],
      'Amount out asset A': swapExactOut[0][1],
      'Amount in asset B': swapExactOut[1][0],
      'Amount out asset B': swapExactOut[1][1],
    }
  })

  if(
    swapExactOut[0][0] === expectedAmountIn0 && 
    swapExactOut[0][1] === expectedAmountOut0 &&
    swapExactOut[1][0] === expectedAmountIn1 &&
    swapExactOut[1][1] === expectedAmountOut1
  ){
    console.log('🟢 Aggregator test swap exact output passed')
    return true;
  } else {
    console.error('🔴 Aggregator test swap exact output failed')
    return false;
  }
} 

const swap_exact_tokens_for_tokens_one_protocol_two_hops = async ()=>{
 /*  const { 
    assets,
    contracts,
    testUser,
    phoenixAdmin,
    soroswap_liquidity_pools,
    phoenix_pools
  } = await prepareTestEnvironment(3); */

  const networkPassphrase = loadedConfig.passphrase;
  const soroswapRouterAddress = await (await AxiosClient.get('https://api.soroswap.finance/api/testnet/router')).data.address;
  console.log("-------------------------------------------------------");
  console.log("Creating new tokens");
  console.log("-------------------------------------------------------");
  const tokenAdmin = loadedConfig.tokenAdmin;
  const phoenixAdmin = loadedConfig.phoenixAdmin;
  const aggregatorAdmin = loadedConfig.admin;
  const testUser = loadedConfig.testUser;
  
  const assetA = generateRandomAsset();
  const assetB = generateRandomAsset();
  const assetC = generateRandomAsset();

  const cID_A = assetA.contractId(networkPassphrase);
  const cID_B = assetB.contractId(networkPassphrase);
  const cID_C = assetC.contractId(networkPassphrase);

  console.log('------------------------')
  console.log("----Using addresses:----")
  console.log('------------------------')
  console.log(`🔎 Contract ID for ${assetA.code} => ${cID_A}`)
  console.log(`🔎 Contract ID for ${assetB.code} => ${cID_B}`)
  
  console.log(`🔎 Test user => ${testUser.publicKey()}`);
  console.log(`🔎 Phoenix admin => ${phoenixAdmin.publicKey()}`);
  console.log(`🔎 Token admin => ${tokenAdmin.publicKey()}`);
  console.log("-------------------------------------------------------");
  console.log("Setting trustlines");
  console.log("-------------------------------------------------------");
  
  const assets = [assetA, assetB, assetC];

  const isTokenAdminFound = await loadedConfig.horizonRpc.loadAccount(tokenAdmin.publicKey()).catch(()=>false)

  if(!!!isTokenAdminFound){
    console.log(`🟡 Founding token admin`);
    const friendbot = await loadedConfig.horizonRpc.friendbot(tokenAdmin.publicKey());
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
  for(let asset of assets){
    console.log(asset)
    await deployAndMint(asset, testUser, "120000000000");
  }

  console.log("-------------------------------------------------------");
  console.log("Creating Soroswap liquidity pool");
  console.log("-------------------------------------------------------");
  console.log('🟡 Creating first Soroswap liquidity pool');
  const firstPoolParams = {
    contractID_A: cID_A,
    contractID_B: cID_B,
    user: testUser,
    amount_A: 100000000000000000,
    amount_B: 400000000000000000,
  };

  await create_soroswap_liquidity_pool(soroswapRouterAddress, firstPoolParams);

  const fetchFirstPoolParams: xdr.ScVal[] = [
    new Address(cID_A).toScVal(),
    new Address(cID_B).toScVal(),
  ];

  console.log('🟡 Fetching Soroswap pair address');
  const firstSoroswapPool = await invokeCustomContract(soroswapRouterAddress, 'router_pair_for', fetchFirstPoolParams, testUser, true);
  const firstSoroswapPoolCID = scValToNative(firstSoroswapPool.result.retval);
  console.log('🟢 Soroswap pair address:', firstSoroswapPoolCID)

  console.log('🟡 Fetching liquidity pool balance');
  const firstSoroswapPoolBalance = await invokeCustomContract(firstSoroswapPoolCID, 'get_reserves', [], testUser, true);
  console.log(scValToNative(firstSoroswapPoolBalance.result.retval));
  console.log(`🟢 Soroswap pair balance: ${scValToNative(firstSoroswapPoolBalance.result.retval)}`);


  console.log('-------------------------------------------------------');
  console.log('🟡 Creating second Soroswap liquidity pool');
  const secondPoolParams = {
    contractID_A: cID_B,
    contractID_B: cID_C,
    user: testUser,
    amount_A: 400000000000000000,
    amount_B: 800000000000000000,
  };

  await create_soroswap_liquidity_pool(soroswapRouterAddress, secondPoolParams);

  const fetchPoolSecondParams: xdr.ScVal[] = [
    new Address(cID_B).toScVal(),
    new Address(cID_C).toScVal(),
  ];

  console.log('🟡 Fetching Soroswap pair address');
  const secondSoroswapPool = await invokeCustomContract(soroswapRouterAddress, 'router_pair_for', fetchPoolSecondParams, testUser, true);
  const secondSoroswapPoolCID = scValToNative(secondSoroswapPool.result.retval);
  console.log('🟢 Soroswap pair address:', secondSoroswapPoolCID)

  console.log('🟡 Fetching liquidity pool balance');
  const secondSoroswapPoolBalance = await invokeCustomContract(secondSoroswapPoolCID, 'get_reserves', [], testUser, true);
  console.log(scValToNative(secondSoroswapPoolBalance.result.retval));
  console.log(`🟢 Soroswap pair balance: ${scValToNative(secondSoroswapPoolBalance.result.retval)}`);

  
  console.log('-------------------------------------------------------');
  console.log('Aggregator exact swap one protcol two hops test');
  console.log('-------------------------------------------------------');

  const soroswapAdapter =  addressBook.getContractId('soroswap_adapter');
  console.log('soroswapAdapter:', soroswapAdapter);

  const dexDistributionRaw = [
    {
      protocol_id: "soroswap",
      path: [cID_A, cID_B, cID_C],
      parts: 1,
    },
  ];

  const assetAUserBalanceBefore = await fetchAssetBalance(assets[0], testUser);
  console.log('🔎 Asset A user balance before swap:', assetAUserBalanceBefore);

  const assetCUserBalanceBefore = await fetchAssetBalance(assets[2], testUser);
  console.log('🔎 Asset C user balance before swap:', assetCUserBalanceBefore);

  const dexDistributionVec = await createDexDistribution(dexDistributionRaw);
  const swapExactIn = await callAggregatorSwap(cID_A, cID_C, 123456789, dexDistributionVec, testUser, SwapMethod.EXACT_INPUT);
  console.log('🟡 Swap exact in one protocol two hops:', swapExactIn);

  const assetAUserBalanceAfter = await fetchAssetBalance(assets[0], testUser);
  console.log('🔎 Asset A user balance after swap:', assetAUserBalanceAfter);

  const assetCUserBalanceAfter = await fetchAssetBalance(assets[2], testUser);
  console.log('🔎 Asset C user balance after swap:', assetCUserBalanceAfter);

  const expectedAmountIn0 = 123456789n;
  const expectedAmountOut0 = 492345671n;
  const expectedAmountOut1 = 981737265n;

  console.log(' -------------- Asset balances table -------------')
  console.table({
    'Initial balance': {
      'User Asset A': assetAUserBalanceBefore,
      'User Asset C': assetCUserBalanceBefore,
      'Soroswap Asset A': scValToNative(firstSoroswapPoolBalance.result.retval)[0],
      'Soroswap Asset C': scValToNative(secondSoroswapPoolBalance.result.retval)[1],
    },
    'Balance after exact output swap': {
      'User Asset A': assetAUserBalanceAfter,
      'User Asset C': assetCUserBalanceAfter,
      'Soroswap Asset A': scValToNative(firstSoroswapPoolBalance.result.retval)[0],
      'Soroswap Asset C': scValToNative(secondSoroswapPoolBalance.result.retval)[1],
    },
    'Expected amounts': {
      'Expected amount in': expectedAmountIn0,
      'Expected amount out A': expectedAmountOut0,
      'Expected amount out C': expectedAmountOut1,
    },
    'Swap result': {
      'Amount in': swapExactIn[0][0],
      'Amount out A': swapExactIn[0][1],
      'Amount out C': swapExactIn[0][2],
    }
  })

  if(
    swapExactIn[0][0] === expectedAmountIn0 && 
    swapExactIn[0][1] === expectedAmountOut0 &&
    swapExactIn[0][2] === expectedAmountOut1
  ){
    console.log('🟢 Aggregator test swap exact input one protocol two hops passed')
    return true;
  } else {
    console.error('🔴 Aggregator test swap exact input one protocol two hops failed')
    return false;
  }

}
const main = async ()=>{
  const exactInputResult = await swapExactInputAggregatorTest();
  const exactOutputResult = await swapExactOutputAggregatorTest();
  const exactInputOneProtocolTwoHops = await swap_exact_tokens_for_tokens_one_protocol_two_hops();
  console.log("-------------------------------------------------------");
  console.log("Test results");
  console.log("-------------------------------------------------------");
  console.table({
    'Exact input test': {
      'Status': exactInputResult ? '🟢 Passed' : '🔴 Failed',
    },
    'Exact output test': {
      'Status': exactOutputResult ? '🟢 Passed' : '🔴 Failed',
    },
    'Exact input one protocol two hops': {
      'Status': exactInputOneProtocolTwoHops ? '🟢 Passed' : '🔴 Failed',
    }
  })
}

main();