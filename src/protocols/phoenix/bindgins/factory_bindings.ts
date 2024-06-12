import type {
  i128,
  i64,
  Option,
  u32,
  u64,
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
    contractId: "CDZSRZNAR3M33JXPH4M2LX3FLD2SKABAYCPKA3K5D4QY542KCOI3GPLA",
  }
} as const

export const Errors = {
  1: { message: "" },
  2: { message: "" },
  3: { message: "" },
  4: { message: "" },
  5: { message: "" },
  6: { message: "" },
  7: { message: "" },
};

export interface PairTupleKey {
  token_a: string;
  token_b: string;
}

export interface Config {
  admin: string;
  lp_token_decimals: u32;
  lp_wasm_hash: Buffer;
  multihop_address: string;
  stake_wasm_hash: Buffer;
  token_wasm_hash: Buffer;
  whitelisted_accounts: Array<string>;
}

export interface UserPortfolio {
  lp_portfolio: Array<LpPortfolio>;
  stake_portfolio: Array<StakePortfolio>;
}

export interface LpPortfolio {
  assets: readonly [Asset, Asset];
}

export interface StakePortfolio {
  stakes: Array<Stake>;
  staking_contract: string;
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

export interface StakedResponse {
  stakes: Array<Stake>;
}

export interface Stake {
  /**
   * The amount of staked tokens
   */
  stake: i128;
  /**
   * The timestamp when the stake was made
   */
  stake_timestamp: u64;
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
  initialize: (
    {
      admin,
      multihop_wasm_hash,
      lp_wasm_hash,
      stake_wasm_hash,
      token_wasm_hash,
      whitelisted_accounts,
      lp_token_decimals,
    }: {
      admin: string;
      multihop_wasm_hash: Buffer;
      lp_wasm_hash: Buffer;
      stake_wasm_hash: Buffer;
      token_wasm_hash: Buffer;
      whitelisted_accounts: Array<string>;
      lp_token_decimals: u32;
    },
    options?: {
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
    }
  ) => Promise<AssembledTransaction<null>>;

