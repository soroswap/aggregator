import { Address, nativeToScVal, xdr } from '@stellar/stellar-sdk';
import { AddressBook } from '../../utils/address_book.js';
import { airdropAccount, deployContract, installContract, invokeContract } from '../../utils/contract.js';
import { config } from '../../utils/env_config.js';
import { TokensBook } from '../../utils/tokens_book.js';
import { phoenixMultiAddLiquidity } from './multi_add_liquidity_phoenix.js';
import { deployAndInitPhoenix } from './phoenix_deploy.js';

export async function phoenixSetup(loadedConfig: any, addressBook: any) {

  const network = process.argv[2];
  const soroswapDir = network === 'standalone' ? '.soroban' : 'public';
  const soroswapTokensBook = TokensBook.loadFromFile(
   `../../protocols/soroswap/${soroswapDir}`
  );

    console.log('Loading Config for Phoenix');
    const phoenixAdmin = loadedConfig.getUser('PHOENIX_DEPLOYER_SECRET_KEY')

    console.log('Airdropping Phoenix Admin');
    await airdropAccount(phoenixAdmin);

    
    const tokensAdminAccount = loadedConfig.getUser("TEST_TOKENS_ADMIN_SECRET_KEY");
    await airdropAccount(tokensAdminAccount);

    console.log('Deploying and Initalizing Phoenix');
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
  
    console.log("Initializing Phoenix Adapter")
    await invokeContract(
      'phoenix_adapter',
      addressBook,
      'initialize',
      phoenixAdapterInitParams,
      loadedConfig.admin
    );

    await phoenixMultiAddLiquidity(3, soroswapTokensBook, addressBook, phoenixAdmin, tokensAdminAccount);
  }

// await phoenixSetup();
// addressBook.writeToFile();