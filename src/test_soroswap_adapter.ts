import { Address, nativeToScVal, xdr } from '@stellar/stellar-sdk';
import { AddressBook } from './utils/address_book.js';
import { invokeContract } from './utils/contract.js';
import { config } from './utils/env_config.js';
import { getCurrentTimePlusOneHour } from './utils/tx.js';

export async function testSoroswapAdapter(addressBook: AddressBook) {
  console.log('-------------------------------------------------------');
  console.log('Testing Soroswap Adapter');
  console.log('-------------------------------------------------------');

  const pathRaw = ["CAPCD5BA3VYK4YWTXUBBXKXXIETXU2GGZZIQ4KDFI4WWTVZHV6OBIUNO", "CCKW6SMINDG6TUWJROIZ535EW2ZUJQEDGSKNIK3FBK26PAMBZDVK2BZA"];

  const aggregatorSwapParams: xdr.ScVal[] = [
    new Address(loadedConfig.admin.publicKey()).toScVal(), //admin: Address,
    nativeToScVal(pathRaw.map((pathAddress) => new Address(pathAddress))),
    nativeToScVal(1000000000, {type: "i128"}),
    nativeToScVal(0, {type: "i128"}),
    nativeToScVal(getCurrentTimePlusOneHour()), //deadline 
    nativeToScVal(true, {type: "bool"})
  ];

  await invokeContract(
    'soroswap_adapter',
    addressBook,
    'swap',
    aggregatorSwapParams,
    loadedConfig.admin
  );

}

const network = process.argv[2];
const addressBook = AddressBook.loadFromFile(network);

const loadedConfig = config(network);

await testSoroswapAdapter(addressBook);