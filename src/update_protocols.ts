import { Address, xdr } from '@stellar/stellar-sdk';
import { AddressBook } from './utils/address_book.js';
import { invokeContract } from './utils/contract.js';
import { config } from './utils/env_config.js';

export async function updateAggregatorProtocols(addressBook: AddressBook) {
  if(network == 'mainnet') throw new Error('Mainnet not yet supported')
  
  const protocolAddressPair = [
    {
      protocol_id: "phoenix",
      address: new Address(addressBook.getContractId('phoenix_adapter')),
    },
  ];

  const protocolAddressPairScVal = protocolAddressPair.map((pair) => {
    return xdr.ScVal.scvMap([
      new xdr.ScMapEntry({
        key: xdr.ScVal.scvSymbol('address'),
        val: pair.address.toScVal(),
      }),
      new xdr.ScMapEntry({
        key: xdr.ScVal.scvSymbol('protocol_id'),
        val: xdr.ScVal.scvString(pair.protocol_id),
      }),
    ]);
  });

  const aggregatorProtocolAddressesScVal = xdr.ScVal.scvVec(protocolAddressPairScVal);

  const aggregatorInitParams: xdr.ScVal[] = [
    aggregatorProtocolAddressesScVal, // proxy_addresses: Vec<ProxyAddressPair>,
  ];

  console.log("Initializing Aggregator")
  await invokeContract(
    'aggregator',
    addressBook,
    'update_protocols',
    aggregatorInitParams,
    loadedConfig.admin
  );
}

const network = process.argv[2];
const addressBook = AddressBook.loadFromFile(network);

const loadedConfig = config(network);