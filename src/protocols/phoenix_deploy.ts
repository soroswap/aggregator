import { Address } from 'stellar-sdk';
import { AddressBook } from '../utils/address_book.js';
import { airdropAccount, bumpContractCode, deployContract, installContract, invokeContract } from '../utils/contract.js';
import { config } from '../utils/env_config.js';


export async function deployAndInitPhoenix(addressBook: AddressBook) {
  const phoenixAdmin = loadedConfig.getUser('PHOENIX')
  await airdropAccount(phoenixAdmin);

  console.log('Installing Phoenix Contracts');
  // Phoenix Factory
  await installContract('phoenix_factory', addressBook, phoenixAdmin);
  await bumpContractCode('phoenix_factory', addressBook, phoenixAdmin);
  // Phoenix Multihop
  await installContract('phoenix_multihop', addressBook, phoenixAdmin);
  await bumpContractCode('phoenix_multihop', addressBook, phoenixAdmin);
  // Phoenix Token
  await installContract('phoenix_token', addressBook, phoenixAdmin);
  await bumpContractCode('phoenix_token', addressBook, phoenixAdmin);
  // Phoenix Pool
  await installContract('phoenix_pool', addressBook, phoenixAdmin);
  await bumpContractCode('phoenix_pool', addressBook, phoenixAdmin);
  // Phoenix Stake
  await installContract('phoenix_stake', addressBook, phoenixAdmin);
  await bumpContractCode('phoenix_stake', addressBook, phoenixAdmin);

  console.log('-------------------------------------------------------');
  console.log('Deploying Phoenix Factory');
  console.log('-------------------------------------------------------');
  await deployContract('phoenix_factory', 'phoenix_factory', addressBook, phoenixAdmin);

  console.log('-------------------------------------------------------');
  console.log('Deploying Phoenix Multihop');
  console.log('-------------------------------------------------------');
  await deployContract('phoenix_multihop', 'phoenix_multihop', addressBook, phoenixAdmin);

  console.log('-------------------------------------------------------');
  console.log('Initializing Phoenix Multihop');
  console.log('-------------------------------------------------------');
  // Initializing Phoenix Multihop
  const multihopInitParams = [
    new Address(phoenixAdmin.publicKey()).toScVal(),
    new Address(addressBook.getContractId('phoenix_factory')).toScVal(),
  ];
  console.log("Phoenix Factory Address", addressBook.getContractId('phoenix_factory'))
  await invokeContract('phoenix_multihop', addressBook, 'initialize', multihopInitParams, phoenixAdmin);

  // console.log('-------------------------------------------------------');
  // console.log('Initializing Phoenix Factory');
  // console.log('-------------------------------------------------------');
  // // Initializing Phoenix Factory
  // const factoryInitParams = [
  //   new Address(phoenixAdmin.publicKey()).toScVal(),
  //   nativeToScVal(Buffer.from(addressBook.getWasmHash('phoenix_multihop'), 'hex')),
  //   nativeToScVal(Buffer.from(addressBook.getWasmHash('phoenix_pool'), 'hex')),
  //   nativeToScVal(Buffer.from(addressBook.getWasmHash('phoenix_stake'), 'hex')),
  //   nativeToScVal(Buffer.from(addressBook.getWasmHash('phoenix_token'), 'hex')),
  //   nativeToScVal([{address: new Address(phoenixAdmin.publicKey())}])
  // ];
  // await invokeContract('phoenix_factory', addressBook, 'initialize', factoryInitParams, phoenixAdmin);

  // if (network != 'mainnet') {
  //   // mocks
  //   console.log('Installing and deploying: Phoenix Mocked Contracts');
  // }
  // console.log('Deploying and Initializing Soroswap Aggregator');
}

const network = process.argv[2];

const soroswapDir = network === 'standalone' ? '.soroban' : 'public';
const soroswapAddressBook = AddressBook.loadFromFile(
  network,
  `../../contracts/protocols/soroswap/${soroswapDir}`
);

const loadedConfig = config(network);
