import { Address, xdr } from 'stellar-sdk';
import { deployAndInitPhoenix } from './protocols/phoenix_deploy.js';
import { AddressBook } from './utils/address_book.js';
import {
  airdropAccount,
  bumpContractCode,
  bumpContractInstance,
  deployContract,
  installContract,
  invokeContract,
} from './utils/contract.js';
import { config } from './utils/env_config.js';
import { TokensBook } from './utils/tokens_book.js';

export async function deployAndInitAggregator(addressBook: AddressBook) {
  await airdropAccount(loadedConfig.admin);

  console.log('Installing Aggregator Contract');
  await installContract('aggregator', addressBook, loadedConfig.admin);
  await bumpContractCode('aggregator', addressBook, loadedConfig.admin);

  console.log('-------------------------------------------------------');
  console.log('Deploying and Initializing Soroswap Aggregator');
  console.log('-------------------------------------------------------');
  await deployContract('aggregator', 'aggregator', addressBook, loadedConfig.admin);
  await bumpContractInstance('aggregator', addressBook, loadedConfig.admin);

  const routerAddress = soroswapAddressBook.getContractId('router');
  console.log("Soroswap Router Address", routerAddress)
  const protocolAddressPair = [
    {
      protocol_id: 0,
      address: new Address(routerAddress),
    },
  ];

  const protocolAddressPairScVal = protocolAddressPair.map((pair) => {
    return xdr.ScVal.scvMap([
      new xdr.ScMapEntry({
        key: xdr.ScVal.scvSymbol('address'),
        val: pair.address.toScVal(),
      }),
      new xdr.ScMapEntry({
        key: xdr.ScVal.scvSymbol('protocol_id'),
        val: xdr.ScVal.scvI32(pair.protocol_id),
      }),
    ]);
  });

  const aggregatorProtocolAddressesScVal = xdr.ScVal.scvVec(protocolAddressPairScVal);

  const aggregatorInitParams: xdr.ScVal[] = [
    new Address(loadedConfig.admin.publicKey()).toScVal(),
    aggregatorProtocolAddressesScVal,
  ];

  console.log("Initializing Aggregator")
  await invokeContract(
    'aggregator',
    addressBook,
    'initialize',
    aggregatorInitParams,
    loadedConfig.admin
  );

  if (network != 'mainnet') {
    // mocks
    console.log('Installing and deploying: Phoenix Mocked Contracts');
    await deployAndInitPhoenix(addressBook)
    // await phoenixMultiAddLiquidity(3, soroswapTokensBook, addressBook);
  }
  // console.log('Deploying and Initializing Soroswap Aggregator');
}

const network = process.argv[2];
const addressBook = AddressBook.loadFromFile(network);

const soroswapDir = network === 'standalone' ? '.soroban' : 'public';
const soroswapAddressBook = AddressBook.loadFromFile(
  network,
  `../../contracts/protocols/soroswap/${soroswapDir}`
);
const soroswapTokensBook = TokensBook.loadFromFile(
  `../../contracts/protocols/soroswap/${soroswapDir}`
);

const loadedConfig = config(network);

await deployAndInitAggregator(addressBook);
addressBook.writeToFile();
