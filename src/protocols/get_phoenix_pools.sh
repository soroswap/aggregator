#!/bin/bash
# Check if the argument is provided
if [ -z "$1" ]; then
    echo "Usage: $0 <network>"
    exit 1
fi

NETWORK="$1"
echo "----------------------------------------"
CONFIGS_FILE="/workspace/configs.json"

FRIENDBOT_URL=$(jq -r --arg NETWORK "$NETWORK" '.networkConfig[] | select(.network == $NETWORK) | .friendbot_url' "$CONFIGS_FILE")
SOROBAN_RPC_URL=$(jq -r --arg NETWORK "$NETWORK" '.networkConfig[] | select(.network == $NETWORK) | .soroban_rpc_url' "$CONFIGS_FILE")
SOROBAN_NETWORK_PASSPHRASE=$(jq -r --arg NETWORK "$NETWORK" '.networkConfig[] | select(.network == $NETWORK) | .soroban_network_passphrase' "$CONFIGS_FILE")

soroban config network add "$NETWORK" \
  --rpc-url "$SOROBAN_RPC_URL" \
  --network-passphrase "$SOROBAN_NETWORK_PASSPHRASE"

ARGS="--network $NETWORK --source phoenix-admin"

echo "Getting Phoenix pools"
soroban contract invoke \
  $ARGS \
  --id CDM4B5WKRBXZUWUAGSXR5NPMPOVUGG53WVRGRAOSSNSOPAYO4AQEYIIB \
  -- \
  query_pools