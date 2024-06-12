import { Address, nativeToScVal, xdr } from '@stellar/stellar-sdk';
import { AddressBook } from '../../utils/address_book.js';
import { airdropAccount, deployContract, installContract, invokeContract } from '../../utils/contract.js';
import { config } from '../../utils/env_config.js';
import { TokensBook } from '../../utils/tokens_book.js';
import { phoenixMultiAddLiquidity } from './multi_add_liquidity_phoenix.js';
import { deployAndInitPhoenix } from './phoenix_deploy.js';

export async function phoenixSetup() {
  if(network == 'mainnet') throw new Error('Mainnet not yet supported')
  if (network != 'mainnet') {
    // mocks
    // console.log('Installing and deploying: Phoenix Mocked Contracts');
    const phoenixAdmin = loadedConfig.getUser('PHOENIX')
    await airdropAccount(phoenixAdmin);

    const tokensAdminAccount = loadedConfig.getUser("TEST_TOKENS_ADMIN_SECRET_KEY");
    await airdropAccount(tokensAdminAccount);

    await deployAndInitPhoenix(addressBook, phoenixAdmin)
    
    console.log("Phoenix Adapter");
    console.log('Installing Phoenix Adapter Contract');
    await installContract('phoenix_adapter', addressBook, loadedConfig.admin);
    await deployContract('phoenix_adapter', 'phoenix_adapter', addressBook, loadedConfig.admin);
  
    const multihopAddress = addressBook.getContractId('phoenix_multihop');
    const phoenixAdapterInitParams: xdr.ScVal[] = [
      nativeToScVal("phoenix"), // protocol_id
      new Address(multihopAddress).toScVal(), // protocol_address (soroswap router)
    ];
  
    console.log("Initializing Soroswap Adapter")
    await invokeContract(
      'phoenix_adapter',
      addressBook,
      'initialize',
      phoenixAdapterInitParams,
      loadedConfig.admin
    );

    await phoenixMultiAddLiquidity(3, soroswapTokensBook, addressBook, phoenixAdmin, tokensAdminAccount);
  }
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

const loadedConfig = config(network);

await phoenixSetup();
addressBook.writeToFile();