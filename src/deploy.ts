import { Address, nativeToScVal, xdr } from '@stellar/stellar-sdk';
import { deployAndInitPhoenix } from './protocols/phoenix/phoenix_deploy.js';
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
  console.log("Soroswap Adapter");
  console.log('Installing Aggregator Contract');
  await installContract('soroswap_adapter', addressBook, loadedConfig.admin);
  await deployContract('soroswap_adapter', 'soroswap_adapter', addressBook, loadedConfig.admin);

  const routerAddress = "CB74KXQXEGKGPU5C5FI22X64AGQ63NANVLRZBS22SSCMLJDXNHED72MO" //soroswapAddressBook.getContractId('router');
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
  
  const protocolAddressPair = [
    {
      protocol_id: "soroswap",
      address: new Address(addressBook.getContractId('soroswap_adapter')),
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
        val: xdr.ScVal.scvString(pair.protocol_id),
      }),
    ]);
  });

  const aggregatorProtocolAddressesScVal = xdr.ScVal.scvVec(protocolAddressPairScVal);

  const aggregatorInitParams: xdr.ScVal[] = [
    new Address(loadedConfig.admin.publicKey()).toScVal(), //admin: Address,
    aggregatorProtocolAddressesScVal, // proxy_addresses: Vec<ProxyAddressPair>,
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
    const phoenixAdmin = loadedConfig.getUser('PHOENIX')
    await airdropAccount(phoenixAdmin);

    const tokensAdminAccount = loadedConfig.getUser("TEST_TOKENS_ADMIN_SECRET_KEY");
    await airdropAccount(tokensAdminAccount);

    await deployAndInitPhoenix(addressBook, phoenixAdmin)
    // TODO: Set phoenix multihop contract address to the aggregator with the update_protocols method
    // TODO: Fix phoenixMultiAddLiquidity currently is giving a scval error when trying to create the pool
    // await phoenixMultiAddLiquidity(3, soroswapTokensBook, addressBook, phoenixAdmin, tokensAdminAccount);
  }
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


// soroban contract invoke --id CA7QOHC7FFME2E7LYN675MHW4TKTEDXILQ5KRUBB2AMRVU3GN75KR2SX --source-account admin --network testnet -- initialize --admin GAZZFSUQVDVKAMQ2QTJY4DLC7HVZ37MM5SFD6CWSLE4Z3CAU4U5LC5DE --proxy_addresses '[{"address":"CB74KXQXEGKGPU5C5FI22X64AGQ63NANVLRZBS22SSCMLJDXNHED72MO","protocol_id":"soroswap"}]'
// CA7QOHC7FFME2E7LYN675MHW4TKTEDXILQ5KRUBB2AMRVU3GN75KR2SX