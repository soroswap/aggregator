import { Address, nativeToScVal, xdr } from '@stellar/stellar-sdk';
import { AddressBook } from './utils/address_book.js';
import { invokeContract } from './utils/contract.js';
import { config } from './utils/env_config.js';
import { getCurrentTimePlusOneHour } from './utils/tx.js';

export async function testSoroswapAdapter(addressBook: AddressBook) {
  console.log('-------------------------------------------------------');
  console.log('Testing Phoenix Adapter');
  console.log('-------------------------------------------------------');

  const pathRaw = ["CDPU5TPNUMZ5JY3AUSENSINOEB324WI65AHI7PJBUKR3DJP2ULCBWQCS", "CCGCRYUTDRP52NOPS35FL7XIOZKKGQWSP3IYFE6B66KD4YOGJMWVC5PR"];

  const phoenixSwapParams: xdr.ScVal[] = [
    new Address(loadedConfig.admin.publicKey()).toScVal(), //admin: Address,
    nativeToScVal(pathRaw.map((pathAddress) => new Address(pathAddress))),
    nativeToScVal(1000000000, {type: "i128"}),
    nativeToScVal(0, {type: "i128"}),
    nativeToScVal(getCurrentTimePlusOneHour()), //deadline 
    nativeToScVal(true, {type: "bool"})
  ];

  await invokeContract(
    'phoenix_adapter',
    addressBook,
    'swap',
    phoenixSwapParams,
    loadedConfig.admin
  );

}

const network = process.argv[2];
const addressBook = AddressBook.loadFromFile(network);

const loadedConfig = config(network);

await testSoroswapAdapter(addressBook);