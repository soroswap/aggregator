import { Address, nativeToScVal, scValToNative, xdr } from '@stellar/stellar-sdk';
import { randomBytes } from 'crypto';
import { phoenixSetup } from './protocols/phoenix/phoenix_setup.js';
import { updateAdapters } from './update_protocols.js';
import { AddressBook } from './utils/address_book.js';
import { airdropAccount, deployContract, installContract, invokeContract } from './utils/contract.js';
import { config } from './utils/env_config.js';
import { TokensBook } from './utils/tokens_book.js';

export async function deployAndInitAggregator(addressBook: AddressBook) {
  // if(network == 'mainnet') throw new Error('Mainnet not yet supported')
  // await airdropAccount(loadedConfig.admin);

  // console.log('-------------------------------------------------------');
  // console.log('Deploying Deployer');
  // console.log('-------------------------------------------------------');
  // await installContract('deployer', addressBook, loadedConfig.admin);
  // await deployContract('deployer', 'deployer', addressBook, loadedConfig.admin);

  // console.log('-------------------------------------------------------');
  // console.log('Deploying Adapters using the deployer');
  // console.log('-------------------------------------------------------');
  // console.log("** Soroswap Adapter");
  // await installContract('soroswap_adapter', addressBook, loadedConfig.admin);
  
  // const routerAddress = soroswapAddressBook.getContractId('router');

  // const initArgs = xdr.ScVal.scvVec([
  //   xdr.ScVal.scvString("soroswap"), // protocol_id as ScVal string
  //   new Address(routerAddress).toScVal() // protocol_address as ScVal address
  // ]);

  // const soroswapAdapterDeployParams: xdr.ScVal[] = [
  //   new Address(loadedConfig.admin.publicKey()).toScVal(),
  //   nativeToScVal(Buffer.from(addressBook.getWasmHash("soroswap_adapter"), "hex")),
  //   nativeToScVal(randomBytes(32)),
  //   xdr.ScVal.scvSymbol('initialize'),
  //   initArgs
  // ]

  // const response = await invokeContract(
  //   'deployer',
  //   addressBook,
  //   'deploy',
  //   soroswapAdapterDeployParams,
  //   loadedConfig.admin
  // );

  // const soroswapAdapterAddress = scValToNative(response.returnValue)[0]
  // console.log('🚀 « soroswapAdapterAddress:', soroswapAdapterAddress);
  // // SAVE ADDRES IN ADDRESS BOOK
  // addressBook.setContractId("soroswap_adapter", soroswapAdapterAddress)

  // console.log('-------------------------------------------------------');
  // console.log('Deploying Aggregator');
  // console.log('-------------------------------------------------------');
  // console.log('Installing Aggregator Contract');
  // await installContract('aggregator', addressBook, loadedConfig.admin);
  
  // const adaptersVec = [
  //   {
  //     protocol_id: "soroswap",
  //     address: new Address(addressBook.getContractId('soroswap_adapter')),
  //     paused: false
  //   },
  // ];

  // const adaptersVecScVal = xdr.ScVal.scvVec(adaptersVec.map((adapter) => {
  //   return xdr.ScVal.scvMap([
  //     new xdr.ScMapEntry({
  //       key: xdr.ScVal.scvSymbol('address'),
  //       val: adapter.address.toScVal(),
  //     }),
  //     new xdr.ScMapEntry({
  //       key: xdr.ScVal.scvSymbol('paused'),
  //       val: nativeToScVal(adapter.paused),
  //     }),
  //     new xdr.ScMapEntry({
  //       key: xdr.ScVal.scvSymbol('protocol_id'),
  //       val: xdr.ScVal.scvString(adapter.protocol_id),
  //     }),
  //   ]);
  // }));

  // const initAggregatorArgs = xdr.ScVal.scvVec([
  //   new Address(loadedConfig.admin.publicKey()).toScVal(),
  //   adaptersVecScVal
  // ]);

  // const soroswapAggregatorDeployParams: xdr.ScVal[] = [
  //   new Address(loadedConfig.admin.publicKey()).toScVal(),
  //   nativeToScVal(Buffer.from(addressBook.getWasmHash("aggregator"), "hex")),
  //   nativeToScVal(randomBytes(32)),
  //   xdr.ScVal.scvSymbol('initialize'),
  //   initAggregatorArgs
  // ]

  // const response_aggregator = await invokeContract(
  //   'deployer',
  //   addressBook,
  //   'deploy',
  //   soroswapAggregatorDeployParams,
  //   loadedConfig.admin
  // );

  // const soroswapAggregatorAddress = scValToNative(response_aggregator.returnValue)[0]
  // console.log('🚀 « soroswapAggregatorAddress:', soroswapAggregatorAddress);
  // addressBook.setContractId("aggregator", soroswapAggregatorAddress)

  // console.log("Aggregator initialized")

  if (network != 'mainnet') {
    console.log("Setting up Phoenix protocol")
    // mocks
    await phoenixSetup(loadedConfig, addressBook);
    console.log("Updating adapters on aggregator.. adding Phoenix")
    await updateAdapters(addressBook);
  }

  // TODO: IF MAINNET, UPDATE PHOENIX ADAPTERS WITH MAINNET DEPLOYMENT ADDRESS
  console.log("Aggregator setup complete")
}

const network = process.argv[2];
const addressBook = AddressBook.loadFromFile(network);

const soroswapDir = network === 'standalone' ? '.soroban' : 'public';
const soroswapAddressBook = AddressBook.loadFromFile(
  network,
  `../../protocols/soroswap/${soroswapDir}`
);
const soroswapTokensBook = TokensBook.loadFromFile(
  `./protocols/soroswap/${soroswapDir}`
);

const loadedConfig = config(network);

await deployAndInitAggregator(addressBook);
addressBook.writeToFile();
