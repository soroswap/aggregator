// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

/**
 * @title ISoroswapAggregatorAdapterInterface
 * @notice Interface for aggregator adapters
 * @dev Note: initialize has been separated into IAggregatorAdapterInitialize
 *      to allow different adapters to define their own initialization requirements
 */
interface ISoroswapAggregatorAdapterInterface {
    /**
     * @notice Swaps tokens through the adapter
     * @param tokenFrom The address of the token to swap from
     * @param tokenTo The address of the token to swap to
     * @param amountFrom The amount of tokens to swap
     * @param minAmountTo The minimum amount of tokens to receive
     * @param params Additional parameters for the adapter
     * @return amountTo The amount of tokens received
     */
    function swap(
        address tokenFrom,
        address tokenTo,
        uint256 amountFrom,
        uint256 minAmountTo,
        bytes calldata params
    ) external returns (uint256 amountTo);
}
