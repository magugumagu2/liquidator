// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

interface IKittenPair {
    function swap(
        uint256 amount0Out,
        uint256 amount1Out,
        address to,
        bytes calldata data
    ) external;

    function metadata()
        external
        view
        returns (
            uint dec0,
            uint dec1,
            uint r0,
            uint r1,
            bool st,
            address t0,
            address t1
        );
    
    function token0() external view returns (address);

    function token1() external view returns (address);

    function reserve0() external view returns (uint);

    function reserve1() external view returns (uint);

    function getAmountOut(uint amountIn, address tokenIn) external view returns (uint);
}