import type {
  i128,
  i64,
  Option,
  u32
} from "@stellar/stellar-sdk/contract";
import {
  AssembledTransaction,
  Client as ContractClient,
  ClientOptions as ContractClientOptions,
  Spec as ContractSpec,
} from "@stellar/stellar-sdk/contract";
import { Buffer } from "buffer";

export * from "@stellar/stellar-sdk";
export * as contract from "@stellar/stellar-sdk/contract";
export * as rpc from "@stellar/stellar-sdk/rpc";

export const networks = {
  testnet: {
    networkPassphrase: "Test SDF Network ; September 2015",
    contractId: "CDZH5K5YC7CXZKRCVHKVKQAB6QQUTJY7FDW6FLMFSLCAPXU5W43JCTAL",
  }
} as const

export const Errors = {
  1: {message:""},
  2: {message:""},
  3: {message:""},
  4: {message:""},
  5: {message:""},
  6: {message:""},
  7: {message:""},
  8: {message:""},
  9: {message:""},
  10: {message:""},
  11: {message:""},
  12: {message:""},
  13: {message:""},
  14: {message:""},
  15: {message:""},
  16: {message:""},
  17: {message:""},
  18: {message:""},
  19: {message:""},
  20: {message:""},
  21: {message:""}
}
export enum PairType {
  Xyk = 0,
}


export interface Config {
  fee_recipient: string;
  /**
 * The maximum amount of slippage (in bps) that is tolerated during providing liquidity
 */
max_allowed_slippage_bps: i64;
  /**
 * The maximum amount of spread (in bps) that is tolerated during swap
 */
max_allowed_spread_bps: i64;
  /**
 * The maximum allowed percentage (in bps) for referral fee
 */
max_referral_bps: i64;
  pool_type: PairType;
  share_token: string;
  stake_contract: string;
  token_a: string;
  token_b: string;
  /**
 * The total fees (in bps) charged by a pool of this type.
 * In relation to the returned amount of tokens
 */
total_fee_bps: i64;
}


export interface Asset {
  /**
 * Address of the asset
 */
address: string;
  /**
 * The total amount of those tokens in the pool
 */
amount: i128;
}


export interface ComputeSwap {
  /**
 * The commision amount is the fee that is charged by the pool for the swap service.
 */
commission_amount: i128;
  /**
 * The referral fee is the fee that will be given back to the referral. `0` if no referral is
 * set.
 */
referral_fee_amount: i128;
  /**
 * The amount that will be returned to the user, after all fees and spread has been taken into
 * account.
 */
return_amount: i128;
  /**
 * The spread amount, that is the difference between expected and actual swap amount.
 */
spread_amount: i128;
}


export interface Referral {
  /**
 * Address of the referral
 */
address: string;
  /**
 * fee in bps, later parsed to percentage
 */
fee: i64;
}


/**
 * This struct is used to return a query result with the total amount of LP tokens and assets in a specific pool.
 */
export interface PoolResponse {
  /**
 * The asset A in the pool together with asset amounts
 */
asset_a: Asset;
  /**
 * The asset B in the pool together with asset amounts
 */
asset_b: Asset;
  /**
 * The total amount of LP tokens currently issued
 */
asset_lp_share: Asset;
  /**
 * The address of the Stake contract for the liquidity pool
 */
stake_address: string;
}


export interface LiquidityPoolInfo {
  pool_address: string;
  pool_response: PoolResponse;
  total_fee_bps: i64;
}


export interface SimulateSwapResponse {
  ask_amount: i128;
  commission_amount: i128;
  spread_amount: i128;
  total_return: i128;
}


export interface SimulateReverseSwapResponse {
  commission_amount: i128;
  offer_amount: i128;
  spread_amount: i128;
}


export interface TokenInitInfo {
  token_a: string;
  token_b: string;
}


export interface StakeInitInfo {
  manager: string;
  max_complexity: u32;
  min_bond: i128;
  min_reward: i128;
}


export interface LiquidityPoolInitInfo {
  admin: string;
  fee_recipient: string;
  max_allowed_slippage_bps: i64;
  max_allowed_spread_bps: i64;
  max_referral_bps: i64;
  stake_init_info: StakeInitInfo;
  swap_fee_bps: i64;
  token_init_info: TokenInitInfo;
}


