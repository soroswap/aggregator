import { Address, Asset, nativeToScVal, scValToNative } from '@stellar/stellar-sdk';
import { AddressBook } from '../../utils/address_book.js';
import { invokeContract } from '../../utils/contract.js';
import { EnvConfig } from '../../utils/env_config.js';
import { randomBytes } from 'crypto';
import { TokensBook } from '../../utils/tokens_book.js';
import { mintToken } from '../../mint_token.js';

export async function createCometPool(addressBook: AddressBook, tokensBook:TokensBook, network: string, loadedConfig: EnvConfig,) {
  const tokens = tokensBook.getTokensByNetwork(network);
  if(!tokens || tokens.length <= 0) throw new Error('No tokens found in the tokens book');
  console.log('🚀 « tokens:', tokens[3]);
  console.log('🚀 « tokens:', tokens[2]);
  console.log('creating comet pair...');

  const tokenPair = [tokens[3].contract, tokens[2].contract];
  if(tokens[2].contract > tokens[3].contract) {
    tokenPair[0] = tokens[2].contract;
    tokenPair[1] = tokens[3].contract;
  }
  await mintToken(tokenPair[0], 800_0000000, loadedConfig.admin.publicKey(), loadedConfig.tokenAdmin);
  await mintToken(tokenPair[1], 200_0000000, loadedConfig.admin.publicKey(), loadedConfig.tokenAdmin);
  const createCometPairResponse = await invokeContract(
    'comet_factory',
    addressBook,
    'new_c_pool',
    [
      nativeToScVal(randomBytes(32)), //bytes32 salt
      new Address(loadedConfig.admin.publicKey()).toScVal(), //controller
      nativeToScVal([new Address(tokenPair[0]).toScVal(), new Address(tokenPair[1]).toScVal()]), //tokens
      nativeToScVal([  //weights
        nativeToScVal(8000000, { type: 'i128' }),
        nativeToScVal(2000000, { type: 'i128' }),
      ]),
      nativeToScVal([ //balances
        nativeToScVal(800_0000000, { type: 'i128' }),
        nativeToScVal(200_0000000, { type: 'i128' }),
      ]),//swap_fee
      nativeToScVal(30_000, { type: 'i128' }),
    ],
    loadedConfig.admin
  );
  const cometPairAddress = scValToNative(createCometPairResponse.returnValue)
  console.log('🚀 « comet pair address:', cometPairAddress);
  return cometPairAddress;
}

