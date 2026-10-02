#!/bin/bash
# Search for adapters that implement initialize
grep -r "function initialize" --include="*.sol" .
grep -r "IAggregatorAdapter" --include="*.sol" .
