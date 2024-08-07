import { invokeCustomContract } from "../utils/contract.js";
import { 
  Address, 
  Asset, 
  BASE_FEE, 
  Horizon, 
  Keypair, 
  Operation, 
  scValToNative, 
  TransactionBuilder, 
} from "@stellar/stellar-sdk";
import { deployStellarAsset } from "../utils/contract.js";

const setTrustline = async (asset: Asset, account: Keypair, rpc: Horizon.Server, passphrase: string, limit?: string,) => {
  const loadedAccount: Horizon.AccountResponse = await rpc.loadAccount(account.publicKey());
  const operation =  Operation.changeTrust({
    asset: asset,
    limit: limit || undefined
  })

  const transaction = new TransactionBuilder(loadedAccount, {
    fee: Number(BASE_FEE).toString(),
    timebounds: { minTime: 0, maxTime: 0 },
    networkPassphrase: passphrase,
  })
    .addOperation(operation)
    .setTimeout(300)
    .build();

  const keyPair = account;
  await transaction.sign(keyPair);
  const transactionResult = await rpc.submitTransaction(transaction);
  if(transactionResult.successful) {
    console.log(`✨Trustline for ${asset.code} set`)
  }
  return transactionResult;
}


/* export async function deployStellarAsset(asset: Asset, source: Keypair, passphrase: string) {
  const xdrAsset = asset.toXDRObject();
  const networkId = hash(Buffer.from(passphrase));
  const preimage = xdr.HashIdPreimage.envelopeTypeContractId(
    new xdr.HashIdPreimageContractId({
      networkId: networkId,
      contractIdPreimage: xdr.ContractIdPreimage.contractIdPreimageFromAsset(xdrAsset),
    })
  );
  const contractId = StrKey.encodeContract(hash(preimage.toXDR()));
  console.log('🚀 « deployed Stellar Asset:', contractId);

  const deployFunction = xdr.HostFunction.hostFunctionTypeCreateContract(
    new xdr.CreateContractArgs({
      contractIdPreimage: xdr.ContractIdPreimage.contractIdPreimageFromAsset(xdrAsset),
      executable: xdr.ContractExecutable.contractExecutableStellarAsset(),
    })
  );
  return await invoke(
    Operation.invokeHostFunction({
      func: deployFunction,
      auth: [],
    }),
    source,
    false
  );
}

 */

const payment = async (destination: string, asset: Asset, amount: string, source: Keypair, rpc: Horizon.Server, passphrase: string,) => {
  const loadedSource = await rpc.loadAccount(source.publicKey());
  const operation = Operation.payment({
    destination: destination,
    asset: asset,
    amount: amount
  })

  const transaction = new TransactionBuilder(loadedSource, {
    fee: BASE_FEE,
    networkPassphrase: passphrase
  })
    .addOperation(operation)
    .setTimeout(300)
    .build();
    await transaction.sign(source);
  const transactionResult = await rpc.submitTransaction(transaction);
  if(transactionResult.successful) {
    console.log(`✨Payment of ${amount} ${asset.code} to ${destination} successful`)
  }
  return transactionResult;
}

export {
  setTrustline,
  payment, 
  deployStellarAsset
}