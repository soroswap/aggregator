## Analysis of Issue #36

### Goal
Separate `initialize` from `SoroswapAggregatorAdapterInterface` and make it an independent trait.

### Current State
- `SoroswapAggregatorAdapterInterface` likely contains an `initialize` function
- This couples initialization logic to the adapter interface

### Target State
- Create a separate `IAggregatorAdapterInitialize` trait
- Remove `initialize` from `SoroswapAggregatorAdapterInterface`
- Adapters that need initialization implement the new trait independently

### Files to Modify
1. `contracts/interfaces/IAggregatorAdapter.sol` - Remove initialize from interface
2. Create `contracts/interfaces/IAggregatorAdapterInitialize.sol` - New independent trait
3. Update any adapters that implement initialize to also implement the new trait
4. Update any callers that rely on initialize being part of the interface