  /**
   * Construct and simulate a create_liquidity_pool transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  create_liquidity_pool: (
    {
      sender,
      lp_init_info,
      share_token_name,
      share_token_symbol,
    }: {
      sender: string;
      lp_init_info: LiquidityPoolInitInfo;
      share_token_name: string;
      share_token_symbol: string;
    },
    options?: {
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
    }
  ) => Promise<AssembledTransaction<string>>;

  /**
   * Construct and simulate a update_whitelisted_accounts transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  update_whitelisted_accounts: (
    {
      sender,
      to_add,
      to_remove,
    }: { sender: string; to_add: Array<string>; to_remove: Array<string> },
    options?: {
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
    }
  ) => Promise<AssembledTransaction<null>>;

  /**
   * Construct and simulate a update_wasm_hashes transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  update_wasm_hashes: (
    {
      lp_wasm_hash,
      stake_wasm_hash,
      token_wasm_hash,
    }: {
      lp_wasm_hash: Option<Buffer>;
      stake_wasm_hash: Option<Buffer>;
      token_wasm_hash: Option<Buffer>;
    },
    options?: {
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
    }
  ) => Promise<AssembledTransaction<null>>;

  /**
   * Construct and simulate a query_pools transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  query_pools: (options?: {
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
  }) => Promise<AssembledTransaction<Array<string>>>;

  /**
   * Construct and simulate a query_pool_details transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  query_pool_details: (
    { pool_address }: { pool_address: string },
    options?: {
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
    }
  ) => Promise<AssembledTransaction<LiquidityPoolInfo>>;

  /**
   * Construct and simulate a query_all_pools_details transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  query_all_pools_details: (options?: {
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
  }) => Promise<AssembledTransaction<Array<LiquidityPoolInfo>>>;

  /**
   * Construct and simulate a query_for_pool_by_token_pair transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  query_for_pool_by_token_pair: (
    { token_a, token_b }: { token_a: string; token_b: string },
    options?: {
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
    }
  ) => Promise<AssembledTransaction<string>>;

  /**
   * Construct and simulate a get_admin transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_admin: (options?: {
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
  }) => Promise<AssembledTransaction<string>>;

  /**
   * Construct and simulate a get_config transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_config: (options?: {
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
  }) => Promise<AssembledTransaction<Config>>;

  /**
   * Construct and simulate a query_user_portfolio transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  query_user_portfolio: (
    { sender, staking }: { sender: string; staking: boolean },
    options?: {
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
    }
  ) => Promise<AssembledTransaction<UserPortfolio>>;

  /**
   * Construct and simulate a update transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  update: (
    { new_wasm_hash }: { new_wasm_hash: Buffer },
    options?: {
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
    }
  ) => Promise<AssembledTransaction<null>>;
}
export class Client extends ContractClient {
  constructor(public readonly options: ContractClientOptions) {
    super(
      new ContractSpec([
        "AAAAAAAAAAAAAAAKaW5pdGlhbGl6ZQAAAAAABwAAAAAAAAAFYWRtaW4AAAAAAAATAAAAAAAAABJtdWx0aWhvcF93YXNtX2hhc2gAAAAAA+4AAAAgAAAAAAAAAAxscF93YXNtX2hhc2gAAAPuAAAAIAAAAAAAAAAPc3Rha2Vfd2FzbV9oYXNoAAAAA+4AAAAgAAAAAAAAAA90b2tlbl93YXNtX2hhc2gAAAAD7gAAACAAAAAAAAAAFHdoaXRlbGlzdGVkX2FjY291bnRzAAAD6gAAABMAAAAAAAAAEWxwX3Rva2VuX2RlY2ltYWxzAAAAAAAABAAAAAA=",
        "AAAAAAAAAAAAAAAVY3JlYXRlX2xpcXVpZGl0eV9wb29sAAAAAAAABAAAAAAAAAAGc2VuZGVyAAAAAAATAAAAAAAAAAxscF9pbml0X2luZm8AAAfQAAAAFUxpcXVpZGl0eVBvb2xJbml0SW5mbwAAAAAAAAAAAAAQc2hhcmVfdG9rZW5fbmFtZQAAABAAAAAAAAAAEnNoYXJlX3Rva2VuX3N5bWJvbAAAAAAAEAAAAAEAAAAT",
        "AAAAAAAAAAAAAAAbdXBkYXRlX3doaXRlbGlzdGVkX2FjY291bnRzAAAAAAMAAAAAAAAABnNlbmRlcgAAAAAAEwAAAAAAAAAGdG9fYWRkAAAAAAPqAAAAEwAAAAAAAAAJdG9fcmVtb3ZlAAAAAAAD6gAAABMAAAAA",
        "AAAAAAAAAAAAAAASdXBkYXRlX3dhc21faGFzaGVzAAAAAAADAAAAAAAAAAxscF93YXNtX2hhc2gAAAPoAAAD7gAAACAAAAAAAAAAD3N0YWtlX3dhc21faGFzaAAAAAPoAAAD7gAAACAAAAAAAAAAD3Rva2VuX3dhc21faGFzaAAAAAPoAAAD7gAAACAAAAAA",
        "AAAAAAAAAAAAAAALcXVlcnlfcG9vbHMAAAAAAAAAAAEAAAPqAAAAEw==",
        "AAAAAAAAAAAAAAAScXVlcnlfcG9vbF9kZXRhaWxzAAAAAAABAAAAAAAAAAxwb29sX2FkZHJlc3MAAAATAAAAAQAAB9AAAAARTGlxdWlkaXR5UG9vbEluZm8AAAA=",
        "AAAAAAAAAAAAAAAXcXVlcnlfYWxsX3Bvb2xzX2RldGFpbHMAAAAAAAAAAAEAAAPqAAAH0AAAABFMaXF1aWRpdHlQb29sSW5mbwAAAA==",
        "AAAAAAAAAAAAAAAccXVlcnlfZm9yX3Bvb2xfYnlfdG9rZW5fcGFpcgAAAAIAAAAAAAAAB3Rva2VuX2EAAAAAEwAAAAAAAAAHdG9rZW5fYgAAAAATAAAAAQAAABM=",
        "AAAAAAAAAAAAAAAJZ2V0X2FkbWluAAAAAAAAAAAAAAEAAAAT",
        "AAAAAAAAAAAAAAAKZ2V0X2NvbmZpZwAAAAAAAAAAAAEAAAfQAAAABkNvbmZpZwAA",
        "AAAAAAAAAAAAAAAUcXVlcnlfdXNlcl9wb3J0Zm9saW8AAAACAAAAAAAAAAZzZW5kZXIAAAAAABMAAAAAAAAAB3N0YWtpbmcAAAAAAQAAAAEAAAfQAAAADVVzZXJQb3J0Zm9saW8AAAA=",
        "AAAAAAAAAAAAAAAGdXBkYXRlAAAAAAABAAAAAAAAAA1uZXdfd2FzbV9oYXNoAAAAAAAD7gAAACAAAAAA",
        "AAAABAAAAAAAAAAAAAAADUNvbnRyYWN0RXJyb3IAAAAAAAAHAAAAAAAAABJBbHJlYWR5SW5pdGlhbGl6ZWQAAAAAAAEAAAAAAAAAD1doaXRlTGlzdGVFbXB0eQAAAAACAAAAAAAAAA1Ob3RBdXRob3JpemVkAAAAAAAAAwAAAAAAAAAVTGlxdWlkaXR5UG9vbE5vdEZvdW5kAAAAAAAABAAAAAAAAAAWVG9rZW5BQmlnZ2VyVGhhblRva2VuQgAAAAAABQAAAAAAAAAPTWluU3Rha2VJbnZhbGlkAAAAAAYAAAAAAAAAEE1pblJld2FyZEludmFsaWQAAAAH",
        "AAAAAQAAAAAAAAAAAAAADFBhaXJUdXBsZUtleQAAAAIAAAAAAAAAB3Rva2VuX2EAAAAAEwAAAAAAAAAHdG9rZW5fYgAAAAAT",
        "AAAAAQAAAAAAAAAAAAAABkNvbmZpZwAAAAAABwAAAAAAAAAFYWRtaW4AAAAAAAATAAAAAAAAABFscF90b2tlbl9kZWNpbWFscwAAAAAAAAQAAAAAAAAADGxwX3dhc21faGFzaAAAA+4AAAAgAAAAAAAAABBtdWx0aWhvcF9hZGRyZXNzAAAAEwAAAAAAAAAPc3Rha2Vfd2FzbV9oYXNoAAAAA+4AAAAgAAAAAAAAAA90b2tlbl93YXNtX2hhc2gAAAAD7gAAACAAAAAAAAAAFHdoaXRlbGlzdGVkX2FjY291bnRzAAAD6gAAABM=",
        "AAAAAQAAAAAAAAAAAAAADVVzZXJQb3J0Zm9saW8AAAAAAAACAAAAAAAAAAxscF9wb3J0Zm9saW8AAAPqAAAH0AAAAAtMcFBvcnRmb2xpbwAAAAAAAAAAD3N0YWtlX3BvcnRmb2xpbwAAAAPqAAAH0AAAAA5TdGFrZVBvcnRmb2xpbwAA",
        "AAAAAQAAAAAAAAAAAAAAC0xwUG9ydGZvbGlvAAAAAAEAAAAAAAAABmFzc2V0cwAAAAAD7QAAAAIAAAfQAAAABUFzc2V0AAAAAAAH0AAAAAVBc3NldAAAAA==",
        "AAAAAQAAAAAAAAAAAAAADlN0YWtlUG9ydGZvbGlvAAAAAAACAAAAAAAAAAZzdGFrZXMAAAAAA+oAAAfQAAAABVN0YWtlAAAAAAAAAAAAABBzdGFraW5nX2NvbnRyYWN0AAAAEw==",
        "AAAAAQAAAAAAAAAAAAAABUFzc2V0AAAAAAAAAgAAABRBZGRyZXNzIG9mIHRoZSBhc3NldAAAAAdhZGRyZXNzAAAAABMAAAAsVGhlIHRvdGFsIGFtb3VudCBvZiB0aG9zZSB0b2tlbnMgaW4gdGhlIHBvb2wAAAAGYW1vdW50AAAAAAAL",
        "AAAAAQAAAG5UaGlzIHN0cnVjdCBpcyB1c2VkIHRvIHJldHVybiBhIHF1ZXJ5IHJlc3VsdCB3aXRoIHRoZSB0b3RhbCBhbW91bnQgb2YgTFAgdG9rZW5zIGFuZCBhc3NldHMgaW4gYSBzcGVjaWZpYyBwb29sLgAAAAAAAAAAAAxQb29sUmVzcG9uc2UAAAAEAAAAM1RoZSBhc3NldCBBIGluIHRoZSBwb29sIHRvZ2V0aGVyIHdpdGggYXNzZXQgYW1vdW50cwAAAAAHYXNzZXRfYQAAAAfQAAAABUFzc2V0AAAAAAAAM1RoZSBhc3NldCBCIGluIHRoZSBwb29sIHRvZ2V0aGVyIHdpdGggYXNzZXQgYW1vdW50cwAAAAAHYXNzZXRfYgAAAAfQAAAABUFzc2V0AAAAAAAALlRoZSB0b3RhbCBhbW91bnQgb2YgTFAgdG9rZW5zIGN1cnJlbnRseSBpc3N1ZWQAAAAAAA5hc3NldF9scF9zaGFyZQAAAAAH0AAAAAVBc3NldAAAAAAAADhUaGUgYWRkcmVzcyBvZiB0aGUgU3Rha2UgY29udHJhY3QgZm9yIHRoZSBsaXF1aWRpdHkgcG9vbAAAAA1zdGFrZV9hZGRyZXNzAAAAAAAAEw==",
        "AAAAAQAAAAAAAAAAAAAAEUxpcXVpZGl0eVBvb2xJbmZvAAAAAAAAAwAAAAAAAAAMcG9vbF9hZGRyZXNzAAAAEwAAAAAAAAANcG9vbF9yZXNwb25zZQAAAAAAB9AAAAAMUG9vbFJlc3BvbnNlAAAAAAAAAA10b3RhbF9mZWVfYnBzAAAAAAAABw==",
        "AAAAAQAAAAAAAAAAAAAADlN0YWtlZFJlc3BvbnNlAAAAAAABAAAAAAAAAAZzdGFrZXMAAAAAA+oAAAfQAAAABVN0YWtlAAAA",
        "AAAAAQAAAAAAAAAAAAAABVN0YWtlAAAAAAAAAgAAABtUaGUgYW1vdW50IG9mIHN0YWtlZCB0b2tlbnMAAAAABXN0YWtlAAAAAAAACwAAACVUaGUgdGltZXN0YW1wIHdoZW4gdGhlIHN0YWtlIHdhcyBtYWRlAAAAAAAAD3N0YWtlX3RpbWVzdGFtcAAAAAAG",
        "AAAAAQAAAAAAAAAAAAAADVRva2VuSW5pdEluZm8AAAAAAAACAAAAAAAAAAd0b2tlbl9hAAAAABMAAAAAAAAAB3Rva2VuX2IAAAAAEw==",
        "AAAAAQAAAAAAAAAAAAAADVN0YWtlSW5pdEluZm8AAAAAAAAEAAAAAAAAAAdtYW5hZ2VyAAAAABMAAAAAAAAADm1heF9jb21wbGV4aXR5AAAAAAAEAAAAAAAAAAhtaW5fYm9uZAAAAAsAAAAAAAAACm1pbl9yZXdhcmQAAAAAAAs=",
        "AAAAAQAAAAAAAAAAAAAAFUxpcXVpZGl0eVBvb2xJbml0SW5mbwAAAAAAAAgAAAAAAAAABWFkbWluAAAAAAAAEwAAAAAAAAANZmVlX3JlY2lwaWVudAAAAAAAABMAAAAAAAAAGG1heF9hbGxvd2VkX3NsaXBwYWdlX2JwcwAAAAcAAAAAAAAAFm1heF9hbGxvd2VkX3NwcmVhZF9icHMAAAAAAAcAAAAAAAAAEG1heF9yZWZlcnJhbF9icHMAAAAHAAAAAAAAAA9zdGFrZV9pbml0X2luZm8AAAAH0AAAAA1TdGFrZUluaXRJbmZvAAAAAAAAAAAAAAxzd2FwX2ZlZV9icHMAAAAHAAAAAAAAAA90b2tlbl9pbml0X2luZm8AAAAH0AAAAA1Ub2tlbkluaXRJbmZvAAAA",
      ]),
      options
    );
  }
  public readonly fromJSON = {
    initialize: this.txFromJSON<null>,
    create_liquidity_pool: this.txFromJSON<string>,
    update_whitelisted_accounts: this.txFromJSON<null>,
    update_wasm_hashes: this.txFromJSON<null>,
    query_pools: this.txFromJSON<Array<string>>,
    query_pool_details: this.txFromJSON<LiquidityPoolInfo>,
    query_all_pools_details: this.txFromJSON<Array<LiquidityPoolInfo>>,
    query_for_pool_by_token_pair: this.txFromJSON<string>,
    get_admin: this.txFromJSON<string>,
    get_config: this.txFromJSON<Config>,
    query_user_portfolio: this.txFromJSON<UserPortfolio>,
    update: this.txFromJSON<null>,
  };
}