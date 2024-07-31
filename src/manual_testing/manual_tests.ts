import { Address, Contract, hash, Keypair, nativeToScVal, Operation, StrKey, xdr } from "@stellar/stellar-sdk";
import { AddressBook } from '../utils/address_book.js';
import { TokensBook, Asset } from '../utils/tokens_book.js';
import { config } from "../utils/env_config.js";
import { airdropAccount } from "../utils/contract.js";
import { readFileSync } from "fs";
import path from "path";
import { invoke } from "../utils/tx.js";
import { randomBytes } from "crypto";
import { fileURLToPath } from "url";

export async function deploySorobanToken(
  wasmKey: string,
  addressBook: AddressBook,
  source: Keypair
) {
  const contractIdSalt = randomBytes(32);
  const networkId = hash(Buffer.from(loadedConfig.passphrase));
  const contractIdPreimage = xdr.ContractIdPreimage.contractIdPreimageFromAddress(
    new xdr.ContractIdPreimageFromAddress({
      address: Address.fromString(source.publicKey()).toScAddress(),
      salt: contractIdSalt,
    })
  );

  const hashIdPreimage = xdr.HashIdPreimage.envelopeTypeContractId(
    new xdr.HashIdPreimageContractId({
      networkId: networkId,
      contractIdPreimage: contractIdPreimage,
    })
  );
  const contractId = StrKey.encodeContract(hash(hashIdPreimage.toXDR()));
  const wasmHash = Buffer.from(addressBook.getWasmHash(wasmKey), 'hex');

  const deployFunction = xdr.HostFunction.hostFunctionTypeCreateContract(
    new xdr.CreateContractArgs({
      contractIdPreimage: contractIdPreimage,
      executable: xdr.ContractExecutable.contractExecutableWasm(wasmHash),
    })
  );

  // addressBook.writeToFile();
  const result = await invoke(
    Operation.invokeHostFunction({
      func: deployFunction,
      auth: [],
    }),
    source,
    false
  );

  if (result) {
    return contractId;
  }
}

/**
 * Deploy a token contract and initialize it
 * @param name Name of the token
 * @param symbol Symbol of the token
 * @param addressBook AddressBook instance
 * @param source Keypair of the source account
 */
export async function deployToken(
  name: string,
  symbol: string,
  icon: string,
  source: Keypair,
  addressBook: AddressBook,
) {
  try {
    const contractId = await deploySorobanToken('token', addressBook, source);

    // Initializing Token
    const tokenInitParams = [
      new Address(source.publicKey()).toScVal(),
      nativeToScVal(7, { type: 'u32' }),
      nativeToScVal(name, { type: 'string' }),
      nativeToScVal(symbol, { type: 'string' }),
    ];

    const contractInstance = new Contract(contractId!);
    const contractOperation = contractInstance.call('initialize', ...tokenInitParams);
    const result = await invoke(contractOperation, source, false);

    const newToken: Asset = {
      name: name,
      contract: contractId!,
      code: symbol,
      icon: icon,
      decimals: 7,
    }

    if (result.status === 'SUCCESS') {
      return newToken
    } else {
      throw Error (`Token ${symbol} deployment failed with contractId: ${contractId}!`);
    }
  } catch (error) {
    console.log('🚀 deployToken: error:', error);
  }
}
const deploySorobanTestTokens = async (
  numberOfTokens: number, 
  resetTokensBook: boolean, 
  tokensBook: TokensBook, 
  addressBook: AddressBook, 
  source: Keypair) => {
  const fileName = network=='mainnet' ? `../../protocols/soroswap/scripts/token_name_ideas_mainnet.json` : `../../protocols/soroswap/scripts/token_name_ideas.json`;
  try {
    if (resetTokensBook) {
      tokensBook.resetNetworkTokens(network);
    }

    const tokenNameIdeas = readFileSync(path.join(__dirname, fileName));
    const tokenNameIdeasObject = JSON.parse(tokenNameIdeas.toString());
    for (let i = 0; i < numberOfTokens; i++) {
      const tokenIdea = tokenNameIdeasObject.tokens[i];
      const deployedToken = await deployToken(
        tokenIdea.name,
        tokenIdea.symbol,
        tokenIdea.icon,
        source,
        addressBook,
      );
      tokensBook.addToken(network, deployedToken!);
      console.log(
        `🚀 Token ${deployedToken?.code} deployed successfully, address ${deployedToken?.contract}`,
      );
    }
    tokensBook.writeToFile();
  } catch (error) {
    console.log("🚀 deploySorobanTestTokens: error:", error);
  }
}

const test = async (addressBook: AddressBook) => {
  if(!network) throw new Error('Please provide a network')
  if(network == 'mainnet') throw new Error('Mainnet not yet supported')
  const tokensAdminAccount = loadedConfig.getUser("TEST_TOKENS_ADMIN_SECRET_KEY");
  if (network != "mainnet") await airdropAccount(tokensAdminAccount);
  let account = await loadedConfig.horizonRpc.loadAccount(tokensAdminAccount.publicKey())
  console.log('-------------------------------------------------------');
  console.log('Deploying Soroban test tokens');
  console.log('-------------------------------------------------------');
  await deploySorobanTestTokens(3, true, tokensBook, addressBook, tokensAdminAccount);
  console.log('-------------------------------------------------------');
}

const network = process.argv[2];
const addressBook = AddressBook.loadFromFile(network);
const tokensBook = TokensBook.loadFromFile(network);
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const loadedConfig = config(network);

test(addressBook)