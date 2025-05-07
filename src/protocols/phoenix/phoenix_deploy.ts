import { Address, Keypair, nativeToScVal } from '@stellar/stellar-sdk';
import { AddressBook } from '../../utils/address_book.js';
import { bumpContractCode, deployContract, deployContractWithArgs, installContract, invokeContract } from '../../utils/contract.js';
import { config } from '../../utils/env_config.js';


export async function deployAndInitPhoenix(addressBook: AddressBook, phoenixAdmin: Keypair) {
  console.log('Installing Phoenix Contracts');
  // Phoenix Factory
  await installContract('phoenix_factory', addressBook, phoenixAdmin);
  await bumpContractCode('phoenix_factory', addressBook, phoenixAdmin);
  // Phoenix Multihop
  await installContract('phoenix_multihop', addressBook, phoenixAdmin);
  await bumpContractCode('phoenix_multihop', addressBook, phoenixAdmin);

    
  // Phoenix Stable
  await installContract('phoenix_stable', addressBook, phoenixAdmin);
  await bumpContractCode('phoenix_stable', addressBook, phoenixAdmin);
  
  // Phoenix Token
  await installContract('phoenix_token', addressBook, phoenixAdmin);
  await bumpContractCode('phoenix_token', addressBook, phoenixAdmin);
  // Phoenix Pool
  await installContract('phoenix_pool', addressBook, phoenixAdmin);
  await bumpContractCode('phoenix_pool', addressBook, phoenixAdmin);
  // Phoenix Stake
  await installContract('phoenix_stake', addressBook, phoenixAdmin);
  await bumpContractCode('phoenix_stake', addressBook, phoenixAdmin);



  console.log('-------------------------------------------------------'); 
  console.log('Deploying Phoenix Factory With Constructor');
  console.log('-------------------------------------------------------');


  //     let args = ( 
//         admin.clone(),
//         multihop_wasm_hash,
//         lp_wasm_hash,
//         stable_wasm_hash,
//         stake_wasm_hash,
//         token_wasm_hash,
//         whitelisted_accounts,
//         10u32,
//     );

  // Initializing Phoenix Factory
  const factoryInitParams = [
    new Address(phoenixAdmin.publicKey()).toScVal(), //admin
    nativeToScVal(Buffer.from(addressBook.getWasmHash('phoenix_multihop'), 'hex')),
    nativeToScVal(Buffer.from(addressBook.getWasmHash('phoenix_pool'), 'hex')),
    nativeToScVal(Buffer.from(addressBook.getWasmHash('phoenix_stable'), 'hex')), // stable_wasm
    nativeToScVal(Buffer.from(addressBook.getWasmHash('phoenix_stake'), 'hex')),
    nativeToScVal(Buffer.from(addressBook.getWasmHash('phoenix_token'), 'hex')),
    nativeToScVal([new Address(phoenixAdmin.publicKey())]),
    nativeToScVal(7, { type: 'u32' })
  ];

  // export async function deployContractWithArgs(
  //   contractKey: string,
  //   wasmKey: string,
  //   addressBook: AddressBook,
  //   args: xdr.ScVal[],
  //   source: Keypair
  // )

  await deployContractWithArgs(
    'phoenix_factory',
    'phoenix_factory',
    addressBook,
    factoryInitParams,
    phoenixAdmin);
}

const network = process.argv[2];
const loadedConfig = config(network);
