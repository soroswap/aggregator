import { Address, nativeToScVal, xdr } from '@stellar/stellar-sdk';
import { AddressBook } from './utils/address_book.js';
import { invokeContract } from './utils/contract.js';
import { config } from './utils/env_config.js';
import { getCurrentTimePlusOneHour } from './utils/tx.js';
import { TokensBook } from './utils/tokens_book.js';

export async function testCometAdapter(addressBook: AddressBook) {
  console.log('-------------------------------------------------------');
  console.log('Testing Comet Adapter');
  console.log('-------------------------------------------------------');

  const tokens = TokensBook.loadFromFile().getTokensByNetwork("testnet")!!

  const pathRaw = [tokens[0].contract, tokens[1].contract];

  const aggregatorSwapParams: xdr.ScVal[] = [
    nativeToScVal(1_000_000_0, {type: "i128"}),
    nativeToScVal(0, {type: "i128"}),
    nativeToScVal(pathRaw.map((pathAddress) => new Address(pathAddress))),
    new Address(loadedConfig.admin.publicKey()).toScVal(), //admin: Address,
    nativeToScVal(getCurrentTimePlusOneHour()), //deadline 
  ];

  await invokeContract(
    'comet_adapter',
    addressBook,
    'swap_exact_tokens_for_tokens',
    aggregatorSwapParams,
    loadedConfig.admin
  );

}

const network = process.argv[2];
const addressBook = AddressBook.loadFromFile(network);

const loadedConfig = config(network);

await testCometAdapter(addressBook);