export interface Client {
  /**
   * Construct and simulate a initialize transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  initialize: ({stake_wasm_hash, token_wasm_hash, lp_init_info, factory_addr, share_token_decimals, share_token_name, share_token_symbol}: {stake_wasm_hash: Buffer, token_wasm_hash: Buffer, lp_init_info: LiquidityPoolInitInfo, factory_addr: string, share_token_decimals: u32, share_token_name: string, share_token_symbol: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<null>>

  /**
   * Construct and simulate a provide_liquidity transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  provide_liquidity: ({sender, desired_a, min_a, desired_b, min_b, custom_slippage_bps}: {sender: string, desired_a: Option<i128>, min_a: Option<i128>, desired_b: Option<i128>, min_b: Option<i128>, custom_slippage_bps: Option<i64>}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<null>>

  /**
   * Construct and simulate a swap transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  swap: ({sender, offer_asset, offer_amount, ask_asset_min_amount, max_spread_bps}: {sender: string, offer_asset: string, offer_amount: i128, ask_asset_min_amount: Option<i128>, max_spread_bps: Option<i64>}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<i128>>

  /**
   * Construct and simulate a withdraw_liquidity transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  withdraw_liquidity: ({sender, share_amount, min_a, min_b}: {sender: string, share_amount: i128, min_a: i128, min_b: i128}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<readonly [i128, i128]>>

  /**
   * Construct and simulate a update_config transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  update_config: ({new_admin, total_fee_bps, fee_recipient, max_allowed_slippage_bps, max_allowed_spread_bps, max_referral_bps}: {new_admin: Option<string>, total_fee_bps: Option<i64>, fee_recipient: Option<string>, max_allowed_slippage_bps: Option<i64>, max_allowed_spread_bps: Option<i64>, max_referral_bps: Option<i64>}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<null>>

  /**
   * Construct and simulate a upgrade transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  upgrade: ({new_wasm_hash}: {new_wasm_hash: Buffer}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<null>>

  /**
   * Construct and simulate a query_config transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  query_config: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Config>>

  /**
   * Construct and simulate a query_share_token_address transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  query_share_token_address: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<string>>

  /**
   * Construct and simulate a query_stake_contract_address transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  query_stake_contract_address: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<string>>

  /**
   * Construct and simulate a query_pool_info transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  query_pool_info: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<PoolResponse>>

  /**
   * Construct and simulate a query_pool_info_for_factory transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  query_pool_info_for_factory: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<LiquidityPoolInfo>>

  /**
   * Construct and simulate a simulate_swap transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  simulate_swap: ({offer_asset, offer_amount}: {offer_asset: string, offer_amount: i128}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<SimulateSwapResponse>>

  /**
   * Construct and simulate a simulate_reverse_swap transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  simulate_reverse_swap: ({ask_asset, ask_amount}: {ask_asset: string, ask_amount: i128}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<SimulateReverseSwapResponse>>

  /**
   * Construct and simulate a query_share transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  query_share: ({amount}: {amount: i128}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<readonly [Asset, Asset]>>

  /**
   * Construct and simulate a update transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  update: ({new_wasm_hash}: {new_wasm_hash: Buffer}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<null>>

}
export class Client extends ContractClient {
  constructor(public readonly options: ContractClientOptions) {
    super(
      new ContractSpec([ "AAAAAAAAAAAAAAAKaW5pdGlhbGl6ZQAAAAAABwAAAAAAAAAPc3Rha2Vfd2FzbV9oYXNoAAAAA+4AAAAgAAAAAAAAAA90b2tlbl93YXNtX2hhc2gAAAAD7gAAACAAAAAAAAAADGxwX2luaXRfaW5mbwAAB9AAAAAVTGlxdWlkaXR5UG9vbEluaXRJbmZvAAAAAAAAAAAAAAxmYWN0b3J5X2FkZHIAAAATAAAAAAAAABRzaGFyZV90b2tlbl9kZWNpbWFscwAAAAQAAAAAAAAAEHNoYXJlX3Rva2VuX25hbWUAAAAQAAAAAAAAABJzaGFyZV90b2tlbl9zeW1ib2wAAAAAABAAAAAA",
        "AAAAAAAAAAAAAAARcHJvdmlkZV9saXF1aWRpdHkAAAAAAAAGAAAAAAAAAAZzZW5kZXIAAAAAABMAAAAAAAAACWRlc2lyZWRfYQAAAAAAA+gAAAALAAAAAAAAAAVtaW5fYQAAAAAAA+gAAAALAAAAAAAAAAlkZXNpcmVkX2IAAAAAAAPoAAAACwAAAAAAAAAFbWluX2IAAAAAAAPoAAAACwAAAAAAAAATY3VzdG9tX3NsaXBwYWdlX2JwcwAAAAPoAAAABwAAAAA=",
        "AAAAAAAAAAAAAAAEc3dhcAAAAAUAAAAAAAAABnNlbmRlcgAAAAAAEwAAAAAAAAALb2ZmZXJfYXNzZXQAAAAAEwAAAAAAAAAMb2ZmZXJfYW1vdW50AAAACwAAAAAAAAAUYXNrX2Fzc2V0X21pbl9hbW91bnQAAAPoAAAACwAAAAAAAAAObWF4X3NwcmVhZF9icHMAAAAAA+gAAAAHAAAAAQAAAAs=",
        "AAAAAAAAAAAAAAASd2l0aGRyYXdfbGlxdWlkaXR5AAAAAAAEAAAAAAAAAAZzZW5kZXIAAAAAABMAAAAAAAAADHNoYXJlX2Ftb3VudAAAAAsAAAAAAAAABW1pbl9hAAAAAAAACwAAAAAAAAAFbWluX2IAAAAAAAALAAAAAQAAA+0AAAACAAAACwAAAAs=",
        "AAAAAAAAAAAAAAANdXBkYXRlX2NvbmZpZwAAAAAAAAYAAAAAAAAACW5ld19hZG1pbgAAAAAAA+gAAAATAAAAAAAAAA10b3RhbF9mZWVfYnBzAAAAAAAD6AAAAAcAAAAAAAAADWZlZV9yZWNpcGllbnQAAAAAAAPoAAAAEwAAAAAAAAAYbWF4X2FsbG93ZWRfc2xpcHBhZ2VfYnBzAAAD6AAAAAcAAAAAAAAAFm1heF9hbGxvd2VkX3NwcmVhZF9icHMAAAAAA+gAAAAHAAAAAAAAABBtYXhfcmVmZXJyYWxfYnBzAAAD6AAAAAcAAAAA",
        "AAAAAAAAAAAAAAAHdXBncmFkZQAAAAABAAAAAAAAAA1uZXdfd2FzbV9oYXNoAAAAAAAD7gAAACAAAAAA",
        "AAAAAAAAAAAAAAAMcXVlcnlfY29uZmlnAAAAAAAAAAEAAAfQAAAABkNvbmZpZwAA",
        "AAAAAAAAAAAAAAAZcXVlcnlfc2hhcmVfdG9rZW5fYWRkcmVzcwAAAAAAAAAAAAABAAAAEw==",
        "AAAAAAAAAAAAAAAccXVlcnlfc3Rha2VfY29udHJhY3RfYWRkcmVzcwAAAAAAAAABAAAAEw==",
        "AAAAAAAAAAAAAAAPcXVlcnlfcG9vbF9pbmZvAAAAAAAAAAABAAAH0AAAAAxQb29sUmVzcG9uc2U=",
        "AAAAAAAAAAAAAAAbcXVlcnlfcG9vbF9pbmZvX2Zvcl9mYWN0b3J5AAAAAAAAAAABAAAH0AAAABFMaXF1aWRpdHlQb29sSW5mbwAAAA==",
        "AAAAAAAAAAAAAAANc2ltdWxhdGVfc3dhcAAAAAAAAAIAAAAAAAAAC29mZmVyX2Fzc2V0AAAAABMAAAAAAAAADG9mZmVyX2Ftb3VudAAAAAsAAAABAAAH0AAAABRTaW11bGF0ZVN3YXBSZXNwb25zZQ==",
        "AAAAAAAAAAAAAAAVc2ltdWxhdGVfcmV2ZXJzZV9zd2FwAAAAAAAAAgAAAAAAAAAJYXNrX2Fzc2V0AAAAAAAAEwAAAAAAAAAKYXNrX2Ftb3VudAAAAAAACwAAAAEAAAfQAAAAG1NpbXVsYXRlUmV2ZXJzZVN3YXBSZXNwb25zZQA=",
        "AAAAAAAAAAAAAAALcXVlcnlfc2hhcmUAAAAAAQAAAAAAAAAGYW1vdW50AAAAAAALAAAAAQAAA+0AAAACAAAH0AAAAAVBc3NldAAAAAAAB9AAAAAFQXNzZXQAAAA=",
        "AAAAAAAAAAAAAAAGdXBkYXRlAAAAAAABAAAAAAAAAA1uZXdfd2FzbV9oYXNoAAAAAAAD7gAAACAAAAAA",
        "AAAABAAAAAAAAAAAAAAADUNvbnRyYWN0RXJyb3IAAAAAAAAVAAAAAAAAABJTcHJlYWRFeGNlZWRzTGltaXQAAAAAAAEAAAAAAAAAKFByb3ZpZGVMaXF1aWRpdHlTbGlwcGFnZVRvbGVyYW5jZVRvb0hpZ2gAAAACAAAAAAAAADNQcm92aWRlTGlxdWlkaXR5QXRMZWFzdE9uZVRva2VuTXVzdEJlQmlnZ2VyVGhlblplcm8AAAAAAwAAAAAAAAAyV2l0aGRyYXdMaXF1aWRpdHlNaW5pbXVtQW1vdW50T2ZBT3JCSXNOb3RTYXRpc2ZpZWQAAAAAAAQAAAAAAAAALVNwbGl0RGVwb3NpdEJvdGhQb29sc0FuZERlcG9zaXRNdXN0QmVQb3NpdGl2ZQAAAAAAAAUAAAAAAAAAK1ZhbGlkYXRlRmVlQnBzVG90YWxGZWVzQ2FudEJlR3JlYXRlclRoYW4xMDAAAAAABgAAAAAAAAAnR2V0RGVwb3NpdEFtb3VudHNNaW5BQmlnZ2VyVGhlbkRlc2lyZWRBAAAAAAcAAAAAAAAAJ0dldERlcG9zaXRBbW91bnRzTWluQkJpZ2dlclRoZW5EZXNpcmVkQgAAAAAIAAAAAAAAACpHZXREZXBvc2l0QW1vdW50c0Ftb3VudEFCaWdnZXJUaGVuRGVzaXJlZEEAAAAAAAkAAAAAAAAAJEdldERlcG9zaXRBbW91bnRzQW1vdW50QUxlc3NUaGVuTWluQQAAAAoAAAAAAAAAKkdldERlcG9zaXRBbW91bnRzQW1vdW50QkJpZ2dlclRoZW5EZXNpcmVkQgAAAAAACwAAAAAAAAAkR2V0RGVwb3NpdEFtb3VudHNBbW91bnRCTGVzc1RoZW5NaW5CAAAADAAAAAAAAAAUVG90YWxTaGFyZXNFcXVhbFplcm8AAAANAAAAAAAAAB5EZXNpcmVkQW1vdW50c0JlbG93T3JFcXVhbFplcm8AAAAAAA4AAAAAAAAAE01pbkFtb3VudHNCZWxvd1plcm8AAAAADwAAAAAAAAAOQXNzZXROb3RJblBvb2wAAAAAABAAAAAAAAAAEkFscmVhZHlJbml0aWFsaXplZAAAAAAAEQAAAAAAAAAWVG9rZW5BQmlnZ2VyVGhhblRva2VuQgAAAAAAEgAAAAAAAAAKSW52YWxpZEJwcwAAAAAAEwAAAAAAAAAPU2xpcHBhZ2VJbnZhbGlkAAAAABQAAAAAAAAAH1N3YXBNaW5SZWNlaXZlZEJpZ2dlclRoYW5SZXR1cm4AAAAAFQ==",
        "AAAAAwAAAAAAAAAAAAAACFBhaXJUeXBlAAAAAQAAAAAAAAADWHlrAAAAAAA=",
        "AAAAAQAAAAAAAAAAAAAABkNvbmZpZwAAAAAACgAAAAAAAAANZmVlX3JlY2lwaWVudAAAAAAAABMAAABUVGhlIG1heGltdW0gYW1vdW50IG9mIHNsaXBwYWdlIChpbiBicHMpIHRoYXQgaXMgdG9sZXJhdGVkIGR1cmluZyBwcm92aWRpbmcgbGlxdWlkaXR5AAAAGG1heF9hbGxvd2VkX3NsaXBwYWdlX2JwcwAAAAcAAABDVGhlIG1heGltdW0gYW1vdW50IG9mIHNwcmVhZCAoaW4gYnBzKSB0aGF0IGlzIHRvbGVyYXRlZCBkdXJpbmcgc3dhcAAAAAAWbWF4X2FsbG93ZWRfc3ByZWFkX2JwcwAAAAAABwAAADhUaGUgbWF4aW11bSBhbGxvd2VkIHBlcmNlbnRhZ2UgKGluIGJwcykgZm9yIHJlZmVycmFsIGZlZQAAABBtYXhfcmVmZXJyYWxfYnBzAAAABwAAAAAAAAAJcG9vbF90eXBlAAAAAAAH0AAAAAhQYWlyVHlwZQAAAAAAAAALc2hhcmVfdG9rZW4AAAAAEwAAAAAAAAAOc3Rha2VfY29udHJhY3QAAAAAABMAAAAAAAAAB3Rva2VuX2EAAAAAEwAAAAAAAAAHdG9rZW5fYgAAAAATAAAAZFRoZSB0b3RhbCBmZWVzIChpbiBicHMpIGNoYXJnZWQgYnkgYSBwb29sIG9mIHRoaXMgdHlwZS4KSW4gcmVsYXRpb24gdG8gdGhlIHJldHVybmVkIGFtb3VudCBvZiB0b2tlbnMAAAANdG90YWxfZmVlX2JwcwAAAAAAAAc=",
        "AAAAAQAAAAAAAAAAAAAABUFzc2V0AAAAAAAAAgAAABRBZGRyZXNzIG9mIHRoZSBhc3NldAAAAAdhZGRyZXNzAAAAABMAAAAsVGhlIHRvdGFsIGFtb3VudCBvZiB0aG9zZSB0b2tlbnMgaW4gdGhlIHBvb2wAAAAGYW1vdW50AAAAAAAL",
        "AAAAAQAAAAAAAAAAAAAAC0NvbXB1dGVTd2FwAAAAAAQAAABRVGhlIGNvbW1pc2lvbiBhbW91bnQgaXMgdGhlIGZlZSB0aGF0IGlzIGNoYXJnZWQgYnkgdGhlIHBvb2wgZm9yIHRoZSBzd2FwIHNlcnZpY2UuAAAAAAAAEWNvbW1pc3Npb25fYW1vdW50AAAAAAAACwAAAF9UaGUgcmVmZXJyYWwgZmVlIGlzIHRoZSBmZWUgdGhhdCB3aWxsIGJlIGdpdmVuIGJhY2sgdG8gdGhlIHJlZmVycmFsLiBgMGAgaWYgbm8gcmVmZXJyYWwgaXMKc2V0LgAAAAATcmVmZXJyYWxfZmVlX2Ftb3VudAAAAAALAAAAZFRoZSBhbW91bnQgdGhhdCB3aWxsIGJlIHJldHVybmVkIHRvIHRoZSB1c2VyLCBhZnRlciBhbGwgZmVlcyBhbmQgc3ByZWFkIGhhcyBiZWVuIHRha2VuIGludG8KYWNjb3VudC4AAAANcmV0dXJuX2Ftb3VudAAAAAAAAAsAAABSVGhlIHNwcmVhZCBhbW91bnQsIHRoYXQgaXMgdGhlIGRpZmZlcmVuY2UgYmV0d2VlbiBleHBlY3RlZCBhbmQgYWN0dWFsIHN3YXAgYW1vdW50LgAAAAAADXNwcmVhZF9hbW91bnQAAAAAAAAL",
        "AAAAAQAAAAAAAAAAAAAACFJlZmVycmFsAAAAAgAAABdBZGRyZXNzIG9mIHRoZSByZWZlcnJhbAAAAAAHYWRkcmVzcwAAAAATAAAAJmZlZSBpbiBicHMsIGxhdGVyIHBhcnNlZCB0byBwZXJjZW50YWdlAAAAAAADZmVlAAAAAAc=",
        "AAAAAQAAAG5UaGlzIHN0cnVjdCBpcyB1c2VkIHRvIHJldHVybiBhIHF1ZXJ5IHJlc3VsdCB3aXRoIHRoZSB0b3RhbCBhbW91bnQgb2YgTFAgdG9rZW5zIGFuZCBhc3NldHMgaW4gYSBzcGVjaWZpYyBwb29sLgAAAAAAAAAAAAxQb29sUmVzcG9uc2UAAAAEAAAAM1RoZSBhc3NldCBBIGluIHRoZSBwb29sIHRvZ2V0aGVyIHdpdGggYXNzZXQgYW1vdW50cwAAAAAHYXNzZXRfYQAAAAfQAAAABUFzc2V0AAAAAAAAM1RoZSBhc3NldCBCIGluIHRoZSBwb29sIHRvZ2V0aGVyIHdpdGggYXNzZXQgYW1vdW50cwAAAAAHYXNzZXRfYgAAAAfQAAAABUFzc2V0AAAAAAAALlRoZSB0b3RhbCBhbW91bnQgb2YgTFAgdG9rZW5zIGN1cnJlbnRseSBpc3N1ZWQAAAAAAA5hc3NldF9scF9zaGFyZQAAAAAH0AAAAAVBc3NldAAAAAAAADhUaGUgYWRkcmVzcyBvZiB0aGUgU3Rha2UgY29udHJhY3QgZm9yIHRoZSBsaXF1aWRpdHkgcG9vbAAAAA1zdGFrZV9hZGRyZXNzAAAAAAAAEw==",
        "AAAAAQAAAAAAAAAAAAAAEUxpcXVpZGl0eVBvb2xJbmZvAAAAAAAAAwAAAAAAAAAMcG9vbF9hZGRyZXNzAAAAEwAAAAAAAAANcG9vbF9yZXNwb25zZQAAAAAAB9AAAAAMUG9vbFJlc3BvbnNlAAAAAAAAAA10b3RhbF9mZWVfYnBzAAAAAAAABw==",
        "AAAAAQAAAAAAAAAAAAAAFFNpbXVsYXRlU3dhcFJlc3BvbnNlAAAABAAAAAAAAAAKYXNrX2Ftb3VudAAAAAAACwAAAAAAAAARY29tbWlzc2lvbl9hbW91bnQAAAAAAAALAAAAAAAAAA1zcHJlYWRfYW1vdW50AAAAAAAACwAAAAAAAAAMdG90YWxfcmV0dXJuAAAACw==",
        "AAAAAQAAAAAAAAAAAAAAG1NpbXVsYXRlUmV2ZXJzZVN3YXBSZXNwb25zZQAAAAADAAAAAAAAABFjb21taXNzaW9uX2Ftb3VudAAAAAAAAAsAAAAAAAAADG9mZmVyX2Ftb3VudAAAAAsAAAAAAAAADXNwcmVhZF9hbW91bnQAAAAAAAAL",
        "AAAAAQAAAAAAAAAAAAAADVRva2VuSW5pdEluZm8AAAAAAAACAAAAAAAAAAd0b2tlbl9hAAAAABMAAAAAAAAAB3Rva2VuX2IAAAAAEw==",
        "AAAAAQAAAAAAAAAAAAAADVN0YWtlSW5pdEluZm8AAAAAAAAEAAAAAAAAAAdtYW5hZ2VyAAAAABMAAAAAAAAADm1heF9jb21wbGV4aXR5AAAAAAAEAAAAAAAAAAhtaW5fYm9uZAAAAAsAAAAAAAAACm1pbl9yZXdhcmQAAAAAAAs=",
        "AAAAAQAAAAAAAAAAAAAAFUxpcXVpZGl0eVBvb2xJbml0SW5mbwAAAAAAAAgAAAAAAAAABWFkbWluAAAAAAAAEwAAAAAAAAANZmVlX3JlY2lwaWVudAAAAAAAABMAAAAAAAAAGG1heF9hbGxvd2VkX3NsaXBwYWdlX2JwcwAAAAcAAAAAAAAAFm1heF9hbGxvd2VkX3NwcmVhZF9icHMAAAAAAAcAAAAAAAAAEG1heF9yZWZlcnJhbF9icHMAAAAHAAAAAAAAAA9zdGFrZV9pbml0X2luZm8AAAAH0AAAAA1TdGFrZUluaXRJbmZvAAAAAAAAAAAAAAxzd2FwX2ZlZV9icHMAAAAHAAAAAAAAAA90b2tlbl9pbml0X2luZm8AAAAH0AAAAA1Ub2tlbkluaXRJbmZvAAAA" ]),
      options
    )
  }
  public readonly fromJSON = {
    initialize: this.txFromJSON<null>,
        provide_liquidity: this.txFromJSON<null>,
        swap: this.txFromJSON<i128>,
        withdraw_liquidity: this.txFromJSON<readonly [i128, i128]>,
        update_config: this.txFromJSON<null>,
        upgrade: this.txFromJSON<null>,
        query_config: this.txFromJSON<Config>,
        query_share_token_address: this.txFromJSON<string>,
        query_stake_contract_address: this.txFromJSON<string>,
        query_pool_info: this.txFromJSON<PoolResponse>,
        query_pool_info_for_factory: this.txFromJSON<LiquidityPoolInfo>,
        simulate_swap: this.txFromJSON<SimulateSwapResponse>,
        simulate_reverse_swap: this.txFromJSON<SimulateReverseSwapResponse>,
        query_share: this.txFromJSON<readonly [Asset, Asset]>,
        update: this.txFromJSON<null>
  }
}