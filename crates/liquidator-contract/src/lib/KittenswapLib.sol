// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {IKittenPair} from "../interfaces/IKittenPair.sol";
import {IKittenPairFactory} from "../interfaces/IKittenPairFactory.sol";
import {IERC20} from "../interfaces/IERC20.sol";

library KittenswapLib {
    struct GetAmountInArgs {
        address pair;
        address factory;
        uint amountOut;
        address tokenOut;
        bool stable;
    }

    struct GetAmountInInternalArgs {
        uint reserve0;
        uint reserve1;
        uint decimals0;
        uint decimals1;
        address token0;
        address token1;
    }

    function getAmountIn(
        GetAmountInArgs memory args
    ) internal view returns (uint) {
        uint amountIn = _getAmountIn(
            args, 
            GetAmountInInternalArgs({
                reserve0: IKittenPair(args.pair).reserve0(),
                reserve1: IKittenPair(args.pair).reserve1(),
                decimals0: IERC20(IKittenPair(args.pair).token0()).decimals(),
                decimals1: IERC20(IKittenPair(args.pair).token1()).decimals(),
                token0: IKittenPair(args.pair).token0(),
                token1: IKittenPair(args.pair).token1()
            })
        );
        // add fee to amount in; round up for safety
        return (amountIn * 10000) / (10000 - IKittenPairFactory(args.factory).getFee(address(this), args.stable)) + 1;
    }

    struct GetAmountInLocals {
        uint xy;
        uint reserveIn;
        uint reserveOut;
        uint amountOut;
        uint x;
    }

    function _getAmountIn(
        GetAmountInArgs memory args,
        GetAmountInInternalArgs memory iArgs
    ) internal view returns (uint) {
        GetAmountInLocals memory locals;
        if (args.stable) {
            locals.xy = _k(iArgs.reserve0, iArgs.reserve1, args.stable, iArgs);
            iArgs.reserve0 = (iArgs.reserve0 * 1e18) / iArgs.decimals0;
            iArgs.reserve1 = (iArgs.reserve1 * 1e18) / iArgs.decimals1;
            (locals.reserveOut, locals.reserveIn) = args.tokenOut == iArgs.token0
                ? (iArgs.reserve0, iArgs.reserve1)
                : (iArgs.reserve1, iArgs.reserve0);
            locals.amountOut = args.tokenOut == iArgs.token0
                ? (args.amountOut * 1e18) / iArgs.decimals0
                : (args.amountOut * 1e18) / iArgs.decimals1;
            locals.x = locals.reserveIn - _get_x(locals.reserveOut - args.amountOut, locals.xy, locals.reserveIn);
            return ((locals.x - locals.reserveIn) * (args.tokenOut == iArgs.token0 ? iArgs.decimals1 : iArgs.decimals0)) / 1e18;
        } else {
            (locals.reserveIn, locals.reserveOut) = args.tokenOut == iArgs.token0
                ? (iArgs.reserve1, iArgs.reserve0)
                : (iArgs.reserve0, iArgs.reserve1);
            return (locals.reserveIn * locals.amountOut) / (locals.reserveOut - locals.amountOut);
        }
    }

    function _k(uint x, uint y, bool stable, GetAmountInInternalArgs memory iArgs) internal view returns (uint) {
        if (stable) {
            uint _x = (x * 1e18) / iArgs.decimals0;
            uint _y = (y * 1e18) / iArgs.decimals1;
            uint _a = (_x * _y) / 1e18;
            uint _b = ((_x * _x) / 1e18 + (_y * _y) / 1e18);
            return (_a * _b) / 1e18; // x3y+y3x >= k
        } else {
            return x * y; // xy >= k
        }
    }

    function _get_x(uint y0, uint xy, uint x) internal pure returns (uint) {
        for (uint i = 0; i < 255; i++) {
            uint x_prev = x;
            uint k = _f(x, y0);
            if (k < xy) {
                uint dx = ((xy - k) * 1e18) / _d(x, y0);
                x = x + dx;
            } else {
                uint dx = ((k - xy) * 1e18) / _d(x, y0);
                x = x - dx;
            }
            if (x > x_prev) {
                if (x - x_prev <= 1) {
                    return x;
                }
            } else {
                if (x_prev - x <= 1) {
                    return x;
                }
            }
        }
        return x;
    }

    function _f(uint x, uint y0) internal pure returns (uint) {
        return
            (y0 * ((((x * x) / 1e18) * x) / 1e18)) /
            1e18 +
            (((((y0 * y0) / 1e18) * y0) / 1e18) * x) /
            1e18;
    }

    function _d(uint x, uint y0) internal pure returns (uint) {
        return
            (3 * y0 * ((x * x) / 1e18)) /
            1e18 +
            ((((y0 * y0) / 1e18) * y0) / 1e18);
    }
}
