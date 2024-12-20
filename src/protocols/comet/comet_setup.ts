import { Address, nativeToScVal, scValToNative, xdr } from '@stellar/stellar-sdk';
import { airdropAccount, deployContract, installContract, invokeContract } from '../../utils/contract.js';
import { TokensBook } from '../../utils/tokens_book.js';
import { deployAndInitComet } from './comet_deploy.js';
import { createCometPool } from './create_pool.js';
import { randomBytes } from 'crypto';

export async function cometSetup(loadedConfig: any, addressBook: any) {
  // Preparing setup
  const network = process.argv[2];
  const soroswapDir = network === 'standalone' ? '.soroban' : 'public';
  const soroswapTokensBook = TokensBook.loadFromFile(
   `../../protocols/soroswap/${soroswapDir}`
  );
  // Preoaring users and admin
  console.log('Loading Config for Comet');
  const admin = loadedConfig.getUser('TEST_USER_SECRET_KEY')

  console.log('Airdropping Admin');
  await airdropAccount(admin);

  const tokensAdminAccount = loadedConfig.getUser("TEST_TOKENS_ADMIN_SECRET_KEY");
  await airdropAccount(tokensAdminAccount);

  // Deploying and Initializing Comet
  console.log('Deploying and Initalizing Comet');
  await deployAndInitComet(addressBook, admin)
  
  console.log('Installing Comet Adapter contract');
  await installContract('comet_adapter', addressBook, loadedConfig.admin);
  await deployContract('comet_adapter', 'comet_adapter', addressBook, loadedConfig.admin);

  // Initializing Comet Adapter
  console.log("Initializing Comet adapter")

  //Creating Comet Pool
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
  return cometAdapterAddress;
}