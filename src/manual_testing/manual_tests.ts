import { invokeContract } from "../utils/contract.js";
import { AddressBook } from '../utils/address_book.js';
import { config } from '../utils/env_config.js';
import { scValToNative } from "@stellar/stellar-sdk";

const aggregatorManualTest = async ()=>{
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
}


const network = process.argv[2];
const addressBook = AddressBook.loadFromFile(network);

const loadedConfig = config(network);

aggregatorManualTest()