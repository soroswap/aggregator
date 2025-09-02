# Soroswap Aggregator 

The Soroswap Aggregator aggregates liquidity from different Soroban based AMMs

📚 Documentation: [Soroswap Documentation](https://docs.soroswap.finance/)

🔒 Audit Report: [Runtime Verification](./audits/2024-08-31_Soroswap_Aggregator_Audit_by_RuntimeVerification.pdf)

📑 Audit Summary: [Runtime Verification](./audits/2024-08-31_Soroswap_Aggregator_Audit_Summary_by_RuntimeVerification.pdf)

🌐 Deployed Address: [`./public/mainnet.json`](./public/mainnet.json)


# Setup and Deployment

**For standalone development read #Development section**

> [!IMPORTANT] 
> Be sure to clone the repository with its submodules to ensure proper execution of development, testing, and deploying scripts.

## 1. Setup

1.1. Clone this repo. Submodules are necesary to get the Public and Testnet addresses of the underlying protocols like Soroswap, Phoenix, Aqua, etc... or to deploy on Standalone and Testnet those protocols.

```bash
git clone --recurse-submodules http://github.com/soroswap/aggregator.git
```
> [!TIP]
> If you forgot to clone with the `--recurse-submodules` flag, you can run `git submodule update --init --recursive` to get the submodules.

> [!NOTE]
If there was a Testnet reset, and the soroswap/core repo has new Tesnet deployments for test tokens, you want to bring those changes
```
cd protocols/soroswap/
git pull origin main
```
> [!NOTE]
If you want to update the submodules, do
```
git pull --recurse-submodules
```

1.2 Copy the `.env.example` file into `.env` and modify the necessary parameters
```bash
cp .env.example .env
```
For the **secret keys**, you can create an account and private keys in https://laboratory.stellar.org/#account-creator?network=test.
For `MAINNET_RPC_URL`, you will need to subscribe to one of the Stellar Mainnet RPC providers: https://app.validationcloud.io/, https://nownodes.io/ or others (ask in the Stellar Discord)

1.2 In one terminal: (choose standalone, futurenet or testnet)

```bash
bash scripts/quickstart.sh standalone # or futurenet or testnet
```

1.3. In another terminal, to enter the docker container

```bash
bash scripts/run.sh
```

1.4 yarn install

```bash
yarn
```

2.- Build the Smart Contracts: after you have the enviroment setted up and inside the docker container you have to build the smart contracts with

```bash 
cd /workspace/contracts
make build
```

### (Optional) Upgrade, Compile other protocols and test
If you are considering other protocol that have changed their versions, upgrade them.
Make sure that you did a recursive pull

However, you dont need to do this very often.

For Phoenix:
```
cd protocols/phoenix-contracts/
make build
cp target/wasm32v1-none/release/*.wasm ../../contracts/adapters/phoenix/phoenix_contracts/

# make sure the tests still pass
cd /workspace/contracts/adapters/phoenix
rustup install 1.79.0 # Phoenix needs to downgrade
rustup override set 1.79.0
rustup target add wasm32v1-none
make test

```

For Aqua:
```
cd protocols/aqua
npm install -g @go-task/cli
task build
cp target/wasm32v1-none/release/*.wasm ../../contracts/adapters/aqua/aqua_contracts/
```

## 2. Run Tests and Scout Audit
```
cd /workspace/contracts/
make test
```
For Scout Audits (tool created by CoinFabrik), you should enter in each of the sub projects, for example
```bash
cd /workspace/contracts/aggregator
cargo scout-audit
```
or, in the case you want to audit the Soroswap.Finance adapter (deprecated)
```bash
cd /workspace/contracts/adapters/soroswap
cargo scout-audit
```

## 3.- Check CPU Instructios and Memory Usage
```
cd /workspace/contracts/aggregator
cargo test budget -- --nocapture
```
Export it into a file that you will save together with your changes
```
cargo test budget -- --nocapture > aggregator_budget.txt
```

## 4.- Deployment

### 4.1- (Optional) Deploy each protocol.
We will consider other protocol addresses from your local ignored files `.soroban/mainnet.contrats.json` and `.soroban/testnet.contrats.json`.

If you want to change their addresses do it there. Also sometimes projects do not keep a testnet version of their protocols. If you need to deploy your own Phoenix, Comet or Aqua version of the protocol, do:

```
cd /workspace
yarn build
yarn setup-phoenix testnet # To Setup Phoenix. Now you will have the new deployed addresses in .soroban/testnet.contrats.json

```
### 4.2 Deploy the Aggregator Aggregator

To deploy the smart contracts you first would need to build the source with
```bash
cd /workspace/contracts
make build
```
The .wasm files will already be optimized and will be available in 
`/workspace/contracts/target/wasm32v1-none/release/` with a name like `[NAME-OF-CONTRACT].optimized.wasm`

after the WASMs are built you can run this to deploy, networks can be `testnet`, `standalone`, `futurenet`, `mainnet`. The RPCs will be taken from the `configs.json` file.

```bash
cd /workspace
yarn build
yarn deploy <network> # use testnet or mainnet
```

NOTE: For Testnet we will use the deployed addresse on the soroswap core repo, so make sure to pull the last version:

```
cd protocols/soroswap/
git pull origin main
```

when deployment is completed you can find the addresses in ./.soroban directory

## 6.- Run Tests directly on the Blockchain
Run javascript tests
```
cd /workspace
yarn test testnet
```


## 6.- Publish deployed address.
If you want to publish the json files that are in the ignored `.soroban` folder, do:

```bash
yarn publish_addresses <network>
```


```
## 7.- Integration Test in Public Testnet. 
Its important to allways test contracts in a live testnet Blockchain.
We have prepared some scripts to interact with the deployed Soroswap.Finance testnet version and with a custom deployed Phoenix protocol. This is because Phoenix does not officially support a testnet version.

You can test the Aggregator methods by running the following command:
```
bash scripts/quickstart.sh standalone
bash scripts/run.sh
yarn test:manual <network>
```
## Development
When deploying to any network other than mainnet the script will also deploy Phoenix Protocol for testing purposes

**For development in standalone you should deploy soroswap smart contracts from the soroswap submodule, to do so there is a script you can run... You will need to set the .env inside the submodule**
```bash
bash scripts/deploySoroswap.sh <network>
```

### Add liquidity to the phoenix pools:
If you want to create and add liquidity to the Phoenix pools, you can do so by running the following command:

> [!IMPORTANT] 
> The `TEST_TOKENS_ADMIN_SECRET_KEY` in your `.env` file must be identical to the one used for Soroswap deployment to successfully add liquidity to the pools in Phoenix.

```bash
yarn add-liquidity:phoenix <network> <number of tokens>*
```
This will create pairs with all tokens listed in the [Soroswap tokens list](https://github.com/soroswap/core/blob/main/public/tokens.json).

> [!NOTE] 
> *The number of tokens is optional, if not provided, the script will add liquidity to all the pools.

### Add new protocol as submodule
```bash
git submodule add <remote_url> <destination_folder>
```

OLD


## 5.- Publish Phoenix Aggregator in Mainnet
```
yarn build && yarn deploy-phoenix-adapter mainnet