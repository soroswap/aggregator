import { Address, Asset, nativeToScVal, Networks, scValToNative, xdr } from '@stellar/stellar-sdk';
import { AddressBook } from './utils/address_book.js';
import { deployContract, installContract, invokeContract } from './utils/contract.js';
import { EnvConfig } from './utils/env_config.js';
import { randomBytes } from 'crypto';
import { TokensBook } from './utils/tokens_book.js';
import test from 'node:test';
import { mintToken } from './mint_token.js';
import { deployAndMint } from './manual_testing/utils.js';

async function initTokens(loadedConfig: EnvConfig, tokensBook: TokensBook) {
  const tokens = tokensBook.getTokensByNetwork('testnet');

  if (tokens != undefined && tokens.length > 1) {
    return;
  }

  tokensBook.addToken('testnet', {
    name: 'xlm',
    contract: 'CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC',
    code: 'XLM',
    decimals: 7,
  });

  let asset = new Asset('hi', loadedConfig.tokenAdmin.publicKey())

  await deployAndMint(asset, loadedConfig.admin, "100000000000");

  tokensBook.addToken('testnet', {
    name: 'hi',
    contract: asset.contractId(Networks.TESTNET),
    code: 'hi',
    decimals: 7,
  });

  tokensBook.writeToFile()
}

export async function cometSetup(loadedConfig: EnvConfig, addressBook: AddressBook) {
  const tokensBook = TokensBook.loadFromFile(`../../.soroban`);

  await initTokens(loadedConfig, tokensBook)

  const tokens = tokensBook.getTokensByNetwork("testnet")

  if (tokens == undefined) {
    throw 'could not find tokens';
  }

  let tokenA = tokens[0]; // xlm
  let tokenB = tokens[1];

  await mintToken(
    tokenB.contract,
    25000000000000,
    loadedConfig.admin.publicKey(),
    loadedConfig.tokenAdmin
  );

  console.log('uploading comet pair...');
  await installContract('comet_pool', addressBook, loadedConfig.admin);

  console.log('deploying comet factory...');
  await installContract('comet_factory', addressBook, loadedConfig.admin);
  await deployContract('comet_factory', 'comet_factory', addressBook, loadedConfig.admin);

  await invokeContract(
    'comet_factory',
    addressBook,
    'init',
    [nativeToScVal(Buffer.from(addressBook.getWasmHash('comet_pool'), 'hex'))],
    loadedConfig.admin
  );

  console.log('creating comet pair...');
  const createCometPairResponse = await invokeContract(
    'comet_factory',
    addressBook,
    'new_c_pool',
    [
      nativeToScVal(randomBytes(32)),
      new Address(loadedConfig.admin.publicKey()).toScVal(),
      nativeToScVal([new Address(tokenA.contract).toScVal(), new Address(tokenB.contract).toScVal()]),
      nativeToScVal([
        nativeToScVal(8000000, { type: 'i128' }),
        nativeToScVal(2000000, { type: 'i128' }),
      ]),
      nativeToScVal([
        nativeToScVal(800_0000000, { type: 'i128' }),
        nativeToScVal(200_0000000, { type: 'i128' }),
      ]),
      nativeToScVal(30000, { type: 'i128' }),
    ],
    loadedConfig.admin
  );
  const cometPairAddress = scValToNative(createCometPairResponse.returnValue)

  addressBook.setContractId("comet_pair", cometPairAddress)
  console.log('🚀 « comet pair address:', cometPairAddress);

  await installContract('comet_adapter', addressBook, loadedConfig.admin);

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
  // SAVE ADDRES IN ADDRESS BOOK
  addressBook.setContractId('comet_adapter', cometAdapterAddress);
}
