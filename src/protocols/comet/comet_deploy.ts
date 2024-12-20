import { Address, Keypair, nativeToScVal } from '@stellar/stellar-sdk';
import { AddressBook } from '../../utils/address_book.js';
import { bumpContractCode, deployContract, installContract, invokeContract } from '../../utils/contract.js';
import { config } from '../../utils/env_config.js';


export async function deployAndInitComet(addressBook: AddressBook, cometAdmin: Keypair) {
  console.log('Installing comet Contracts');
  // comet Factory
  console.log('-------------------------------------------------------');
  console.log('Install and deploy Comet factory');
  console.log('-------------------------------------------------------');
  await installContract('comet_factory', addressBook, cometAdmin);
  await bumpContractCode('comet_factory', addressBook, cometAdmin);
  
  // comet Pool
  console.log('-------------------------------------------------------');
  console.log('Install and deploy Comet pool');
  console.log('-------------------------------------------------------');
  await installContract('comet_pool', addressBook, cometAdmin);
  await bumpContractCode('comet_pool', addressBook, cometAdmin);

  // comet adapter
  console.log('-------------------------------------------------------');
  console.log('Install and deploy Comet adapter');
  console.log('-------------------------------------------------------');
  await installContract('comet_adapter', addressBook, cometAdmin);
  await bumpContractCode('comet_adapter', addressBook, cometAdmin);

  console.log('-------------------------------------------------------');
  console.log('Install and deploy Comet factory');
  console.log('-------------------------------------------------------');
  await deployContract('comet_factory', 'comet_factory', addressBook, cometAdmin);

  console.log('-------------------------------------------------------');
  console.log('Initializing comet Factory');
  console.log('-------------------------------------------------------');

  // Initializing comet Factory
  await invokeContract(
    'comet_factory',
    addressBook,
    'init',
    [nativeToScVal(Buffer.from(addressBook.getWasmHash('comet_pool'), 'hex'))],
    loadedConfig.admin
  );
}

const network = process.argv[2];
const loadedConfig = config(network);
