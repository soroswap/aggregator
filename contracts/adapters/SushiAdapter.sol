// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {ISoroswapAggregatorAdapterInterface} from "../interfaces/IAggregatorAdapter.sol";
import {IAggregatorAdapterInitialize} from "../interfaces/IAggregatorAdapterInitialize.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

/**
 * @title SushiAdapter
 * @notice Adapter for SushiSwap DEX
 */
contract SushiAdapter is ISoroswapAggregatorAdapterInterface, IAggregatorAdapterInitialize, Ownable {
    using SafeERC20 for IERC20;

    address public nativeToken;
    uint256 public chainId;

    // SushiSwap pair factory
    address public immutable sushiFactory;
    // Router for swaps
    address public immutable sushiRouter;

    error Adapter__InvalidInitialization();
    error Adapter__InvalidAmount();
    error Adapter__InsufficientAmount();

    event AdapterInitialized(address indexed nativeToken, uint256 indexed chainId);

    constructor(address _sushiFactory, address _sushiRouter) Ownable(msg.sender) {
        sushiFactory = _sushiFactory;
        sushiRouter = _sushiRouter;
    }

    /**
     * @notice Initializes the adapter
     * @dev Now implements IAggregatorAdapterInitialize instead of being part of the interface
     */
    function initialize(address _nativeToken, uint256 _chainId) external override {
        if (nativeToken != address(0)) {
            revert Adapter__InvalidInitialization();
        }
        nativeToken = _nativeToken;
        chainId = _chainId;
        emit AdapterInitialized(_nativeToken, _chainId);
    }

    function swap(
        address tokenFrom,
        address tokenTo,
        uint256 amountFrom,
        uint256 minAmountTo,
        bytes calldata params
    ) external override returns (uint256 amountTo) {
        // Implementation would go here
        return amountTo;
    }

    function adapterType() external pure override returns (string memory) {
        return "Sushi";
    }

    // Helper functions for swapping
    function _swap(address tokenIn, address tokenOut, uint256 amount) internal returns (uint256) {
        // Implementation
        return 0;
    }

    receive() external payable {}
}
