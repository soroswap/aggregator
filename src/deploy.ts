import { Address, nativeToScVal, xdr } from '@stellar/stellar-sdk';
import { phoenixSetup } from './protocols/phoenix/phoenix_setup.js';
import { updateAdapters } from './update_protocols.js';
import { AddressBook } from './utils/address_book.js';
import { airdropAccount, bumpContractCode, deployContract, installContract, invokeContract } from './utils/contract.js';
import { config } from './utils/env_config.js';
import { TokensBook } from './utils/tokens_book.js';

export async function deployAndInitAggregator(addressBook: AddressBook) {
  if(network == 'mainnet') throw new Error('Mainnet not yet supported')
  await airdropAccount(loadedConfig.admin);

  console.log('-------------------------------------------------------');
  console.log('Deploying Adapters');
  console.log('-------------------------------------------------------');
  console.log("** Soroswap Adapter");
  await installContract('soroswap_adapter', addressBook, loadedConfig.admin);
  await deployContract('soroswap_adapter', 'soroswap_adapter', addressBook, loadedConfig.admin);

  const routerAddress = soroswapAddressBook.getContractId('router');
  const soroswapAdapterInitParams: xdr.ScVal[] = [
    nativeToScVal("soroswap"), // protocol_id
    new Address(routerAddress).toScVal(), // protocol_address (soroswap router)
  ];

  console.log("Initializing Soroswap Adapter")
  await invokeContract(
    'soroswap_adapter',
    addressBook,
    'initialize',
    soroswapAdapterInitParams,
    loadedConfig.admin
  );

  console.log('-------------------------------------------------------');
  console.log('Deploying Aggregator');
  console.log('-------------------------------------------------------');
  console.log('Installing Aggregator Contract');
  await installContract('aggregator', addressBook, loadedConfig.admin);
  await bumpContractCode('aggregator', addressBook, loadedConfig.admin);
  
  await deployContract('aggregator', 'aggregator', addressBook, loadedConfig.admin);
  
    // pub struct Adapter {
    //   pub protocol_id: String,
    //   pub address: Address,
    //   pub paused: bool,
  // }

  const adaptersVec = [
    {
      protocol_id: "soroswap",
      address: new Address(addressBook.getContractId('soroswap_adapter')),
      paused: false
    },
  ];

  const adaptersVecScVal = xdr.ScVal.scvVec(adaptersVec.map((adapter) => {
    return xdr.ScVal.scvMap([
      new xdr.ScMapEntry({
        key: xdr.ScVal.scvSymbol('address'),
        val: adapter.address.toScVal(),
      }),
      new xdr.ScMapEntry({
        key: xdr.ScVal.scvSymbol('paused'),
        val: nativeToScVal(adapter.paused),
      }),
      new xdr.ScMapEntry({
        key: xdr.ScVal.scvSymbol('protocol_id'),
        val: xdr.ScVal.scvString(adapter.protocol_id),
      }),
    ]);
  }));


  // fn initialize(e: Env, admin: Address, adapter_vec: Vec<Adapter>)
  const aggregatorInitParams: xdr.ScVal[] = [ 
    new Address(loadedConfig.admin.publicKey()).toScVal(), //admin: Address,
    adaptersVecScVal, // adapter_vec: Vec<Adapter>,
  ];

  console.log("Initializing Aggregator")
  
  await invokeContract(
    'aggregator',
    addressBook,
    'initialize',
    aggregatorInitParams,
    loadedConfig.admin
  );
  console.log("Aggregator initialized")

  if (network != 'mainnet') {
    console.log("Setting up Phoenix protocol")
    // mocks
    await phoenixSetup();
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
