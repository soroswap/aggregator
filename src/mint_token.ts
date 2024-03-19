import { Address, Keypair, nativeToScVal, xdr } from 'stellar-sdk';
import { invokeCustomContract } from './utils/contract.js';

export async function mintToken(contractId: string, amount: number, to: string, admin: Keypair) {
  try {
    const mintTokensParams: xdr.ScVal[] = [
      new Address(to).toScVal(),
      nativeToScVal(amount, { type: 'i128' }),
    ]

    return await invokeCustomContract(contractId, 'mint', mintTokensParams, admin);
  } catch (error) {
    console.log('🚀 « error:', error);
    
  }
}