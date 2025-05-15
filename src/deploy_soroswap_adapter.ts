import { Address, nativeToScVal, scValToNative, xdr } from '@stellar/stellar-sdk';
import { randomBytes } from 'crypto';
import { updateAdapters } from './update_protocols.js';
import { AddressBook } from './utils/address_book.js';
import { installContract, invokeContract } from './utils/contract.js';
import { config } from './utils/env_config.js';

export async function deploySoroswapAdapter(addressBook: AddressBook) {
  

  console.log('-------------------------------------------------------');
  console.log('Deploying Adapter using the deployer');
  console.log('-------------------------------------------------------');
  console.log("** Aqua Adapter");
  await installContract('aqua_adapter', addressBook, loadedConfig.admin);
  console.log('🚀 « aqua_adapter deployed');

  
  const aquaRouterAddress = aquaAddressBook.getContractId('aqua_router');

  const aquaAdapterInitParams = xdr.ScVal.scvVec([
    xdr.ScVal.scvString("aqua"), // protocol_id
    new Address(aquaRouterAddress).toScVal(), // protocol_address 
  ]);

  const aquaAdapterDeployParams: xdr.ScVal[] = [
    new Address(loadedConfig.admin.publicKey()).toScVal(),
    nativeToScVal(Buffer.from(addressBook.getWasmHash("aqua_adapter"), "hex")),
    nativeToScVal(randomBytes(32)),
    xdr.ScVal.scvSymbol('initialize'),
    aquaAdapterInitParams
  ]

  const response = await invokeContract(
    'deployer',
    addressBook,
    'deploy',
    aquaAdapterDeployParams,
    loadedConfig.admin
  );

  const aquaAdapterAddress = scValToNative(response.returnValue)[0]
  console.log('🚀 « aquaAdapterAddress:', aquaAdapterAddress);
  // SAVE ADDRES IN ADDRESS BOOK
  addressBook.setContractId("aqua_adapter", aquaAdapterAddress)

  console.log("Updating adapters on aggregator.. adding Aqua")
  await updateAdapters(addressBook, [{protocol_id: 'aqua', address: new Address(aquaAdapterAddress), paused: false}]);

}

const network = process.argv[2];
if(network != 'mainnet') throw new Error('Only Mainnet is Supported')

const addressBook = AddressBook.loadFromFile(network);

const aquaAddressBook = AddressBook.loadFromFile(
  network,
  `../../protocols/aqua-addresses`
);

const loadedConfig = config(network);

await deploySoroswapAdapter(addressBook);
addressBook.writeToFile();
