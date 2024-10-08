import { Address, nativeToScVal, scValToNative, xdr } from '@stellar/stellar-sdk';
import { randomBytes } from 'crypto';
import { phoenixSetup } from './protocols/phoenix/phoenix_setup.js';
import { updateAdapters } from './update_protocols.js';
import { AddressBook } from './utils/address_book.js';
import { airdropAccount, deployContract, installContract, invokeContract } from './utils/contract.js';
import { config } from './utils/env_config.js';
import { TokensBook } from './utils/tokens_book.js';

export async function deployPhoenixAdapter(addressBook: AddressBook) {
  // this is ment to be only for mainnet

  if(network != 'mainnet') throw new Error('Only Mainnet is Supported')
  // await airdropAccount(loadedConfig.admin);

  // deployer has already been deployed.

  // console.log('-------------------------------------------------------');
  // console.log('Deploying Adapter using the deployer');
  // console.log('-------------------------------------------------------');
  // console.log("** Phoenix Adapter");
  // await installContract('phoenix_adapter', addressBook, loadedConfig.admin);

  
  
  
  const multihopAddress = phoenixAddressBook.getContractId('phoenix_multihop');

  const phoenixAdapterInitParams = xdr.ScVal.scvVec([
    xdr.ScVal.scvString("phoenix"), // protocol_id
    new Address(multihopAddress).toScVal(), // protocol_address (soroswap router)
  ]);

  const phoenixAdapterDeployParams: xdr.ScVal[] = [
    new Address(loadedConfig.admin.publicKey()).toScVal(),
    nativeToScVal(Buffer.from(addressBook.getWasmHash("phoenix_adapter"), "hex")),
    nativeToScVal(randomBytes(32)),
    xdr.ScVal.scvSymbol('initialize'),
    phoenixAdapterInitParams
  ]

  const response = await invokeContract(
    'deployer',
    addressBook,
    'deploy',
    phoenixAdapterDeployParams,
    loadedConfig.admin
  );

  const phoenixAdapterAddress = scValToNative(response.returnValue)[0]
  console.log('🚀 « phoenixAdapterAddress:', phoenixAdapterAddress);
  // SAVE ADDRES IN ADDRESS BOOK
  addressBook.setContractId("phoenix_adapter", phoenixAdapterAddress)

  console.log("Updating adapters on aggregator.. adding Phoenix")
  await updateAdapters(addressBook);

}

const network = process.argv[2];
if(network != 'mainnet') throw new Error('Only Mainnet is Supported')

const addressBook = AddressBook.loadFromFile(network);

const phoenixAddressBook = AddressBook.loadFromFile(
  network,
  `../../protocols/phoenix-addresses`
);

const loadedConfig = config(network);

await deployPhoenixAdapter(addressBook);
addressBook.writeToFile();
