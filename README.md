# Soroswap Aggregator 

The aggregator currently aggregates the pools from soroswap protocol and phoenix protocol
**For standalone development read #Development section**

### 1. Setup

1.1. Clone this repo

```
git clone --recurse-submodules http://github.com/soroswap/aggregator.git
```

1.2 In one terminal: (choose standalone, futurenet or testnet)

```
bash scripts/quickstart.sh standalone # or futurenet or testnet
```

1.3. In another terminal, to enter the docker container

```
bash scripts/run.sh
```

1.4 yarn install

```
yarn
```

after you have the enviroment setted up and inside the docker container you have to build the smart contracts with

```bash 
cd /workspace/contracts
make build
```

this could take a while since is building all the protocols smart contracts
after everything is built you can now follow the following instructions

## Deployment

To deploy the smart contracts you first would need to build the source with
```bash
yarn build
```

after the scripts are built you can run this to deploy, networks can be 'testnet', 'standalone', 'futurenet', 'mainnet'
```bash
yarn deploy <network>
```

when deployment is completed you can find the addresses in ./.sosorban directory

## Development
When deploying to any network other than mainnet the script will also deploy Phoenix Protocol for testing purposes

**For development in standalone you should deploy soroswap smart contracts from the soroswap submodule, to do so there is a script you can run... You will need to set the .env inside the submodule**
```bash
bash scripts/deploySoroswap.sh <network>
```