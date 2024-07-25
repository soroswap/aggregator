import { Address, Keypair, nativeToScVal, scValToNative, xdr } from '@stellar/stellar-sdk';
import { mintToken } from '../../mint_token.js';
import { AddressBook } from '../../utils/address_book.js';
import { getTokenBalance, invokeContract, invokeCustomContract } from '../../utils/contract.js';
import { Asset, TokensBook } from '../../utils/tokens_book.js';
import { signWithKeypair } from '../../utils/tx.js';
import * as PhoenixFactoryContract from './bindgins/factory_bindings.js';

const network = process.argv[2];

export async function phoenixMultiAddLiquidity(numberOfPaths: number, tokensBook: TokensBook, addressBook: AddressBook, phoenixAdmin: Keypair, tokensAdminAccount: Keypair) {
  const tokens = tokensBook.getTokensByNetwork(network);
  if(!tokens || tokens.length <= 0) throw new Error('No tokens found in the tokens book');
  console.log('🚀 « tokens:', tokens[0]);

  try {
    // Generate paths
    const startAddress = tokens[1].contract;
    console.log('🚀 « startAddress:', startAddress);
    const endAddress = tokens[2].contract;
    console.log('🚀 « endAddress:', endAddress);

    const paths = generatePaths(tokens, startAddress, endAddress, numberOfPaths);
    console.log('🚀 « paths:', paths);

    for (let i = 0; i < paths.length; i++) {
      const path = paths[i];
      console.log('🚀 « path:', path);
      for (let i = 0; i < path.length - 1; i++) {
        let tokenA = path[i];
        let tokenB = path[i + 1];
        
        if (tokenA > tokenB) {
          [tokenA, tokenB] = [tokenB, tokenA];
        }
        
        // Mint tokens
        // export async function mintToken(contractId: string, amount: number, to: string, admin: Keypair) {

        await mintToken(tokenA, 25000000000000, phoenixAdmin.publicKey(), tokensAdminAccount);
        await mintToken(tokenB, 25000000000000, phoenixAdmin.publicKey(), tokensAdminAccount);
        
        console.log('-------------------------------------------------------');
        console.log("Adding liquidity for pair: ", tokenA, "|", tokenB);
        
        console.log("TOKEN A Balance:", await getTokenBalance(tokenA, phoenixAdmin.publicKey(), phoenixAdmin));
        console.log("TOKEN B Balance:", await getTokenBalance(tokenB, phoenixAdmin.publicKey(), phoenixAdmin));


        const factory_contract = new PhoenixFactoryContract.Client({
          publicKey: phoenixAdmin.publicKey()!,
          contractId: addressBook.getContractId("phoenix_factory"),
          networkPassphrase: 'Test SDF Network ; September 2015',
          rpcUrl: "https://soroban-testnet.stellar.org/",
          signTransaction: (tx: string) => signWithKeypair(tx, 'Test SDF Network ; September 2015', phoenixAdmin),
        });

        const tx = await factory_contract.create_liquidity_pool({
          sender: phoenixAdmin.publicKey(),
          lp_init_info: {
            admin: phoenixAdmin.publicKey(),
            fee_recipient: phoenixAdmin.publicKey(),
            max_allowed_slippage_bps: 4000n,
            max_allowed_spread_bps: 400n,
            max_referral_bps: 5000n,
            swap_fee_bps: 0n,
            stake_init_info: {
              manager: phoenixAdmin.publicKey(),
              max_complexity: 10,
              min_bond: 6n,
              min_reward: 3n
            },
            token_init_info: {
              token_a: tokenA,
              token_b: tokenB,
            }
          },
          share_token_name: `TOKEN${i}`,
          share_token_symbol: `TKN${i}`,
        });
        
        try {
          const result = await tx.signAndSend();
          console.log('🚀 « result:', result);
        } catch (error) {
          console.log('🚀 « error:', error);
        }

        console.log("Getting pair address")
        const getPairParams: xdr.ScVal[] = [
          new Address(tokenA).toScVal(),
          new Address(tokenB).toScVal()
        ]
        const pairAddress = await invokeContract('phoenix_factory', addressBook, 'query_for_pool_by_token_pair', getPairParams, phoenixAdmin, true)
        console.log('🚀 « pairAddress:', scValToNative(pairAddress.result.retval));

        console.log('Adding liquidity')
        const addLiquidityParams: xdr.ScVal[] = [
          new Address(phoenixAdmin.publicKey()).toScVal(),
          nativeToScVal(2000000000000, { type: "i128" }),
          nativeToScVal(null),
          nativeToScVal(2000000000000, { type: "i128" }),
          nativeToScVal(null),
          nativeToScVal(null)
        ]
        
        await invokeCustomContract(scValToNative(pairAddress.result.retval), 'provide_liquidity', addLiquidityParams, phoenixAdmin)

        console.log("TOKEN A Balance AFTER:", await getTokenBalance(tokenA, phoenixAdmin.publicKey(), phoenixAdmin));
        console.log("TOKEN B Balance AFTER:", await getTokenBalance(tokenB, phoenixAdmin.publicKey(), phoenixAdmin));
      }      
    }
  } catch (error) {
    console.log('🚀 « error:', error);
    
  }
}

function generatePaths(tokens: Asset[], startAddress: string, endAddress: string, numberOfPaths: number): string[][] {
  // Filter out the start and end tokens from the list to avoid including them as intermediates
  const intermediateTokens = tokens.filter(token => token.contract !== startAddress && token.contract !== endAddress);
  console.log('🚀 « intermediateTokens:', intermediateTokens);

  // Function to generate a path
  const createPath = (intermediates: Asset[]): string[] => {
    return [startAddress, ...intermediates.map(token => token.contract), endAddress];
  };

  // Store generated paths
  let paths: string[][] = [];

  // Generate paths based on the number of paths requested
  for (let i = 0; i < numberOfPaths; i++) {
    // Determine the number of intermediates to include in this path
    const numIntermediates = Math.min(i, intermediateTokens.length);

    // Select intermediates for the path
    let selectedIntermediates: Asset[] = [];
    for (let j = 0; j < numIntermediates; j++) {
      // Simple selection strategy: cycle through intermediates
      const intermediateIndex = (j + i) % intermediateTokens.length;
      selectedIntermediates.push(intermediateTokens[intermediateIndex]);
    }

    // Create and add the new path
    paths.push(createPath(selectedIntermediates));
  }

  return paths;
}
