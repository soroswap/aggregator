import { Address, Keypair, nativeToScVal, xdr } from 'stellar-sdk';
import { mintToken } from '../../mint_token.js';
import { AddressBook } from '../../utils/address_book.js';
import { getTokenBalance, invokeContract } from '../../utils/contract.js';
import { Token, TokensBook } from '../../utils/tokens_book.js';

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
        const tokenA = path[i];
        const tokenB = path[i+1];
        
        // Mint tokens
        await mintToken(tokenA, 25000000000000, phoenixAdmin.publicKey(), tokensAdminAccount);
        await mintToken(tokenB, 25000000000000, phoenixAdmin.publicKey(), tokensAdminAccount);
        
        console.log('-------------------------------------------------------');
        console.log("Adding liquidity for pair: ", tokenA, "|", tokenB);
        console.log("TOKEN A Balance:", await getTokenBalance(tokenA, tokensAdminAccount.publicKey(), phoenixAdmin));
        console.log("TOKEN B Balance:", await getTokenBalance(tokenB, tokensAdminAccount.publicKey(), phoenixAdmin));

        // Create Liquidity pool
        console.log("Creating liquidity Pool")
        const lpInitInfo = {
          admin: new Address(phoenixAdmin.publicKey()),
          swap_fee_bps: 1000,
          fee_recipient: new Address(phoenixAdmin.publicKey()),
          max_allowed_slippage_bps: 10000,
          max_allowed_spread_bps: 10000,
          max_referral_bps: 10000,
          token_init_info: {
            token_a: new Address(tokenA),
            token_b: new Address(tokenB)
          },
          stake_init_info: {
            min_bond: 100,
            min_reward: 100,
            manager: new Address(phoenixAdmin.publicKey()),
          }
        }
        const createLPoolParams: xdr.ScVal[] = [
          new Address(phoenixAdmin.publicKey()).toScVal(),
          nativeToScVal(lpInitInfo),
          nativeToScVal(`TOKEN${i}`),
          nativeToScVal(`TKN${i}`)
        ]

        console.log('🚀 « createLPoolParams:', createLPoolParams);
  
        await invokeContract('phoenix_factory', addressBook, 'create_liquidity_pool', createLPoolParams, phoenixAdmin);

        // console.log("Getting pair address")
        // const getPairParams: xdr.ScVal[] = [
        //   new Address(tokenA).toScVal(),
        //   new Address(tokenB).toScVal()
        // ]
        // const pairAddress = await invokeContract('phoenix_factory', addressBook, 'query_for_pool_by_token_pair', getPairParams, phoenixAdmin)
        // console.log('🚀 « pairAddress:', pairAddress);

        // console.log('Adding liquidity')
        // const addLiquidityParams: xdr.ScVal[] = [
        //   new Address(phoenixAdmin.publicKey()).toScVal(),
        //   nativeToScVal(20000000000000),
        //   nativeToScVal(20000000000000)
        // ] 
        // await invokeCustomContract(pairAddress, 'provide_liquidity', addLiquidityParams, phoenixAdmin)

        // console.log("TOKEN A Balance AFTER:", await getTokenBalance(tokenA, phoenixAdmin.publicKey(), phoenixAdmin));
        // console.log("TOKEN B Balance AFTER:", await getTokenBalance(tokenB, phoenixAdmin.publicKey(), phoenixAdmin));
      }      
    }
  } catch (error) {
    console.log('🚀 « error:', error);
    
  }
}

function generatePaths(tokens: Token[], startAddress: string, endAddress: string, numberOfPaths: number): string[][] {
  // Filter out the start and end tokens from the list to avoid including them as intermediates
  const intermediateTokens = tokens.filter(token => token.contract !== startAddress && token.contract !== endAddress);
  console.log('🚀 « intermediateTokens:', intermediateTokens);

  // Function to generate a path
  const createPath = (intermediates: Token[]): string[] => {
    return [startAddress, ...intermediates.map(token => token.contract), endAddress];
  };

  // Store generated paths
  let paths: string[][] = [];

  // Generate paths based on the number of paths requested
  for (let i = 0; i < numberOfPaths; i++) {
    // Determine the number of intermediates to include in this path
    const numIntermediates = Math.min(i, intermediateTokens.length);

    // Select intermediates for the path
    let selectedIntermediates: Token[] = [];
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
