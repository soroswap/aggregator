// import {
//   BackstopClient,
//   EmitterClient,
//   Network,
//   PoolFactoryClient,
//   PoolInitMeta,
//   TxOptions,
// } from '@blend-capital/blend-sdk';
// import { Asset } from 'stellar-sdk';
// import { CometClient } from '../external/comet.js';
// import { tryDeployStellarAsset } from '../external/token.js';
import { Address, nativeToScVal, xdr } from 'stellar-sdk';
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

  // Constructing the ProtocolAddressPair in ScVal format
  const protocolAddressPairScVal = [
    {
      protocol_id: 0,
      address: new Address(routerAddress),
    },
  ].map((pair) => {
    return xdr.ScVal.scvMap([
      new xdr.ScMapEntry({
        key: nativeToScVal('protocol_id'),
        val: xdr.ScVal.scvI32(pair.protocol_id),
      }),
      new xdr.ScMapEntry({
        key: nativeToScVal('address'),
        val: pair.address.toScVal(),
      }),
    ]);
  });

  // Assuming protocolAddressPairScVal is now an array of ScMapEntry or similar that represents each ProtocolAddressPair
  const aggregatorProtocolAddressesScVal = xdr.ScVal.scvVec(protocolAddressPairScVal);

  // Initializing Soroswap Aggregator
  const aggregatorInitParams = [
    new Address(loadedConfig.admin.publicKey()).toScVal(),
    aggregatorProtocolAddressesScVal,
  ];

  console.log('🚀 « aggregatorInitParams:', aggregatorInitParams);
  await invokeContract(
    'aggregator',
    addressBook,
    'initialize',
    aggregatorInitParams,
    loadedConfig.admin
  );

  console.log(rpc_network);
  // add any other contracts here router / factory / etc same idea install and bump
  // if (network != 'mainnet') {
  //   // mocks
  //   console.log('Installing and deploying: Phoenix Mocked Contracts');
  // }
  // console.log('Deploying and Initializing Soroswap Aggregator');
}

const network = process.argv[2];
const addressBook = AddressBook.loadFromFile(network);

const soroswapDir = network === 'standalone' ? '.soroban' : 'public';
const soroswapAddressBook = AddressBook.loadFromFile(
  network,
  `../../contracts/protocols/soroswap/${soroswapDir}`
);

const loadedConfig = config(network);
const rpc_network = {
  rpc: loadedConfig.rpc.serverURL.toString(),
  passphrase: loadedConfig.passphrase,
  opts: { allowHttp: true },
};

await deployAndInitAggregator(addressBook);
addressBook.writeToFile();
