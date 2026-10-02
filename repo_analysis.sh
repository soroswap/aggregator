#!/bin/bash
# Fetching repo structure
curl -s "https://api.github.com/repos/soroswap/aggregator/git/trees/main?recursive=1" | jq -r '.tree[] | select(.path | test("\\.sol$")) | .path' | sort
