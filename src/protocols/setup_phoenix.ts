import { AddressBook } from "../utils/address_book.js";
import { config } from "../utils/env_config.js";
import { phoenixSetup } from "./phoenix/phoenix_setup.js";

const network = process.argv[2];
const addressBook = AddressBook.loadFromFile(network);
const loadedConfig = config(network);

async function setupPhoenix(){
  await phoenixSetup(loadedConfig, addressBook);
}

await setupPhoenix();