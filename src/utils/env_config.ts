import { Horizon, Keypair, rpc } from '@stellar/stellar-sdk';
import dotenv from "dotenv";
import * as fs from "fs";
import path from "path";
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
dotenv.config({ path: path.join(__dirname, "../../.env") });

interface NetworkConfig {
  network: string;
  friendbot_url: string;
  horizon_rpc_url: string;
  soroban_rpc_url: string;
  soroban_network_passphrase: string;
}

interface Config {
  previewHash: string;
  quickstartHash: string;
  networkConfig: NetworkConfig[];
}

export class EnvConfig {
  rpc: rpc.Server;
  horizonRpc: Horizon.Server;
  passphrase: string;
  friendbot: string | undefined;
  admin: Keypair;
  tokenAdmin: Keypair;
  phoenixAdmin: Keypair;
  testUser: Keypair;

  constructor(
    rpc: rpc.Server,
    horizonRpc: Horizon.Server,
    passphrase: string,
    friendbot: string | undefined,
    admin: Keypair,
    tokenAdmin: Keypair,
    phoenixAdmin: Keypair,
    testUser: Keypair,
  ) {
    this.rpc = rpc;
    this.horizonRpc = horizonRpc;
    this.passphrase = passphrase;
    this.friendbot = friendbot;
    this.admin = admin;
    this.tokenAdmin = tokenAdmin;
    this.phoenixAdmin = phoenixAdmin;
    this.testUser = testUser ;
  }

  /**
   * Load the environment config from the .env file
   * @returns Environment config
   */
  static loadFromFile(network: string): EnvConfig {
    const fileContents = fs.readFileSync(
      path.join(__dirname, "../../configs.json"),
      "utf8",
    );
    const configs: Config = JSON.parse(fileContents);

    let rpc_url, horizon_rpc_url, friendbot_url, passphrase;
    
    const networkConfig = configs.networkConfig.find((config) => config.network === network);
    if (!networkConfig) {
      throw new Error(`Network configuration for '${network}' not found`);
    }

    if (network === 'mainnet') {
      passphrase = networkConfig.soroban_network_passphrase;
      rpc_url = process.env.MAINNET_RPC_URL;
      horizon_rpc_url = networkConfig.horizon_rpc_url;
      friendbot_url = undefined;
    } else {
      rpc_url = networkConfig.soroban_rpc_url;
      horizon_rpc_url = networkConfig.horizon_rpc_url;
      friendbot_url = networkConfig.friendbot_url;
      passphrase = networkConfig.soroban_network_passphrase;
    }

    const admin = process.env.AGGREGATOR_DEPLOYER_ADMIN_SECRET_KEY;
    const tokenAdmin = process.env.TEST_TOKENS_ADMIN_SECRET_KEY;
    const phoenixAdmin = process.env.PHOENIX_DEPLOYER_SECRET_KEY;
    const testUser = process.env.TEST_USER_SECRET_KEY;

    if (
      rpc_url === undefined ||
      horizon_rpc_url === undefined ||
      (network != "mainnet" && friendbot_url === undefined) ||
      passphrase === undefined
      ) {
      throw new Error('Error: Configuration is missing required fields, include <network>');
    }
    if (   
      admin === undefined ||
      tokenAdmin === undefined ||
      phoenixAdmin === undefined ||
      testUser === undefined
    ) {
      throw new Error('Error: Configuration is missing required fields, please read .env.example to set up the required fields');
    }

    const allowHttp = network === "standalone";

    console.log(admin)

    return new EnvConfig(
      new rpc.Server(rpc_url, { allowHttp }),
      new Horizon.Server(horizon_rpc_url, {allowHttp}),
      passphrase,
      friendbot_url,
      Keypair.fromSecret(admin),
      Keypair.fromSecret(tokenAdmin),
      Keypair.fromSecret(phoenixAdmin),
      Keypair.fromSecret(testUser),
    );
  }

  /**
   * Get the Keypair for a user from the env file
   * @param userKey - The name of the user in the env file
   * @returns Keypair for the user
   */
  getUser(userKey: string): Keypair {
    const userSecretKey = process.env[userKey];
    if (userSecretKey === undefined) {
      throw new Error(`${userKey} secret key not found in .env`);
    }
    try {
      return Keypair.fromSecret(userSecretKey);
    }
    catch (e) {
      throw new Error(`${userKey} secret key
        might not be found in .env. Failed with error ${e}`);
    }
  }
}

export const config = (network: string) => {
  return EnvConfig.loadFromFile(network);
};
