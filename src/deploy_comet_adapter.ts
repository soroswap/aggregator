import { Address, nativeToScVal, scValToNative, xdr } from '@stellar/stellar-sdk';
import { randomBytes } from 'crypto';
import { updateAdapters } from './update_protocols.js';
import { AddressBook } from './utils/address_book.js';
import { installContract, invokeContract } from './utils/contract.js';
import { config } from './utils/env_config.js';
import { TokensBook } from './utils/tokens_book.js';
import { createCometPool } from './protocols/comet/create_pool.js';

export async function deployCometAdapter(addressBook: AddressBook) {
  // this is ment to be only for mainnet

  if(network != 'mainnet') throw new Error('Only Mainnet is Supported')
  // await airdropAccount(loadedConfig.admin);

  console.log('-------------------------------------------------------');
  console.log('Deploying Adapter using the deployer');
  console.log('-------------------------------------------------------');
  console.log("** Comet Adapter");
  await installContract('comet_adapter', addressBook, loadedConfig.admin);

  const soroswapTokensBook = TokensBook.loadFromFile(
    `../../protocols/soroswap/public`
   );

  const cometPairAddress = await createCometPool(addressBook, soroswapTokensBook,  network, loadedConfig);
    const initArgs = xdr.ScVal.scvVec([
      xdr.ScVal.scvString('comet_blend'), // protocol_id as ScVal string
      new Address(cometPairAddress).toScVal(), // protocol_address as ScVal address
    ]);
  
    const cometAdapterDeployParams: xdr.ScVal[] = [
      new Address(loadedConfig.admin.publicKey()).toScVal(),
      nativeToScVal(Buffer.from(addressBook.getWasmHash('comet_adapter'), 'hex')),
      nativeToScVal(randomBytes(32)),
      xdr.ScVal.scvSymbol('initialize'),
      initArgs,
    ];
    const response = await invokeContract(
      'deployer',
      addressBook,
      'deploy',
      cometAdapterDeployParams,
      loadedConfig.admin
    );
    const cometAdapterAddress = scValToNative(response.returnValue)[0];
    console.log('🚀 « cometAdapterAddress:', cometAdapterAddress);
    addressBook.setContractId('comet_adapter', cometAdapterAddress);

  console.log("Updating adapters on aggregator.. adding Comet")
  await updateAdapters(addressBook, [{protocol_id: 'comet_blend', address: new Address(cometAdapterAddress), paused: false}]);

}

const network = process.argv[2];
if(network != 'mainnet') throw new Error('Only Mainnet is Supported')

const addressBook = AddressBook.loadFromFile(network);
const loadedConfig = config(network);

await deployCometAdapter(addressBook);
addressBook.writeToFile();
