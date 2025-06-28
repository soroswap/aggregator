#!/bin/bash

# Get preview hash from configs.json
previewHash=$(jq -r '.previewHash' configs.json)
previewVersion=$(echo "$previewHash" | cut -d'@' -f1)

echo "=== Setting up development environment for Soroban on testnet ==="

# Verify if Docker network exists
echo "1. Checking Docker network soroban-network"
(docker network inspect soroban-network -f '{{.Id}}' 2>/dev/null) \
  || docker network create soroban-network
echo "✓ Docker network ready"
echo ""

# Check if a previous container exists
echo "2. Looking for previous container soroban-preview-${previewVersion}"
containerID=$(docker ps --filter="name=soroban-preview-${previewVersion}" --all --quiet)
if [[ ${containerID} ]]; then
    echo "🔄 Removing existing container soroban-preview-${previewVersion}"
    docker rm --force soroban-preview-${previewVersion}
else
    echo "✓ No previous container exists"
fi
echo ""

# Create and run the container with explicit DNS
echo "3. Creating container soroban-preview-${previewVersion} with configured DNS"
currentDir=$(pwd)
docker run -dti \
  --volume ${currentDir}:/workspace \
  --name soroban-preview-${previewVersion} \
  -p 8001:8000 \
  --ipc=host \
  --network=host \
  --dns 8.8.8.8 \
  --dns 8.8.4.4 \
  esteblock/soroban-preview:${previewHash}
echo "✓ Container created successfully with Google DNS"
echo ""

# Verify connectivity inside the container
echo "4. Verifying Internet connectivity..."
docker exec soroban-preview-${previewVersion} ping -c 1 google.com || echo "⚠️ Warning: No Internet connection"
echo ""

echo "5. Accessing the container..."
echo "🚀 You're now inside the container and you can:"
echo "   - Build your contracts with: cargo build --release --target wasm32-unknown-unknown"
echo "   - Configure the testnet network"
echo "   - Deploy your contracts"
echo ""
echo "✅ Commands to execute in order:"
echo "   1. cd /workspace/contracts/aggregator"
echo "   2. make build"
echo "   3. stellar network add testnet \\"
echo "      --rpc-url \"https://testnet.stellar.validationcloud.io/v1/FQ2s-y1fhQSsMzT1GH4fkp6KZRf8uif6TtmNCv_yCoA\" \\"
echo "      --network-passphrase \"Test SDF Network ; September 2015\""
echo "   4. stellar contract deploy \\"
echo "      --wasm ../target/wasm32-unknown-unknown/release/soroswap_aggregator.optimized.wasm \\"
echo "      --source admin_aggregator \\"
echo "      --network testnet"
echo ""
echo "⚠️ If you continue having DNS issues, run inside the container:"
echo "   echo 'nameserver 8.8.8.8' > /etc/resolv.conf"
echo ""
echo "To exit the container use: exit"
echo "====================================="

# Access the container
docker exec -it soroban-preview-${previewVersion} bash 