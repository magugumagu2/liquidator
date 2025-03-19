// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity >=0.5.0;

interface IKittenswapSwapCallback {
    function hook(address sender, uint256 amount0Out, uint256 amount1Out, bytes calldata data) external;
}
