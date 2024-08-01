import { invokeContract, invokeCustomContract } from "../utils/contract.js";
import { AddressBook } from '../utils/address_book.js';
import { config } from '../utils/env_config.js';
import { Address, Asset, BASE_FEE, Keypair, Networks, Operation, scValToNative, TransactionBuilder } from "@stellar/stellar-sdk";

const aggregatorManualTest = async (networkPassphrase: string)=>{
  const usdc_address = "CCGCRYUTDRP52NOPS35FL7XIOZKKGQWSP3IYFE6B66KD4YOGJMWVC5PR"
  const xtar_address = "CDPU5TPNUMZ5JY3AUSENSINOEB324WI65AHI7PJBUKR3DJP2ULCBWQCS"

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
  const operation =  Operation.changeTrust({
    asset: assetA,
    limit: '5000000',
    source: tokenAdmin,
  })
  console.log(operation.toXDR('base64'))

  const source = await loadedConfig.horizonRpc.loadAccount(tokenAdmin);

  const operation2 =  Operation.changeTrust({
    asset: assetA,
    source: source.account_id,
  })

  const txn = new TransactionBuilder(source, {
    fee: Number(BASE_FEE).toString(),
    timebounds: { minTime: 0, maxTime: 0 },
    networkPassphrase: loadedConfig.passphrase,
  })
    .addOperation(operation2)
    .setTimeout(300)
    .build();

  const keyPair = Keypair.fromSecret(loadedConfig.tokenAdmin.secret());
  await txn.sign(keyPair);
  console.log(txn.toXDR())
  
}


const network = process.argv[2];
const addressBook = AddressBook.loadFromFile(network);

const loadedConfig = config(network);

let networkPassphrase = Networks.TESTNET
switch(network.toLocaleUpperCase()){
  case 'TESTNET':
    networkPassphrase = Networks.TESTNET;
    break;
  case 'STANDALONE':
    networkPassphrase = Networks.STANDALONE;
    break;
  default:
    throw new Error('Network not supported.')
}

aggregatorManualTest(networkPassphrase)