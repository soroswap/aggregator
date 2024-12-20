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
  console.log('🚀 « tokens:', tokens[0]);
  console.log('🚀 « tokens:', tokens[2]);
  console.log('creating comet pair...');

  await mintToken(tokens[0].contract, 800_0000000, loadedConfig.admin.publicKey(), loadedConfig.tokenAdmin);
  await mintToken(tokens[2].contract, 200_0000000, loadedConfig.admin.publicKey(), loadedConfig.tokenAdmin);
  const createCometPairResponse = await invokeContract(
    'comet_factory',
    addressBook,
    'new_c_pool',
    [
      nativeToScVal(randomBytes(32)), //bytes32 salt
      new Address(loadedConfig.admin.publicKey()).toScVal(), //controller
      nativeToScVal([new Address(tokens[0].contract).toScVal(), new Address(tokens[2].contract).toScVal()]), //tokens
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

