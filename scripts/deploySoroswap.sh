NETWORK="$1"

# Validate the input arguments
if [ -z "$NETWORK" ]; then
    echo "Error: Network name must be provided."
    echo "Usage: bash /workspace/scripts/deploy_random_tokens.sh <network> <number_of_tokens(optional)>"
    exit 1
fi

cd contracts/protocols/soroswap/
yarn
yarn build && yarn deploy $NETWORK