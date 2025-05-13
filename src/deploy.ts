import { Address, nativeToScVal, scValToNative, xdr } from '@stellar/stellar-sdk';
import { randomBytes } from 'crypto';
import { phoenixSetup } from './protocols/phoenix/phoenix_setup.js';
import { updateAdapters } from './update_protocols.js';
import { AddressBook } from './utils/address_book.js';
import { airdropAccount, deployContract, installContract, invokeContract } from './utils/contract.js';
import { config } from './utils/env_config.js';
import { TokensBook } from './utils/tokens_book.js';
import { cometSetup } from './protocols/comet/comet_setup.js';
import {Protocol} from './utils/types.js';
import { deployAdapter } from './utils/adapters.js';

export async function deployAdaptersAndAggregator(addressBook: AddressBook) {
  await airdropAccount(loadedConfig.admin);

  console.log('-------------------------------------------------------');
  console.log('Deploying Deployer');
  console.log('-------------------------------------------------------');
  await installContract('deployer', addressBook, loadedConfig.admin);
  await deployContract('deployer', 'deployer', addressBook, loadedConfig.admin);
  
  let soroswapRouter = soroswapAddressBook.getContractId('router');
  let aquaRouter = aquaAddressBook.getContractId('aqua_router');
  let phoenixMultihop;

  if (network == 'mainnet') {
    phoenixMultihop = phoenixAddressBook.getContractId('phoenix_multihop');
  }
  else { // On Tesntet we will get from contracts we have deployed
    phoenixMultihop = addressBook.getContractId('phoenix_multihop');
  }

  // console.log('-------------------------------------------------------');
  // console.log('Deploying Adapters using the deployer');
  // console.log('-------------------------------------------------------');



  // await deployAdapter(addressBook, loadedConfig, 'soroswap', soroswapRouter);
  // await deployAdapter(addressBook, loadedConfig, 'phoenix', phoenixMultihop);
  // await deployAdapter(addressBook, loadedConfig, 'aqua', aquaRouter);

  
  // console.log("** Comet Adapter");
  // await cometSetup(loadedConfig, addressBook)


  console.log('-------------------------------------------------------');
  console.log('Deploying Aggregator');
  console.log('-------------------------------------------------------');
  console.log('Installing Aggregator Contract');
  await installContract('aggregator', addressBook, loadedConfig.admin);
  
  const adaptersVec = [
    {
      protocol_id: 0,
      router: new Address(soroswapRouter),
      paused: false
    },
    {
      protocol_id: 1,
      router: new Address(aquaRouter),
      paused: false
    },
    {
      protocol_id: 2,
      router: new Address(aquaRouter),
      paused: false
    },
  ];

  const adaptersVecScVal = xdr.ScVal.scvVec(adaptersVec.map((adapter) => {
    return xdr.ScVal.scvMap([
      new xdr.ScMapEntry({
        key: xdr.ScVal.scvSymbol('paused'),
        val: nativeToScVal(adapter.paused),
      }),
      new xdr.ScMapEntry({
        key: xdr.ScVal.scvSymbol('protocol_id'),
        val: nativeToScVal(adapter.protocol_id, {type: 'u32'}),
      }),
      new xdr.ScMapEntry({
        key: xdr.ScVal.scvSymbol('router'),
        val: adapter.router.toScVal(),
      }),
    ]);
  }));

  const initAggregatorArgs = xdr.ScVal.scvVec([
    new Address(loadedConfig.admin.publicKey()).toScVal(),
    adaptersVecScVal
  ]);

  const soroswapAggregatorDeployParams: xdr.ScVal[] = [
    new Address(loadedConfig.admin.publicKey()).toScVal(),
    nativeToScVal(Buffer.from(addressBook.getWasmHash("aggregator"), "hex")),
    nativeToScVal(randomBytes(32)),
    xdr.ScVal.scvSymbol('initialize'),
    initAggregatorArgs
  ]

  const response_aggregator = await invokeContract(
    'deployer',
    addressBook,
    'deploy',
    soroswapAggregatorDeployParams,
    loadedConfig.admin
  );

  const soroswapAggregatorAddress = scValToNative(response_aggregator.returnValue)[0]
  console.log('🚀 « soroswapAggregatorAddress:', soroswapAggregatorAddress);
  addressBook.setContractId("aggregator", soroswapAggregatorAddress)

  console.log("Aggregator initialized")

  // const adaptersNames =  adaptersVec.map((adapter) => {
  //   const protocol_id = adapter.protocol_id.toString()
  //   return protocol_id + ', '
  // }
  // )
  // if (network != 'mainnet') {
  //   console.log("Setting up Phoenix protocol")

  //   await phoenixSetup(loadedConfig, addressBook);
  //   console.log("Updating adapters on aggregator.. adding: ", ...adaptersNames)
  //   await updateAdapters(addressBook, adaptersVec);
  // }

  // // TODO: IF MAINNET, UPDATE PHOENIX ADAPTERS WITH MAINNET DEPLOYMENT ADDRESS
  // console.log("Aggregator setup complete")
}

const network = process.argv[2];
const addressBook = AddressBook.loadFromFile(network);

const soroswapDir = network === 'standalone' ? '.soroban' : 'public';
const soroswapAddressBook = AddressBook.loadFromFile(
  network,
  `../../protocols/soroswap/${soroswapDir}`
);
const soroswapTokensBook = TokensBook.loadFromFile(
  `../../protocols/soroswap/${soroswapDir}`
);

const phoenixAddressBook = AddressBook.loadFromFile(
  network,
  `../../protocols/phoenix-addresses`
);

const aquaAddressBook = AddressBook.loadFromFile(
  network,
  `../../protocols/aqua-addresses`
);


const loadedConfig = config(network);

await deployAdaptersAndAggregator(addressBook);
addressBook.writeToFile();
