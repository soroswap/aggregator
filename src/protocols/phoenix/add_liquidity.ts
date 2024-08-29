import { AddressBook } from "../../utils/address_book.js";
import { airdropAccount } from "../../utils/contract.js";
import { config } from "../../utils/env_config.js";
import { TokensBook } from "../../utils/tokens_book.js";
import { phoenixMultiAddLiquidity } from "./multi_add_liquidity_phoenix.js";

const network = process.argv[2];

const addPhoneixLiquidity = async () => {
  if(!network) throw new Error('Network is required, please run the script as "yarn add-liquidity:<protocol> <network> <numberOfTokens>"');
  const loadedConfig = config(network);
  const soroswapDir = network === 'standalone' ? '.soroban' : 'public';
  const soroswapTokensBook = TokensBook.loadFromFile(
    `../../protocols/soroswap/${soroswapDir}`
    
  );
  const numberOfTokens = process.argv[3] ? parseInt(process.argv[3]) : soroswapTokensBook.getTokensByNetwork(network)?.length!;
  console.log('Number of tokens', numberOfTokens);
  const phoenixAdmin = loadedConfig.getUser('PHOENIX_DEPLOYER_SECRET_KEY')
  const addressBook = AddressBook.loadFromFile(network);

  console.log('Airdropping Phoenix Admin');
  await airdropAccount(phoenixAdmin);

  
  const tokensAdminAccount = loadedConfig.getUser("TEST_TOKENS_ADMIN_SECRET_KEY");
  await airdropAccount(tokensAdminAccount);
  await phoenixMultiAddLiquidity(numberOfTokens, soroswapTokensBook, addressBook, phoenixAdmin, tokensAdminAccount);
}

await addPhoneixLiquidity()