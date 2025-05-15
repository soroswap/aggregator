import { Address, Keypair, nativeToScVal, scValToNative, xdr } from '@stellar/stellar-sdk';
import { mintToken } from '../../mint_token.js';
import { AddressBook } from '../../utils/address_book.js';
import { getTokenBalance, invokeContract, invokeCustomContract } from '../../utils/contract.js';
import { Asset, TokensBook } from '../../utils/tokens_book.js';
import { signWithKeypair } from '../../utils/tx.js';
import * as PhoenixFactoryContract from './bindgins/factory_bindings.js';

const network = process.argv[2];
import { StrKey } from "@stellar/stellar-sdk";

function compareAddressesAsBytes(tokenA: string, tokenB: string): number {
    // Decode StrKey to raw bytes
    const bytesA = StrKey.decodeContract(tokenA); // Use decodeEd25519PublicKey for accounts
    const bytesB = StrKey.decodeContract(tokenB);

    // Compare byte arrays lexicographically
    for (let i = 0; i < 32; i++) {
        if (bytesA[i] < bytesB[i]) return -1;
        if (bytesA[i] > bytesB[i]) return 1;
    }
    return 0; // Equal
}

function sortTokens(tokenA: string, tokenB: string): [string, string] {
    console.log("Previous tokenA:", tokenA);
    console.log("Previous tokenB:", tokenB);
    if (tokenA === tokenB) {
        console.error(`Tokens must be different: ${tokenA}`);
        throw new Error("IdenticalTokens");
    }
    // Use byte comparison instead of string comparison
    if (compareAddressesAsBytes(tokenA, tokenB) > 0) {
        console.log('🚀 « tokenA > tokenB (byte comparison):', tokenA, tokenB);
        console.log("tokenA:", tokenB);
        console.log("tokenB:", tokenA);
        return [tokenB, tokenA];
    }
    console.log("tokenA:", tokenA);
    console.log("tokenB:", tokenB);
    return [tokenA, tokenB];
}

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
        let [tokenA, tokenB] = sortTokens(path[i], path[i + 1]);
        
        
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

      //   factory.create_liquidity_pool(
      //     &admin,
      //     &lp_init_info,
      //     &String::from_str(&env, "Pool"),
      //     &String::from_str(&env, "PHO/BTC"),
      //     &PoolType::Xyk,
      //     &None::<u64>,
      //     &100i64,
      //     &1_000,
      // );
    //   fn create_liquidity_pool(
    //     env: Env,
    //     sender: Address,
    //     lp_init_info: LiquidityPoolInitInfo,
    //     share_token_name: String,
    //     share_token_symbol: String,
    //     pool_type: PoolType,
    //     amp: Option<u64>,
    //     default_slippage_bps: i64,
    //     max_allowed_fee_bps: i64,
    // ) 
  //   let lp = factory.create_liquidity_pool(
  //     &admin.clone(), //     sender: Address,
  //     &lp_init_info, //     lp_init_info: LiquidityPoolInitInfo,
  //     &String::from_str(env, "Pool"),  //     share_token_name: String,
  //     &String::from_str(env, "PHO/XLM"),//     share_token_symbol: String,
  //     &PoolType::Xyk, //     pool_type: PoolType,
  //     &None::<u64>,//     amp: Option<u64>,
  //     &100i64, //     default_slippage_bps: i64,
  //     &2_000,//     max_allowed_fee_bps: i64,
  // );
        const tx = await factory_contract.create_liquidity_pool({  
          sender: phoenixAdmin.publicKey(),
          lp_init_info: {
            admin: phoenixAdmin.publicKey(),
            fee_recipient: phoenixAdmin.publicKey(),
            max_allowed_slippage_bps: 4000n,
            default_slippage_bps: 2500n,
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
          pool_type: PhoenixFactoryContract.PoolType.Xyk,
          amp: 0n,
    default_slippage_bps: 100n,
    max_allowed_fee_bps: 2000n,
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
        // fn provide_liquidity(
    //     env: Env,
    //     depositor: Address,
    //     desired_a: Option<i128>,
    //     min_a: Option<i128>,
    //     desired_b: Option<i128>,
    //     min_b: Option<i128>,
    //     custom_slippage_bps: Option<i64>,
    //     deadline: Option<u64>,
    // );
        const addLiquidityParams: xdr.ScVal[] = [
          new Address(phoenixAdmin.publicKey()).toScVal(),
          nativeToScVal(2000000000000, { type: "i128" }),
          nativeToScVal(null),
          nativeToScVal(2000000000000, { type: "i128" }),
          nativeToScVal(null),
          nativeToScVal(null),
          nativeToScVal(null),
          nativeToScVal(false),
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
