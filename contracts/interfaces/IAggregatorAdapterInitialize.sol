// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

/**
 * @title IAggregatorAdapterInitialize
 * @notice Independent initialization trait for aggregator adapters
 * @dev Separated from ISoroswapAggregatorAdapterInterface to allow
 *      different adapters to have different initialization options
 */
interface IAggregatorAdapterInitialize {
    /**
     * @notice Initializes the adapter
     * @param _nativeToken The address of the native token
     * @param _chainId The chain ID
     */
    function initialize(address _nativeToken, uint256 _chainId) external;
}
