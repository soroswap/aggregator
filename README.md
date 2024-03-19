# Soroswap Aggregator 

The aggregator currently aggregates the pools from soroswap protocol and phoenix protocol


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

after the scripts are built you can run this to deploy, networks can be 'testnet', 'standalone', 'futurenet'
```bash
yarn deploy <network>
```

when deployment is completed you can find the addresses in ./.sosorban directore

if you want to publish the deployments (move the contracts from .soroban to public) you can run 
```bash
yarn upload <network>
```

## Standalone

when using standalone as network the deploy script will look inside the .soroban directory inside the soroswap protocol directory, this directory will only exist if you deploy soroswap core inside the contracts/protocols/soroswap directory while being inside thwe container
