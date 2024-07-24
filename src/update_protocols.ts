import { Address, xdr, nativeToScVal } from '@stellar/stellar-sdk';
import { AddressBook } from './utils/address_book.js';
import { invokeContract } from './utils/contract.js';
import { config } from './utils/env_config.js';

export async function updateAdapters(addressBook: AddressBook) {
  if(network == 'mainnet') throw new Error('Mainnet not yet supported')
  
  //   pub struct Adapter {
  //     pub protocol_id: String,
  //     pub address: Address,
  //     pub paused: bool,
  // }
  const adaptersVec = [
    {
      protocol_id: "phoenix",
      address: new Address(addressBook.getContractId('phoenix_adapter')),
      paused: false
    },
  ];

  const adaptersVecScVal = xdr.ScVal.scvVec(adaptersVec.map((adapter) => {
    return xdr.ScVal.scvMap([
      new xdr.ScMapEntry({
        key: xdr.ScVal.scvSymbol('address'),
        val: adapter.address.toScVal(),
      }),
      new xdr.ScMapEntry({
        key: xdr.ScVal.scvSymbol('paused'),
        val: nativeToScVal(adapter.paused),
      }),
      new xdr.ScMapEntry({
        key: xdr.ScVal.scvSymbol('protocol_id'),
        val: xdr.ScVal.scvString(adapter.protocol_id),
      }),
    ]);
  }));


  const aggregatorUpdateAdaptersParams: xdr.ScVal[] = [
    adaptersVecScVal, // adapter_vec: Vec<Adapter>
  ];

  // fn update_adapters(e: Env, adapter_vec: Vec<Adapter>)
  console.log("Updating Adapters")
  await invokeContract(
    'aggregator',
    addressBook,
    'update_adapters',
    aggregatorUpdateAdaptersParams,
    loadedConfig.admin
  );
}

const network = process.argv[2];
const addressBook = AddressBook.loadFromFile(network);

const loadedConfig = config(network);