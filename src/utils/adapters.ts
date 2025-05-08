import { Address, nativeToScVal, scValToNative, xdr } from '@stellar/stellar-sdk';
import { randomBytes } from 'crypto';
import { AddressBook } from './address_book.js';
import { installContract, invokeContract } from './contract.js';
import { EnvConfig } from './env_config.js';
import { Protocol } from './types.js';

export async function deployAdapter(
  addressBook: AddressBook,
  loadedConfig: EnvConfig,
  protocol: Protocol,
  routerAddress: string
): Promise<string> {
  console.log('-------------------------------------------------------');
  console.log(`Deploying ${protocol} Adapter using the deployer`);
  console.log('-------------------------------------------------------');
  console.log(`** ${protocol} Adapter`);

  // Install the adapter contract
  const adapterContractName = `${protocol}_adapter`;
  await installContract(adapterContractName, addressBook, loadedConfig.admin);

  // Construct initialization arguments
  const initArgs = xdr.ScVal.scvVec([
    xdr.ScVal.scvString(protocol), // protocol_id as ScVal string
    new Address(routerAddress).toScVal(), // protocol_address as ScVal address
  ]);

  // Deployment parameters
  const deployParams: xdr.ScVal[] = [
    new Address(loadedConfig.admin.publicKey()).toScVal(), // Admin address
    nativeToScVal(Buffer.from(addressBook.getWasmHash(adapterContractName), 'hex')), // WASM hash
    nativeToScVal(randomBytes(32)), // Salt
    xdr.ScVal.scvSymbol('initialize'), // Initialize function
    initArgs, // Initialization arguments
  ];

  // Deploy the contract
  const response = await invokeContract(
    'deployer',
    addressBook,
    'deploy',
    deployParams,
    loadedConfig.admin
  );

  // Extract and save the deployed adapter address
  const adapterAddress = scValToNative(response.returnValue)[0];
  console.log(`🚀 « ${protocol}AdapterAddress:`, adapterAddress);

  // Save to AddressBook
  addressBook.setContractId(adapterContractName, adapterAddress);
  addressBook.writeToFile();

  return adapterAddress;
}