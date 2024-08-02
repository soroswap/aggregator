import { invokeContract, invokeCustomContract } from "../utils/contract.js";
import { AddressBook } from '../utils/address_book.js';
import { config } from '../utils/env_config.js';
import { Address, Asset, BASE_FEE, Keypair, Networks, Operation, scValToNative, TransactionBuilder } from "@stellar/stellar-sdk";

const network = process.argv[2];
const addressBook = AddressBook.loadFromFile(network);
const loadedConfig = config(network);

const setTrustline = async (asset: Asset, account: Keypair, limit?: string,) => {
  const loadedAccount = await loadedConfig.horizonRpc.loadAccount(account.publicKey());
  const operation =  Operation.changeTrust({
    asset: asset,
    limit: limit || undefined
    //limit: "0" //Remove trustline
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
  const transactionResult = await loadedConfig.horizonRpc.submitTransaction(transaction);
  return transactionResult;
}

const aggregatorManualTest = async ()=>{
  const usdc_address = "CCGCRYUTDRP52NOPS35FL7XIOZKKGQWSP3IYFE6B66KD4YOGJMWVC5PR"
  const xtar_address = "CDPU5TPNUMZ5JY3AUSENSINOEB324WI65AHI7PJBUKR3DJP2ULCBWQCS"
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
  console.log("-------------------------------------------------------");
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
  console.log("XTAR USER BALANCE:", scValToNative(xtarUserBalance.result.retval));

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
  console.log("------Contract ID's-------")
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
      await setTrustline(asset, loadedConfig.admin)
      console.log(`✨Trustline for ${asset.code} set`)
    } catch(e){
      console.log(`❌Error setting trustline for ${asset.code}`)
      console.log(e)
    }
  }
}

aggregatorManualTest()