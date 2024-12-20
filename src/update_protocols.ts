import { Address, xdr, nativeToScVal } from '@stellar/stellar-sdk';
import { AddressBook } from './utils/address_book.js';
import { invokeContract } from './utils/contract.js';
import { config } from './utils/env_config.js';

interface UpdateAdapterProps {
  protocol_id: string, 
  address: Address, 
  paused: boolean
}

export async function updateAdapters(addressBook: AddressBook, adapters: UpdateAdapterProps[]) {
  // if(network == 'mainnet') throw new Error('Mainnet not yet supported')
  
  //   pub struct Adapter {
  //     pub protocol_id: String,
  //     pub address: Address,
  //     pub paused: bool,
  // }
  
  const adaptersVec: UpdateAdapterProps[] = [];
  for (const adapter of adapters) {
    adaptersVec.push({
      protocol_id: adapter.protocol_id,
      address: adapter.address,
      paused: adapter.paused,
    });
  }

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