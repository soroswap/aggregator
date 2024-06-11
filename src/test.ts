import { Address, nativeToScVal, scValToNative, xdr } from '@stellar/stellar-sdk';
import { AddressBook } from './utils/address_book.js';
import { invokeContract } from './utils/contract.js';
import { config } from './utils/env_config.js';
import { getCurrentTimePlusOneHour } from './utils/tx.js';

export async function testAggregator(addressBook: AddressBook) {
  console.log('-------------------------------------------------------');
  console.log('Testing Soroswap Aggregator');
  console.log('-------------------------------------------------------');


  console.log("Initializing Aggregator")
  const result = await invokeContract(
    'aggregator',
    addressBook,
    'get_protocols',
    [],
    loadedConfig.admin,
    true
  );
  console.log('🚀 « result:', scValToNative(result.result.retval));

  const dexDistributionRaw = [
    {
      protocol_id: "soroswap",
      path: ["CAPCD5BA3VYK4YWTXUBBXKXXIETXU2GGZZIQ4KDFI4WWTVZHV6OBIUNO", "CCKW6SMINDG6TUWJROIZ535EW2ZUJQEDGSKNIK3FBK26PAMBZDVK2BZA"],
      parts: 1,
      is_exact_in: true,
    },
  ];

  const dexDistributionScVal = dexDistributionRaw.map((distribution) => {
    return xdr.ScVal.scvMap([
      new xdr.ScMapEntry({
        key: xdr.ScVal.scvSymbol('is_exact_in'),
        val: xdr.ScVal.scvBool(distribution.is_exact_in),
      }),
      new xdr.ScMapEntry({
        key: xdr.ScVal.scvSymbol('parts'),
        val: nativeToScVal(distribution.parts),
      }),
      new xdr.ScMapEntry({
        key: xdr.ScVal.scvSymbol('path'),
        val: nativeToScVal(distribution.path.map((pathAddress) => new Address(pathAddress).toScVal())),
      }),
      new xdr.ScMapEntry({
        key: xdr.ScVal.scvSymbol('protocol_id'),
        val: xdr.ScVal.scvString(distribution.protocol_id),
      }),
    ]);
  });

  const dexDistributionScValVec = xdr.ScVal.scvVec(dexDistributionScVal);

  const aggregatorSwapParams: xdr.ScVal[] = [
    new Address("CAPCD5BA3VYK4YWTXUBBXKXXIETXU2GGZZIQ4KDFI4WWTVZHV6OBIUNO").toScVal(), //_from_token: Address,
    new Address("CCKW6SMINDG6TUWJROIZ535EW2ZUJQEDGSKNIK3FBK26PAMBZDVK2BZA").toScVal(), //_dest_token: Address,
    nativeToScVal(1000000000),
    nativeToScVal(0),
    dexDistributionScValVec, // proxy_addresses: Vec<ProxyAddressPair>,
    new Address(loadedConfig.admin.publicKey()).toScVal(), //admin: Address,
    nativeToScVal(getCurrentTimePlusOneHour()), //deadline 
  ];

  console.log("Initializing Aggregator")
  await invokeContract(
    'aggregator',
    addressBook,
    'swap',
    aggregatorSwapParams,
    loadedConfig.admin
  );

}

const network = process.argv[2];
const addressBook = AddressBook.loadFromFile(network);

const loadedConfig = config(network);

await testAggregator(addressBook);