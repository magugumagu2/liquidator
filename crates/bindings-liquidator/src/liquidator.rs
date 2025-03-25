pub use liquidator::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod liquidator {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("FLASH_MINTER"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("FLASH_MINTER"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IUsdxlFlashMinter",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("USDXL"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("USDXL"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IERC20"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("approvePool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("approvePool"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("executeOperation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("executeOperation"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("fee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("success"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("hook"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hook"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount0Out"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount1Out"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("hyperswapV3Factory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hyperswapV3Factory"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isLiquidator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isLiquidator"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("kittenPairFactory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("kittenPairFactory"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IKittenPairFactory",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("liquidate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("liquidate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("collateralAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("debtAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("debtToCover"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swapPath"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liqPath"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("finalToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("finalGain"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pool"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IPool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("recover"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("recover"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setLiquidator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setLiquidator"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_liquidator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_enabled"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("uniswapV3SwapCallback"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "uniswapV3SwapCallback",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount0Delta"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount1Delta"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("LiquidatorSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("LiquidatorSet"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("liquidator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("enabled"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipTransferred",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static LIQUIDATOR_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15`\x0EW__\xFD[P_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x90\x81\x17\x82U`@Q\x90\x91\x82\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90\x82\x90\xA3Pa0\x12\x80a\0\\_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\xF0W_5`\xE0\x1C\x80cR\x9A5o\x11a\0\x93W\x80c\x9A{\xFFy\x11a\0cW\x80c\x9A{\xFFy\x14a\x02AW\x80c\xEA\x93\x9F\xA6\x14a\x02TW\x80c\xF2\xFD\xE3\x8B\x14a\x02oW\x80c\xFAF\x1E3\x14a\x02\x82W__\xFD[\x80cR\x9A5o\x14a\x01\xDFW\x80cW\x05\xAEC\x14a\x02\x01W\x80cp\xC2j^\x14a\x02\x14W\x80c\x8D\xA5\xCB[\x14a\x02/W__\xFD[\x80c.C\xC9a\x11a\0\xCEW\x80c.C\xC9a\x14a\x01jW\x80c>\r\x95Z\x14a\x01\x9CW\x80cBL&[\x14a\x01\xB7W\x80cDS\xA3t\x14a\x01\xCCW__\xFD[\x80c\x08\xBE\xA1'\x14a\0\xF4W\x80c\r\xB7\xB0(\x14a\x01,W\x80c\x16\xF0\x11[\x14a\x01OW[__\xFD[a\x01\x0Fs\xDA\x12\xF4PX\nL\xC4\x85\xC3\xB5\x01\xBA\xB7\xB0\xB3\xCB\xC3\xB3\x1B\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01?a\x01:6`\x04a(1V[a\x02\x95V[`@Q\x90\x15\x15\x81R` \x01a\x01#V[a\x01\x0Fs\xCE\xCC\xE0\xEB\x9D\xD2\xEFy\x96\xE0\x1E%\xDDp\xE4a\xF9\x18\xA1K\x81V[a\x01}a\x01x6`\x04a(\xA2V[a\x06|V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R\x01a\x01#V[a\x01\x0Fs\xB1\xC0\xFA\x0Bx\x93 \x04Job<\xFE^\xBD\xA9V&\x02\xE3\x81V[a\x01\xCAa\x01\xC56`\x04a)QV[a\x0C\x0EV[\0[a\x01\xCAa\x01\xDA6`\x04a)\x84V[a\x0C\xD6V[a\x01?a\x01\xED6`\x04a)QV[`\x02` R_\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x01\xCAa\x02\x0F6`\x04a)\xBBV[a\raV[a\x01\x0Fs\xCAy\xDBKI\xF6\x08\xEFT\xA5\xCB\x81?\xBE\xD3\xA68{\xC6E\x81V[_Ta\x01\x0F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xCAa\x02O6`\x04a)\xE5V[a\x0E6V[a\x01\x0Fs\xD1/\x1C@!\x97\"C9\xD5\xA3$\xAC~\xF4\xDF]!B\xE9\x81V[a\x01\xCAa\x02}6`\x04a)QV[a\x10oV[a\x01\xCAa\x02\x906`\x04a(1V[a\x10\xE2V[_3s\xD1/\x1C@!\x97\"C9\xD5\xA3$\xAC~\xF4\xDF]!B\xE9\x14a\x02\xFEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FCaller must be flash minter\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x85\x90s\xCAy\xDBKI\xF6\x08\xEFT\xA5\xCB\x81?\xBE\xD3\xA68{\xC6E\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03MW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03q\x91\x90a*JV[\x10\x15a\x03\xBFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FInvalid balance for flash loan\0\0`D\x82\x01R`d\x01a\x02\xF5V[a\x03\xC7a'\x1DV[a\x03\xD3\x83\x85\x01\x85a+*V[`@\x80\x83\x01\x82\x90R\x01Q`\x01`\x01`\xA0\x1B\x03\x16s\xCAy\xDBKI\xF6\x08\xEFT\xA5\xCB\x81?\xBE\xD3\xA68{\xC6E\x14a\x04HW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FDebt asset must be USDXL\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\xF5V[a\x04R\x85\x87a,\x14V[\x81R`@\x80\x82\x01Q` \x01Q\x90Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xA0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xC4\x91\x90a*JV[` \x80\x83\x01\x91\x90\x91R`@\x80\x83\x01Q\x91\x82\x01Q\x82\x82\x01Q``\x84\x01Q`\x80\x90\x94\x01Q\x92Qb\xA7\x18\xA9`\xE0\x1B\x81Rs\xCE\xCC\xE0\xEB\x9D\xD2\xEFy\x96\xE0\x1E%\xDDp\xE4a\xF9\x18\xA1K\x94b\xA7\x18\xA9\x94a\x05\x1C\x94\x93\x92_\x90`\x04\x01a,'V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x053W__\xFD[PZ\xF1\x15\x80\x15a\x05EW=__>=_\xFD[PPPP` \x81\x81\x01Q`@\x80\x84\x01Q\x90\x92\x01Q\x91Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x9BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xBF\x91\x90a*JV[a\x05\xC9\x91\x90a,[V[` \x82\x01\x81\x90R`@\x82\x01Qa\x05\xDF\x91\x90a\x12lV[\x80Q`@Qc\t^\xA7\xB3`\xE0\x1B\x81Rs\xD1/\x1C@!\x97\"C9\xD5\xA3$\xAC~\xF4\xDF]!B\xE9`\x04\x82\x01R`$\x81\x01\x91\x90\x91Rs\xCAy\xDBKI\xF6\x08\xEFT\xA5\xCB\x81?\xBE\xD3\xA68{\xC6E\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x06KW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06o\x91\x90a,nV[P`\x01\x96\x95PPPPPPV[_\x80T\x81\x90`\x01`\x01`\xA0\x1B\x03\x163\x14\x80a\x06\xA5WP3_\x90\x81R`\x02` R`@\x90 T`\xFF\x16[a\x06\xC1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\xF5\x90a,\x89V[`@Qi\x06\xB6\x97GFV\xE77v\x17`\xB4\x1B` \x82\x01R_\x90`*\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x85\x85`@Q` \x01a\x07\x05\x92\x91\x90a,\xD8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x03a\x08\xACW`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x8C\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07bW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x86\x91\x90a*JV[\x90Pa\x08/\x88`@Q\x80a\x01\0\x01`@R\x80\x8A\x8A\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x8E`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8D`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8C`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8B\x81R` \x01_\x81R` \x01`\x01\x15\x15\x81R` \x01`\x01\x15\x15\x81RPa\x14\xE8V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x8B\x93P\x81\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90cp\xA0\x821\x90`$\x01[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08wW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x9B\x91\x90a*JV[a\x08\xA5\x91\x90a,\xE7V[\x91Pa\x0C\0V[`@Qh\x06\x87\x97\x06W'7v\x17`\xBC\x1B` \x82\x01R`)\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x85\x85`@Q` \x01a\x08\xED\x92\x91\x90a,\xD8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x03a\n\x17W`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x8C\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\tJW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tn\x91\x90a*JV[\x90Pa\x08/\x88`@Q\x80a\x01\0\x01`@R\x80\x8A\x8A\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x8E`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8D`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8C`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8B\x81R` \x01_\x81R` \x01`\x01\x15\x15\x81R` \x01`\x01\x15\x15\x81RPa\x16\xF9V[`@Qo:\xB9\xB2<6#60\xB9\xB4&\xB4\xB7:2\xB9`\x81\x1B` \x82\x01R`0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x85\x85`@Q` \x01a\n_\x92\x91\x90a,\xD8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x03a\x0B\xB8W`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x8B\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xBCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xE0\x91\x90a*JV[\x90Pa\x0B\x87\x88`@Q\x80a\x01\0\x01`@R\x80\x8A\x8A\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x8E`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8D`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8C`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8B\x81R` \x01_\x81R` \x01_\x15\x15\x81R` \x01_\x15\x15\x81RPa\x18\x87V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x8A\x93P\x81\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90cp\xA0\x821\x90`$\x01a\x08\\V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FInvalid liquidation path\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\xF5V[P\x98P\x98\x96PPPPPPPV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14\x80a\x0C4WP3_\x90\x81R`\x02` R`@\x90 T`\xFF\x16[a\x0CPW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\xF5\x90a,\x89V[`@Qc\t^\xA7\xB3`\xE0\x1B\x81Rs\xCE\xCC\xE0\xEB\x9D\xD2\xEFy\x96\xE0\x1E%\xDDp\xE4a\xF9\x18\xA1K`\x04\x82\x01R_\x19`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0C\xAEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xD2\x91\x90a,nV[PPV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0C\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\xF5\x90a-\rV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x81\x81R`\x02` R`@\x90\x81\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x7F\x81\xE0 4At\x97,Y\xF6\xC1\x1A\x8Fl\x90\xB1A\x86b\x14\xE3\xD9\xB5D\xD00\xF0\xB52\xF5\xA1\x0F\x90a\rU\x90\x84\x15\x15\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2PPV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\r\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\xF5\x90a-\rV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\r\xC7W`@Q3\x90\x82\x15a\x08\xFC\x02\x90\x83\x90_\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\r\xC2W=__>=_\xFD[PPPV[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0E\x12W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xC2\x91\x90a,nV[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0E\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7Fmsg.sender != activeKittenPair\0\0`D\x82\x01R`d\x01a\x02\xF5V[a\x0E\x98a'\x7FV[a\x0E\xA4\x82\x84\x01\x84a+*V[`\x80\x82\x01\x81\x90RQa\x0E\xB5\x90a\x19\0V[\x15\x15``\x85\x01Rb\xFF\xFF\xFF\x16`@\x84\x01R`\x01`\x01`\xA0\x1B\x03\x90\x81\x16` \x84\x01R\x16\x81R`\x80\x81\x01Q`\xC0\x01Q\x15a\x0FjW`\x80\x80\x82\x01Q` \x81\x01Q`@\x80\x83\x01Q``\x84\x01Q\x93\x90\x94\x01Q\x90Qb\xA7\x18\xA9`\xE0\x1B\x81Rs\xCE\xCC\xE0\xEB\x9D\xD2\xEFy\x96\xE0\x1E%\xDDp\xE4a\xF9\x18\xA1K\x94b\xA7\x18\xA9\x94a\x0F<\x94\x93\x91\x92\x90\x91\x90_\x90`\x04\x01a,'V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0FSW__\xFD[PZ\xF1\x15\x80\x15a\x0FeW=__>=_\xFD[PPPP[`\x80\x81\x01QQa\x0Fy\x90a\x19pV[\x15a\x0F\xCCW`\x80\x81\x01QQa\x0F\x8D\x90a\x19\xC1V[`\x80\x82\x01\x80Q\x91\x90\x91RQ`\xE0\x01Q\x15a\x0F\xB9W`\x80\x81\x01Q`\xA0\x81\x01Qa\x0F\xB4\x91a\x14\xE8V[a\x0F\xCCV[`\x80\x81\x01Q`\xA0\x81\x01Qa\x0F\xCC\x91a\x12lV[\x80`\x80\x01Q`\xE0\x01Q\x15a\x0F\xEBW` \x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R[\x80Q`\x80\x82\x01Q`\xA0\x01Q`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x10BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10f\x91\x90a,nV[PPPPPPPV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x10\x98W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\xF5\x90a-\rV[_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x82U`@Q\x90\x913\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PV[_a\x10\xEF\x82\x84\x01\x84a+*V[\x90P___a\x11\0\x84_\x01Qa\x1A\x10V[\x92P\x92P\x92Pa\x11.s\xB1\xC0\xFA\x0Bx\x93 \x04Job<\xFE^\xBD\xA9V&\x02\xE3a\x11)\x85\x85\x85a\x1AOV[a\x1A\xB9V[\x83`\xC0\x01Q\x15a\x11\xB3W` \x84\x01Q`@\x80\x86\x01Q``\x87\x01Q`\x80\x88\x01Q\x92Qb\xA7\x18\xA9`\xE0\x1B\x81Rs\xCE\xCC\xE0\xEB\x9D\xD2\xEFy\x96\xE0\x1E%\xDDp\xE4a\xF9\x18\xA1K\x94b\xA7\x18\xA9\x94a\x11\x85\x94\x91\x93\x91\x92_\x90`\x04\x01a,'V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x11\x9CW__\xFD[PZ\xF1\x15\x80\x15a\x11\xAEW=__>=_\xFD[PPPP[__\x89\x13a\x11\xC1W\x87a\x11\xC3V[\x88[\x90Pa\x11\xD1\x85_\x01Qa\x1B\rV[\x15a\x11\xEDW\x84Qa\x11\xE1\x90a\x1B'V[\x85Ra\x11\xED\x81\x86a\x16\xF9V[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R\x92\x93P\x83\x92`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x12<W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12`\x91\x90a,nV[PPPPPPPPPPV[`@\x80Q`\xC0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91R\x81Qa\x12\xA8\x90a\x19\0V[\x15\x15`\xA0\x85\x01\x81\x90Rb\xFF\xFF\xFF\x91\x90\x91\x16``\x85\x01R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16` \x85\x01\x81\x90R\x92\x90\x91\x16\x80\x84R`@Qc\x06\x80\x1C\xC3`\xE4\x1B\x81R`\x04\x81\x01\x91\x90\x91R`$\x81\x01\x92\x90\x92R`D\x82\x01Rs\xDA\x12\xF4PX\nL\xC4\x85\xC3\xB5\x01\xBA\xB7\xB0\xB3\xCB\xC3\xB3\x1B\x90ch\x01\xCC0\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x133W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13W\x91\x90a-3V[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x90Ua\x13\xB8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr$\xB7;0\xB64\xB2\x105\xB4\xBA:2\xB7\x1080\xB4\xB9`i\x1B`D\x82\x01R`d\x01a\x02\xF5V[`\xA0\x82\x01\x83\x90R` \x80\x82\x01Q\x82Q`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90\x82\x16\x10`\x80\x80\x85\x01\x91\x90\x91R`@\x80Q\x91\x82\x01\x81R`\x01T\x83\x16\x82Rs\xDA\x12\xF4PX\nL\xC4\x85\xC3\xB5\x01\xBA\xB7\xB0\xB3\xCB\xC3\xB3\x1B\x93\x82\x01\x93\x90\x93R\x91\x82\x01\x85\x90R\x82Q\x16``\x82\x01Ra\x14\"\x90a\x1BDV[`@\x82\x01R`\x01T`\x80\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x02,\r\x9F\x90a\x14OW\x82`@\x01Qa\x14QV[_[\x83`\x80\x01Qa\x14`W_a\x14fV[\x83`@\x01Q[0\x86`@Q` \x01a\x14x\x91\x90a-|V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14\xA6\x94\x93\x92\x91\x90a.\x18V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x14\xBDW__\xFD[PZ\xF1\x15\x80\x15a\x14\xCFW=__>=_\xFD[PP`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90UPPPPPV[___a\x14\xF7\x84_\x01Qa\x1A\x10V[`@Qc\x06\x80\x1C\xC3`\xE4\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\x04\x83\x01\x81\x90R\x90\x85\x16`$\x83\x01\x81\x90R`\x01`D\x84\x01R\x94\x97P\x92\x95P\x90\x93P\x91\x11\x15\x90s\xDA\x12\xF4PX\nL\xC4\x85\xC3\xB5\x01\xBA\xB7\xB0\xB3\xCB\xC3\xB3\x1B\x90ch\x01\xCC0\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15nW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x92\x91\x90a-3V[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x90Ua\x15\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr$\xB7;0\xB64\xB2\x105\xB4\xBA:2\xB7\x1080\xB4\xB9`i\x1B`D\x82\x01R`d\x01a\x02\xF5V[`@\x80Q`\x80\x80\x82\x01\x83R`\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83Rs\xDA\x12\xF4PX\nL\xC4\x85\xC3\xB5\x01\xBA\xB7\xB0\xB3\xCB\xC3\xB3\x1B` \x84\x01R\x90\x88\x01Q\x92\x82\x01\x92\x90\x92R\x90\x84\x16``\x82\x01Ra\x16D\x90a\x1B\xCFV[`\xA0\x86\x01R`\x01T`\x01`\x01`\xA0\x1B\x03\x16c\x02,\r\x9F\x82a\x16eW\x87a\x16gV[_[\x83a\x16rW_a\x16tV[\x88[0\x89`@Q` \x01a\x16\x86\x91\x90a-|V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x16\xB4\x94\x93\x92\x91\x90a.\x18V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x16\xCBW__\xFD[PZ\xF1\x15\x80\x15a\x16\xDDW=__>=_\xFD[PP`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90UPPPPPPPPV[___a\x17\x08\x84_\x01Qa\x1A\x10V[\x91\x94P\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x90\x83\x16\x10_a\x17Gs\xB1\xC0\xFA\x0Bx\x93 \x04Job<\xFE^\xBD\xA9V&\x02\xE3a\x17B\x86\x88\x87a\x1AOV[a\x1DPV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\r\xFE\x16\x81`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\x85W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xA9\x91\x90a-3V[P`\x01`\x01`\xA0\x1B\x03\x81\x16c\x12\x8A\xCB\x080\x84a\x17\xC4\x8Ba.DV[\x86a\x17\xEDWa\x17\xE8`\x01s\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D&a.^V[a\x17\xFDV[a\x17\xFDd\x01\0\x02v\xA3`\x01a.}V[\x8B`@Q` \x01a\x18\x0E\x91\x90a-|V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18=\x95\x94\x93\x92\x91\x90a.\x9CV[`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x18XW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18|\x91\x90a.\xE1V[PPPPPPPPPV[s\xD1/\x1C@!\x97\"C9\xD5\xA3$\xAC~\xF4\xDF]!B\xE9`\x01`\x01`\xA0\x1B\x03\x16c\\\xFF\xE9\xDE0s\xCAy\xDBKI\xF6\x08\xEFT\xA5\xCB\x81?\xBE\xD3\xA68{\xC6E\x85\x85`@Q` \x01a\x18\xD2\x91\x90a-|V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\r\xF6\x94\x93\x92\x91\x90a/\x03V[_\x80\x80\x80a\x19\x0E\x85\x82a\x1E6V[\x93Pa\x19\x1B\x85`\x14a\x1E\x9AV[a\xFF\xFF\x16\x91Pa\x19C`\x01a\x192`\x03`\x14a,\x14V[a\x19<\x91\x90a,\x14V[\x86\x90a\x1FDV[\x90Pa\x19g`\x01a\x19V`\x03`\x14a,\x14V[a\x19`\x91\x90a,\x14V[\x86\x90a\x1E6V[\x92P\x91\x93P\x91\x93V[_`\x01a\x19\x7F`\x03`\x14a,\x14V[a\x19\x89\x91\x90a,\x14V[`\x14`\x01a\x19\x98`\x03\x83a,\x14V[a\x19\xA2\x91\x90a,\x14V[a\x19\xAC\x91\x90a,\x14V[a\x19\xB6\x91\x90a,\x14V[\x82Q\x10\x15\x90P\x91\x90PV[``a\x1A\n`\x01a\x19\xD4`\x03`\x14a,\x14V[a\x19\xDE\x91\x90a,\x14V[`\x01a\x19\xEC`\x03`\x14a,\x14V[a\x19\xF6\x91\x90a,\x14V[\x84Qa\x1A\x02\x91\x90a,[V[\x84\x91\x90a\x1F\x9EV[\x92\x91PPV[_\x80\x80a\x1A\x1D\x84\x82a\x1E6V[\x92Pa\x1A*\x84`\x14a\x1E\x9AV[a\xFF\xFF\x16\x90Pa\x1AFa\x1A?`\x03`\x14a,\x14V[\x85\x90a\x1E6V[\x91P\x91\x93\x90\x92PV[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x11\x15a\x1A\x89W\x91\x92\x91[P`@\x80Q``\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R\x92\x90\x93\x16` \x83\x01Rb\xFF\xFF\xFF\x16\x91\x81\x01\x91\x90\x91R\x90V[_a\x1A\xC4\x83\x83a\x1DPV[\x90P3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14a\r\xC2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x1A[\x9D\x98[\x1AY\x08\x1C\x1B\xDB\xDB`\xA2\x1B`D\x82\x01R`d\x01a\x02\xF5V[_a\x1B\x1A`\x03`\x14a,\x14V[`\x14a\x19\xA2`\x03\x82a,\x14V[``a\x1A\na\x1B8`\x03`\x14a,\x14V[a\x19\xF6`\x03`\x14a,\x14V[\x80Q`@\x80\x83\x01Q``\x84\x01Q\x91Qcx\xA0Q\xAD`\xE1\x1B\x81R_\x93`\x01`\x01`\xA0\x1B\x03\x16\x92c\xF1@\xA3Z\x92a\x1B\x90\x92\x90\x91\x90`\x04\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xABW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\n\x91\x90a*JV[`@\x80Q`\xE0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R\x82_\x01Q`\x01`\x01`\xA0\x1B\x03\x16c9/7\xE9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1CEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Ci\x91\x90a/5V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xC0\x89\x01R\x16`\xA0\x87\x01R\x15\x15`\x80\x86\x01R``\x85\x01R`@\x84\x01R` \x83\x01R\x81R_a\x1C\xA2\x84\x83a \xAAV[` \x85\x01Q\x85Q`\x80\x85\x01Q`@Qc\xCCV\xB2\xC5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x90\x15\x15`$\x82\x01R\x92\x93P_\x92\x91\x16\x90c\xCCV\xB2\xC5\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\xFFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D#\x91\x90a*JV[\x90Pa\x1D1\x81a'\x10a,[V[a\x1D=\x83a'\x10a/\xA6V[a\x1DG\x91\x90a/\xBDV[\x95\x94PPPPPV[_\x81` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82_\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x10a\x1DuW__\xFD[\x81Q` \x80\x84\x01Q`@\x80\x86\x01Q\x81Q`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x81\x86\x01R\x94\x90\x92\x16\x84\x82\x01Rb\xFF\xFF\xFF\x90\x91\x16``\x80\x85\x01\x91\x90\x91R\x81Q\x80\x85\x03\x82\x01\x81R`\x80\x85\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\x01`\x01`\xF8\x1B\x03\x19`\xA0\x84\x01R\x90\x85\x90\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\xA1\x83\x01R`\xB5\x82\x01R\x7F\x1E\xEDC\xDC\xAA.\xFD\xE0g.\xB5qd\x92\0\xA2\x927\xB7\x95\x8E{\x0F\xBDR\xF7_\xA3[~\xC5,`\xD5\x82\x01R`\xF5\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x93\x92PPPV[_a\x1EB\x82`\x14a,\x14V[\x83Q\x10\x15a\x1E\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RttoAddress_outOfBounds`X\x1B`D\x82\x01R`d\x01a\x02\xF5V[P\x01` \x01Q`\x01``\x1B\x90\x04\x90V[_\x81a\x1E\xA7\x81`\x03a,\x14V[\x10\x15a\x1E\xE9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01RptoUint24_overflow`x\x1B`D\x82\x01R`d\x01a\x02\xF5V[a\x1E\xF4\x82`\x03a,\x14V[\x83Q\x10\x15a\x1F;W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01RstoUint24_outOfBounds``\x1B`D\x82\x01R`d\x01a\x02\xF5V[P\x01`\x03\x01Q\x90V[_a\x1FP\x82`\x01a,\x14V[\x83Q\x10\x15a\x1F\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01RqtoBool_outOfBounds`p\x1B`D\x82\x01R`d\x01a\x02\xF5V[P\x01`\x01\x01Q\x90V[``\x81a\x1F\xAC\x81`\x1Fa,\x14V[\x10\x15a\x1F\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rmslice_overflow`\x90\x1B`D\x82\x01R`d\x01a\x02\xF5V[a\x1F\xF5\x82\x84a,\x14V[\x84Q\x10\x15a 9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rpslice_outOfBounds`x\x1B`D\x82\x01R`d\x01a\x02\xF5V[``\x82\x15\x80\x15a WW`@Q\x91P_\x82R` \x82\x01`@Ra \xA1V[`@Q\x91P`\x1F\x84\x16\x80\x15` \x02\x81\x84\x01\x01\x85\x81\x01\x87\x83\x15` \x02\x84\x8B\x01\x01\x01[\x81\x83\x10\x15a \x90W\x80Q\x83R` \x92\x83\x01\x92\x01a xV[PP\x85\x84R`\x1F\x01`\x1F\x19\x16`@RP[P\x94\x93PPPPV[_\x81`\x80\x01Q\x15a\"\xC1W_a \xFA`@Q\x80`\xA0\x01`@R\x80\x85`@\x01Q\x81R` \x01\x85``\x01Q\x81R` \x01\x85`\x80\x01Q\x15\x15\x81R` \x01\x85_\x01Q\x81R` \x01\x85` \x01Q\x81RPa#5V[\x83Q`@\x85\x01Q\x91\x92P\x90a!\x17\x90g\r\xE0\xB6\xB3\xA7d\0\0a/\xA6V[a!!\x91\x90a/\xBDV[`@\x84\x01R` \x83\x01Q``\x84\x01Qa!B\x90g\r\xE0\xB6\xB3\xA7d\0\0a/\xA6V[a!L\x91\x90a/\xBDV[\x83``\x01\x81\x81RPP__\x84`\xC0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x86``\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14a!\x87W\x84``\x01Q\x85`@\x01Qa!\x92V[\x84`@\x01Q\x85``\x01Q[\x91P\x91P\x84`\xA0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x86``\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14a!\xE2W` \x85\x01Q`@\x87\x01Qa!\xD3\x90g\r\xE0\xB6\xB3\xA7d\0\0a/\xA6V[a!\xDD\x91\x90a/\xBDV[a\"\x05V[\x84Q`@\x87\x01Qa!\xFB\x90g\r\xE0\xB6\xB3\xA7d\0\0a/\xA6V[a\"\x05\x91\x90a/\xBDV[`@\x87\x01\x81\x90R_\x90a\"\x18\x90\x83a,[V[\x90P_\x83a\"^`@Q\x80`\xC0\x01`@R\x80\x85\x81R` \x01\x88\x81R` \x01\x87\x81R` \x01\x8A`\x80\x01Q\x15\x15\x81R` \x01\x8A_\x01Q\x81R` \x01\x8A` \x01Q\x81RPa$6V[a\"h\x91\x90a,[V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x87`\xA0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x89``\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14a\"\x9BW\x87Qa\"\xA1V[\x87` \x01Q[a\"\xAB\x90\x83a/\xA6V[a\"\xB5\x91\x90a/\xBDV[\x95PPPPPPa\x1A\nV[__\x83`\xC0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x85``\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14a\"\xF3W\x83``\x01Q\x84`@\x01Qa\"\xFEV[\x83`@\x01Q\x84``\x01Q[\x91P\x91P\x84`@\x01Q\x81a#\x12\x91\x90a,[V[\x82\x86`@\x01Qa#\"\x91\x90a/\xA6V[a#,\x91\x90a/\xBDV[\x92PPPa\x1A\nV[_\x81`@\x01Q\x15a$ W``\x82\x01Q\x82Q_\x91\x90a#\\\x90g\r\xE0\xB6\xB3\xA7d\0\0a/\xA6V[a#f\x91\x90a/\xBDV[\x90P_\x83`\x80\x01Q\x84` \x01Qg\r\xE0\xB6\xB3\xA7d\0\0a#\x86\x91\x90a/\xA6V[a#\x90\x91\x90a/\xBDV[\x90P_g\r\xE0\xB6\xB3\xA7d\0\0a#\xA6\x83\x85a/\xA6V[a#\xB0\x91\x90a/\xBDV[\x90P_g\r\xE0\xB6\xB3\xA7d\0\0a#\xC6\x84\x80a/\xA6V[a#\xD0\x91\x90a/\xBDV[g\r\xE0\xB6\xB3\xA7d\0\0a#\xE3\x86\x80a/\xA6V[a#\xED\x91\x90a/\xBDV[a#\xF7\x91\x90a,\x14V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0a$\x0C\x82\x84a/\xA6V[a$\x16\x91\x90a/\xBDV[\x96\x95PPPPPPV[` \x82\x01Q\x82Qa\x1A\n\x91\x90a/\xA6V[\x91\x90PV[_\x80[`\xFF\x81\x10\x15a%\xF2W_a$T\x84`@\x01Q\x85_\x01Qa& V[\x90P\x83` \x01Q\x81\x10\x15a%IW_a$t\x85`@\x01Q\x86_\x01Qa&\x9BV[\x82\x86` \x01Qa$\x84\x91\x90a,[V[a$\x96\x90g\r\xE0\xB6\xB3\xA7d\0\0a/\xA6V[a$\xA0\x91\x90a/\xBDV[\x90P\x80_\x03a%.W\x84` \x01Q\x82\x03a$\xBFWPPPP`@\x01Q\x90V[\x84` \x01Qa%\x14`@Q\x80`\xA0\x01`@R\x80\x88`@\x01Q`\x01a$\xE3\x91\x90a,\x14V[\x81R` \x01\x88_\x01Q\x81R` \x01\x88``\x01Q\x15\x15\x81R` \x01\x88`\x80\x01Q\x81R` \x01\x88`\xA0\x01Q\x81RPa#5V[\x11\x15a%*W`@\x85\x01Qa\x1DG\x90`\x01a,\x14V[P`\x01[\x80\x85`@\x01Qa%>\x91\x90a,\x14V[`@\x86\x01RPa%\xE9V[_a%[\x85`@\x01Q\x86_\x01Qa&\x9BV[` \x86\x01Qa%j\x90\x84a,[V[a%|\x90g\r\xE0\xB6\xB3\xA7d\0\0a/\xA6V[a%\x86\x91\x90a/\xBDV[\x90P\x80_\x03a%\xD2W\x84` \x01Q\x82\x14\x80a%\xBEWP\x84` \x01Qa%\xBC`\x01\x87`@\x01Qa%\xB5\x91\x90a,[V[\x87Qa& V[\x10[\x15a%\xCEWPPPP`@\x01Q\x90V[P`\x01[\x80\x85`@\x01Qa%\xE2\x91\x90a,[V[`@\x86\x01RP[P`\x01\x01a$9V[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x02`$\x82\x01Ra\x04/`\xF3\x1B`D\x82\x01R`d\x01a\x02\xF5V[_\x80g\r\xE0\xB6\xB3\xA7d\0\0a&5\x84\x86a/\xA6V[a&?\x91\x90a/\xBDV[\x90P_g\r\xE0\xB6\xB3\xA7d\0\0a&U\x85\x80a/\xA6V[a&_\x91\x90a/\xBDV[g\r\xE0\xB6\xB3\xA7d\0\0a&r\x87\x80a/\xA6V[a&|\x91\x90a/\xBDV[a&\x86\x91\x90a,\x14V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0a\x1D=\x82\x84a/\xA6V[_g\r\xE0\xB6\xB3\xA7d\0\0\x83\x81a&\xB1\x82\x80a/\xA6V[a&\xBB\x91\x90a/\xBDV[a&\xC5\x91\x90a/\xA6V[a&\xCF\x91\x90a/\xBDV[g\r\xE0\xB6\xB3\xA7d\0\0\x80a&\xE3\x85\x80a/\xA6V[a&\xED\x91\x90a/\xBDV[a&\xF8\x86`\x03a/\xA6V[a'\x02\x91\x90a/\xA6V[a'\x0C\x91\x90a/\xBDV[a'\x16\x91\x90a,\x14V[\x93\x92PPPV[`@Q\x80``\x01`@R\x80_\x81R` \x01_\x81R` \x01a'z`@\x80Qa\x01\0\x81\x01\x82R``\x80\x82R_` \x83\x01\x81\x90R\x92\x82\x01\x83\x90R\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x91\x90\x91R\x90V[\x90R\x90V[`@\x80Q`\xA0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91R`\x80\x81\x01a'z`@\x80Qa\x01\0\x81\x01\x82R``\x80\x82R_` \x83\x01\x81\x90R\x92\x82\x01\x83\x90R\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x91\x90\x91R\x90V[__\x83`\x1F\x84\x01\x12a'\xFCW__\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a(\x13W__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a(*W__\xFD[\x92P\x92\x90PV[____``\x85\x87\x03\x12\x15a(DW__\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a(hW__\xFD[a(t\x87\x82\x88\x01a'\xECV[\x95\x98\x94\x97P\x95PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a(\x94W__\xFD[PV[\x805a$1\x81a(\x80V[________`\xC0\x89\x8B\x03\x12\x15a(\xB9W__\xFD[\x885a(\xC4\x81a(\x80V[\x97P` \x89\x015a(\xD4\x81a(\x80V[\x96P`@\x89\x015a(\xE4\x81a(\x80V[\x95P``\x89\x015\x94P`\x80\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a)\x06W__\xFD[a)\x12\x8B\x82\x8C\x01a'\xECV[\x90\x95P\x93PP`\xA0\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a)1W__\xFD[a)=\x8B\x82\x8C\x01a'\xECV[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[_` \x82\x84\x03\x12\x15a)aW__\xFD[\x815a'\x16\x81a(\x80V[\x80\x15\x15\x81\x14a(\x94W__\xFD[\x805a$1\x81a)lV[__`@\x83\x85\x03\x12\x15a)\x95W__\xFD[\x825a)\xA0\x81a(\x80V[\x91P` \x83\x015a)\xB0\x81a)lV[\x80\x91PP\x92P\x92\x90PV[__`@\x83\x85\x03\x12\x15a)\xCCW__\xFD[\x825a)\xD7\x81a(\x80V[\x94` \x93\x90\x93\x015\x93PPPV[_____`\x80\x86\x88\x03\x12\x15a)\xF9W__\xFD[\x855a*\x04\x81a(\x80V[\x94P` \x86\x015\x93P`@\x86\x015\x92P``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*-W__\xFD[a*9\x88\x82\x89\x01a'\xECV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[_` \x82\x84\x03\x12\x15a*ZW__\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Qa\x01\0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a*\x99Wa*\x99a*aV[`@R\x90V[_\x82`\x1F\x83\x01\x12a*\xAEW__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*\xC8Wa*\xC8a*aV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a*\xF7Wa*\xF7a*aV[`@R\x81\x81R\x83\x82\x01` \x01\x85\x10\x15a+\x0EW__\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_` \x82\x84\x03\x12\x15a+:W__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+PW__\xFD[\x82\x01a\x01\0\x81\x85\x03\x12\x15a+bW__\xFD[a+ja*uV[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+\x80W__\xFD[a+\x8C\x86\x82\x85\x01a*\x9FV[\x82RPa+\x9B` \x83\x01a(\x97V[` \x82\x01Ra+\xAC`@\x83\x01a(\x97V[`@\x82\x01Ra+\xBD``\x83\x01a(\x97V[``\x82\x01R`\x80\x82\x81\x015\x90\x82\x01R`\xA0\x80\x83\x015\x90\x82\x01Ra+\xE2`\xC0\x83\x01a)yV[`\xC0\x82\x01Ra+\xF3`\xE0\x83\x01a)yV[`\xE0\x82\x01R\x94\x93PPPPV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x1A\nWa\x1A\na,\0V[`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x81R\x93\x85\x16` \x85\x01R\x91\x90\x93\x16`@\x83\x01R``\x82\x01\x92\x90\x92R\x90\x15\x15`\x80\x82\x01R`\xA0\x01\x90V[\x81\x81\x03\x81\x81\x11\x15a\x1A\nWa\x1A\na,\0V[_` \x82\x84\x03\x12\x15a,~W__\xFD[\x81Qa'\x16\x81a)lV[` \x80\x82R`/\x90\x82\x01R\x7FOnly owner or liquidator can cal`@\x82\x01Rn6\x10:44\xB9\x903:\xB71\xBA4\xB7\xB7`\x89\x1B``\x82\x01R`\x80\x01\x90V[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[\x81\x81\x03_\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a-\x06Wa-\x06a,\0V[P\x92\x91PPV[` \x80\x82R`\x0C\x90\x82\x01Rk\x15S\x90UU\x12\x13\xD4\x92V\x91Q`\xA2\x1B`@\x82\x01R``\x01\x90V[_` \x82\x84\x03\x12\x15a-CW__\xFD[\x81Qa'\x16\x81a(\x80V[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_\x82Qa\x01\0` \x84\x01Ra-\x99a\x01 \x84\x01\x82a-NV[\x90P`\x01\x80`\xA0\x1B\x03` \x85\x01Q\x16`@\x84\x01R`\x01\x80`\xA0\x1B\x03`@\x85\x01Q\x16``\x84\x01R``\x84\x01Qa-\xD9`\x80\x85\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\x80\x84\x01Q`\xA0\x84\x01R`\xA0\x84\x01Q`\xC0\x84\x01R`\xC0\x84\x01Qa.\x01`\xE0\x85\x01\x82\x15\x15\x90RV[P`\xE0\x84\x01Q\x80\x15\x15a\x01\0\x85\x01RP\x93\x92PPPV[\x84\x81R\x83` \x82\x01R`\x01\x80`\xA0\x1B\x03\x83\x16`@\x82\x01R`\x80``\x82\x01R_a$\x16`\x80\x83\x01\x84a-NV[_`\x01`\xFF\x1B\x82\x01a.XWa.Xa,\0V[P_\x03\x90V[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x1A\nWa\x1A\na,\0V[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x1A\nWa\x1A\na,\0V[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R\x85\x15\x15` \x83\x01R`@\x82\x01\x85\x90R\x83\x16``\x82\x01R`\xA0`\x80\x82\x01\x81\x90R_\x90a.\xD6\x90\x83\x01\x84a-NV[\x97\x96PPPPPPPV[__`@\x83\x85\x03\x12\x15a.\xF2W__\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R\x84\x16` \x82\x01R`@\x81\x01\x83\x90R`\x80``\x82\x01\x81\x90R_\x90a$\x16\x90\x83\x01\x84a-NV[_______`\xE0\x88\x8A\x03\x12\x15a/KW__\xFD[\x87Q` \x89\x01Q`@\x8A\x01Q``\x8B\x01Q`\x80\x8C\x01Q\x93\x9AP\x91\x98P\x96P\x94Pa/t\x81a)lV[`\xA0\x89\x01Q\x90\x93Pa/\x85\x81a(\x80V[`\xC0\x89\x01Q\x90\x92Pa/\x96\x81a(\x80V[\x80\x91PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x1A\nWa\x1A\na,\0V[_\x82a/\xD7WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V\xFE\xA2dipfsX\"\x12 \x90\xDF%?\xAA\xD0{\xE9\x8F\x88f \x8C\xEE\xE6\xC0\xAC\xB3\x9BJ\xF3\xBD\x9DY\x94\x9E\x01\xBEx\x8APUdsolcC\0\x08\x1C\x003";
    /// The bytecode of the contract.
    pub static LIQUIDATOR_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\xF0W_5`\xE0\x1C\x80cR\x9A5o\x11a\0\x93W\x80c\x9A{\xFFy\x11a\0cW\x80c\x9A{\xFFy\x14a\x02AW\x80c\xEA\x93\x9F\xA6\x14a\x02TW\x80c\xF2\xFD\xE3\x8B\x14a\x02oW\x80c\xFAF\x1E3\x14a\x02\x82W__\xFD[\x80cR\x9A5o\x14a\x01\xDFW\x80cW\x05\xAEC\x14a\x02\x01W\x80cp\xC2j^\x14a\x02\x14W\x80c\x8D\xA5\xCB[\x14a\x02/W__\xFD[\x80c.C\xC9a\x11a\0\xCEW\x80c.C\xC9a\x14a\x01jW\x80c>\r\x95Z\x14a\x01\x9CW\x80cBL&[\x14a\x01\xB7W\x80cDS\xA3t\x14a\x01\xCCW__\xFD[\x80c\x08\xBE\xA1'\x14a\0\xF4W\x80c\r\xB7\xB0(\x14a\x01,W\x80c\x16\xF0\x11[\x14a\x01OW[__\xFD[a\x01\x0Fs\xDA\x12\xF4PX\nL\xC4\x85\xC3\xB5\x01\xBA\xB7\xB0\xB3\xCB\xC3\xB3\x1B\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01?a\x01:6`\x04a(1V[a\x02\x95V[`@Q\x90\x15\x15\x81R` \x01a\x01#V[a\x01\x0Fs\xCE\xCC\xE0\xEB\x9D\xD2\xEFy\x96\xE0\x1E%\xDDp\xE4a\xF9\x18\xA1K\x81V[a\x01}a\x01x6`\x04a(\xA2V[a\x06|V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R\x01a\x01#V[a\x01\x0Fs\xB1\xC0\xFA\x0Bx\x93 \x04Job<\xFE^\xBD\xA9V&\x02\xE3\x81V[a\x01\xCAa\x01\xC56`\x04a)QV[a\x0C\x0EV[\0[a\x01\xCAa\x01\xDA6`\x04a)\x84V[a\x0C\xD6V[a\x01?a\x01\xED6`\x04a)QV[`\x02` R_\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x01\xCAa\x02\x0F6`\x04a)\xBBV[a\raV[a\x01\x0Fs\xCAy\xDBKI\xF6\x08\xEFT\xA5\xCB\x81?\xBE\xD3\xA68{\xC6E\x81V[_Ta\x01\x0F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xCAa\x02O6`\x04a)\xE5V[a\x0E6V[a\x01\x0Fs\xD1/\x1C@!\x97\"C9\xD5\xA3$\xAC~\xF4\xDF]!B\xE9\x81V[a\x01\xCAa\x02}6`\x04a)QV[a\x10oV[a\x01\xCAa\x02\x906`\x04a(1V[a\x10\xE2V[_3s\xD1/\x1C@!\x97\"C9\xD5\xA3$\xAC~\xF4\xDF]!B\xE9\x14a\x02\xFEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FCaller must be flash minter\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x85\x90s\xCAy\xDBKI\xF6\x08\xEFT\xA5\xCB\x81?\xBE\xD3\xA68{\xC6E\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03MW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03q\x91\x90a*JV[\x10\x15a\x03\xBFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FInvalid balance for flash loan\0\0`D\x82\x01R`d\x01a\x02\xF5V[a\x03\xC7a'\x1DV[a\x03\xD3\x83\x85\x01\x85a+*V[`@\x80\x83\x01\x82\x90R\x01Q`\x01`\x01`\xA0\x1B\x03\x16s\xCAy\xDBKI\xF6\x08\xEFT\xA5\xCB\x81?\xBE\xD3\xA68{\xC6E\x14a\x04HW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FDebt asset must be USDXL\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\xF5V[a\x04R\x85\x87a,\x14V[\x81R`@\x80\x82\x01Q` \x01Q\x90Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xA0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xC4\x91\x90a*JV[` \x80\x83\x01\x91\x90\x91R`@\x80\x83\x01Q\x91\x82\x01Q\x82\x82\x01Q``\x84\x01Q`\x80\x90\x94\x01Q\x92Qb\xA7\x18\xA9`\xE0\x1B\x81Rs\xCE\xCC\xE0\xEB\x9D\xD2\xEFy\x96\xE0\x1E%\xDDp\xE4a\xF9\x18\xA1K\x94b\xA7\x18\xA9\x94a\x05\x1C\x94\x93\x92_\x90`\x04\x01a,'V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x053W__\xFD[PZ\xF1\x15\x80\x15a\x05EW=__>=_\xFD[PPPP` \x81\x81\x01Q`@\x80\x84\x01Q\x90\x92\x01Q\x91Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x9BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xBF\x91\x90a*JV[a\x05\xC9\x91\x90a,[V[` \x82\x01\x81\x90R`@\x82\x01Qa\x05\xDF\x91\x90a\x12lV[\x80Q`@Qc\t^\xA7\xB3`\xE0\x1B\x81Rs\xD1/\x1C@!\x97\"C9\xD5\xA3$\xAC~\xF4\xDF]!B\xE9`\x04\x82\x01R`$\x81\x01\x91\x90\x91Rs\xCAy\xDBKI\xF6\x08\xEFT\xA5\xCB\x81?\xBE\xD3\xA68{\xC6E\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x06KW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06o\x91\x90a,nV[P`\x01\x96\x95PPPPPPV[_\x80T\x81\x90`\x01`\x01`\xA0\x1B\x03\x163\x14\x80a\x06\xA5WP3_\x90\x81R`\x02` R`@\x90 T`\xFF\x16[a\x06\xC1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\xF5\x90a,\x89V[`@Qi\x06\xB6\x97GFV\xE77v\x17`\xB4\x1B` \x82\x01R_\x90`*\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x85\x85`@Q` \x01a\x07\x05\x92\x91\x90a,\xD8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x03a\x08\xACW`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x8C\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07bW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x86\x91\x90a*JV[\x90Pa\x08/\x88`@Q\x80a\x01\0\x01`@R\x80\x8A\x8A\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x8E`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8D`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8C`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8B\x81R` \x01_\x81R` \x01`\x01\x15\x15\x81R` \x01`\x01\x15\x15\x81RPa\x14\xE8V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x8B\x93P\x81\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90cp\xA0\x821\x90`$\x01[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08wW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x9B\x91\x90a*JV[a\x08\xA5\x91\x90a,\xE7V[\x91Pa\x0C\0V[`@Qh\x06\x87\x97\x06W'7v\x17`\xBC\x1B` \x82\x01R`)\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x85\x85`@Q` \x01a\x08\xED\x92\x91\x90a,\xD8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x03a\n\x17W`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x8C\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\tJW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tn\x91\x90a*JV[\x90Pa\x08/\x88`@Q\x80a\x01\0\x01`@R\x80\x8A\x8A\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x8E`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8D`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8C`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8B\x81R` \x01_\x81R` \x01`\x01\x15\x15\x81R` \x01`\x01\x15\x15\x81RPa\x16\xF9V[`@Qo:\xB9\xB2<6#60\xB9\xB4&\xB4\xB7:2\xB9`\x81\x1B` \x82\x01R`0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x85\x85`@Q` \x01a\n_\x92\x91\x90a,\xD8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x03a\x0B\xB8W`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x8B\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xBCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xE0\x91\x90a*JV[\x90Pa\x0B\x87\x88`@Q\x80a\x01\0\x01`@R\x80\x8A\x8A\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x8E`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8D`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8C`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8B\x81R` \x01_\x81R` \x01_\x15\x15\x81R` \x01_\x15\x15\x81RPa\x18\x87V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x8A\x93P\x81\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90cp\xA0\x821\x90`$\x01a\x08\\V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FInvalid liquidation path\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\xF5V[P\x98P\x98\x96PPPPPPPV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14\x80a\x0C4WP3_\x90\x81R`\x02` R`@\x90 T`\xFF\x16[a\x0CPW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\xF5\x90a,\x89V[`@Qc\t^\xA7\xB3`\xE0\x1B\x81Rs\xCE\xCC\xE0\xEB\x9D\xD2\xEFy\x96\xE0\x1E%\xDDp\xE4a\xF9\x18\xA1K`\x04\x82\x01R_\x19`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0C\xAEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xD2\x91\x90a,nV[PPV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0C\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\xF5\x90a-\rV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x81\x81R`\x02` R`@\x90\x81\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x7F\x81\xE0 4At\x97,Y\xF6\xC1\x1A\x8Fl\x90\xB1A\x86b\x14\xE3\xD9\xB5D\xD00\xF0\xB52\xF5\xA1\x0F\x90a\rU\x90\x84\x15\x15\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2PPV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\r\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\xF5\x90a-\rV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\r\xC7W`@Q3\x90\x82\x15a\x08\xFC\x02\x90\x83\x90_\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\r\xC2W=__>=_\xFD[PPPV[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0E\x12W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xC2\x91\x90a,nV[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0E\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7Fmsg.sender != activeKittenPair\0\0`D\x82\x01R`d\x01a\x02\xF5V[a\x0E\x98a'\x7FV[a\x0E\xA4\x82\x84\x01\x84a+*V[`\x80\x82\x01\x81\x90RQa\x0E\xB5\x90a\x19\0V[\x15\x15``\x85\x01Rb\xFF\xFF\xFF\x16`@\x84\x01R`\x01`\x01`\xA0\x1B\x03\x90\x81\x16` \x84\x01R\x16\x81R`\x80\x81\x01Q`\xC0\x01Q\x15a\x0FjW`\x80\x80\x82\x01Q` \x81\x01Q`@\x80\x83\x01Q``\x84\x01Q\x93\x90\x94\x01Q\x90Qb\xA7\x18\xA9`\xE0\x1B\x81Rs\xCE\xCC\xE0\xEB\x9D\xD2\xEFy\x96\xE0\x1E%\xDDp\xE4a\xF9\x18\xA1K\x94b\xA7\x18\xA9\x94a\x0F<\x94\x93\x91\x92\x90\x91\x90_\x90`\x04\x01a,'V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0FSW__\xFD[PZ\xF1\x15\x80\x15a\x0FeW=__>=_\xFD[PPPP[`\x80\x81\x01QQa\x0Fy\x90a\x19pV[\x15a\x0F\xCCW`\x80\x81\x01QQa\x0F\x8D\x90a\x19\xC1V[`\x80\x82\x01\x80Q\x91\x90\x91RQ`\xE0\x01Q\x15a\x0F\xB9W`\x80\x81\x01Q`\xA0\x81\x01Qa\x0F\xB4\x91a\x14\xE8V[a\x0F\xCCV[`\x80\x81\x01Q`\xA0\x81\x01Qa\x0F\xCC\x91a\x12lV[\x80`\x80\x01Q`\xE0\x01Q\x15a\x0F\xEBW` \x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R[\x80Q`\x80\x82\x01Q`\xA0\x01Q`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x10BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10f\x91\x90a,nV[PPPPPPPV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x10\x98W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\xF5\x90a-\rV[_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x82U`@Q\x90\x913\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PV[_a\x10\xEF\x82\x84\x01\x84a+*V[\x90P___a\x11\0\x84_\x01Qa\x1A\x10V[\x92P\x92P\x92Pa\x11.s\xB1\xC0\xFA\x0Bx\x93 \x04Job<\xFE^\xBD\xA9V&\x02\xE3a\x11)\x85\x85\x85a\x1AOV[a\x1A\xB9V[\x83`\xC0\x01Q\x15a\x11\xB3W` \x84\x01Q`@\x80\x86\x01Q``\x87\x01Q`\x80\x88\x01Q\x92Qb\xA7\x18\xA9`\xE0\x1B\x81Rs\xCE\xCC\xE0\xEB\x9D\xD2\xEFy\x96\xE0\x1E%\xDDp\xE4a\xF9\x18\xA1K\x94b\xA7\x18\xA9\x94a\x11\x85\x94\x91\x93\x91\x92_\x90`\x04\x01a,'V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x11\x9CW__\xFD[PZ\xF1\x15\x80\x15a\x11\xAEW=__>=_\xFD[PPPP[__\x89\x13a\x11\xC1W\x87a\x11\xC3V[\x88[\x90Pa\x11\xD1\x85_\x01Qa\x1B\rV[\x15a\x11\xEDW\x84Qa\x11\xE1\x90a\x1B'V[\x85Ra\x11\xED\x81\x86a\x16\xF9V[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R\x92\x93P\x83\x92`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x12<W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12`\x91\x90a,nV[PPPPPPPPPPV[`@\x80Q`\xC0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91R\x81Qa\x12\xA8\x90a\x19\0V[\x15\x15`\xA0\x85\x01\x81\x90Rb\xFF\xFF\xFF\x91\x90\x91\x16``\x85\x01R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16` \x85\x01\x81\x90R\x92\x90\x91\x16\x80\x84R`@Qc\x06\x80\x1C\xC3`\xE4\x1B\x81R`\x04\x81\x01\x91\x90\x91R`$\x81\x01\x92\x90\x92R`D\x82\x01Rs\xDA\x12\xF4PX\nL\xC4\x85\xC3\xB5\x01\xBA\xB7\xB0\xB3\xCB\xC3\xB3\x1B\x90ch\x01\xCC0\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x133W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13W\x91\x90a-3V[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x90Ua\x13\xB8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr$\xB7;0\xB64\xB2\x105\xB4\xBA:2\xB7\x1080\xB4\xB9`i\x1B`D\x82\x01R`d\x01a\x02\xF5V[`\xA0\x82\x01\x83\x90R` \x80\x82\x01Q\x82Q`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90\x82\x16\x10`\x80\x80\x85\x01\x91\x90\x91R`@\x80Q\x91\x82\x01\x81R`\x01T\x83\x16\x82Rs\xDA\x12\xF4PX\nL\xC4\x85\xC3\xB5\x01\xBA\xB7\xB0\xB3\xCB\xC3\xB3\x1B\x93\x82\x01\x93\x90\x93R\x91\x82\x01\x85\x90R\x82Q\x16``\x82\x01Ra\x14\"\x90a\x1BDV[`@\x82\x01R`\x01T`\x80\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x02,\r\x9F\x90a\x14OW\x82`@\x01Qa\x14QV[_[\x83`\x80\x01Qa\x14`W_a\x14fV[\x83`@\x01Q[0\x86`@Q` \x01a\x14x\x91\x90a-|V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14\xA6\x94\x93\x92\x91\x90a.\x18V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x14\xBDW__\xFD[PZ\xF1\x15\x80\x15a\x14\xCFW=__>=_\xFD[PP`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90UPPPPPV[___a\x14\xF7\x84_\x01Qa\x1A\x10V[`@Qc\x06\x80\x1C\xC3`\xE4\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\x04\x83\x01\x81\x90R\x90\x85\x16`$\x83\x01\x81\x90R`\x01`D\x84\x01R\x94\x97P\x92\x95P\x90\x93P\x91\x11\x15\x90s\xDA\x12\xF4PX\nL\xC4\x85\xC3\xB5\x01\xBA\xB7\xB0\xB3\xCB\xC3\xB3\x1B\x90ch\x01\xCC0\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15nW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x92\x91\x90a-3V[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x90Ua\x15\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr$\xB7;0\xB64\xB2\x105\xB4\xBA:2\xB7\x1080\xB4\xB9`i\x1B`D\x82\x01R`d\x01a\x02\xF5V[`@\x80Q`\x80\x80\x82\x01\x83R`\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83Rs\xDA\x12\xF4PX\nL\xC4\x85\xC3\xB5\x01\xBA\xB7\xB0\xB3\xCB\xC3\xB3\x1B` \x84\x01R\x90\x88\x01Q\x92\x82\x01\x92\x90\x92R\x90\x84\x16``\x82\x01Ra\x16D\x90a\x1B\xCFV[`\xA0\x86\x01R`\x01T`\x01`\x01`\xA0\x1B\x03\x16c\x02,\r\x9F\x82a\x16eW\x87a\x16gV[_[\x83a\x16rW_a\x16tV[\x88[0\x89`@Q` \x01a\x16\x86\x91\x90a-|V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x16\xB4\x94\x93\x92\x91\x90a.\x18V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x16\xCBW__\xFD[PZ\xF1\x15\x80\x15a\x16\xDDW=__>=_\xFD[PP`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90UPPPPPPPPV[___a\x17\x08\x84_\x01Qa\x1A\x10V[\x91\x94P\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x90\x83\x16\x10_a\x17Gs\xB1\xC0\xFA\x0Bx\x93 \x04Job<\xFE^\xBD\xA9V&\x02\xE3a\x17B\x86\x88\x87a\x1AOV[a\x1DPV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\r\xFE\x16\x81`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\x85W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xA9\x91\x90a-3V[P`\x01`\x01`\xA0\x1B\x03\x81\x16c\x12\x8A\xCB\x080\x84a\x17\xC4\x8Ba.DV[\x86a\x17\xEDWa\x17\xE8`\x01s\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D&a.^V[a\x17\xFDV[a\x17\xFDd\x01\0\x02v\xA3`\x01a.}V[\x8B`@Q` \x01a\x18\x0E\x91\x90a-|V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18=\x95\x94\x93\x92\x91\x90a.\x9CV[`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x18XW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18|\x91\x90a.\xE1V[PPPPPPPPPV[s\xD1/\x1C@!\x97\"C9\xD5\xA3$\xAC~\xF4\xDF]!B\xE9`\x01`\x01`\xA0\x1B\x03\x16c\\\xFF\xE9\xDE0s\xCAy\xDBKI\xF6\x08\xEFT\xA5\xCB\x81?\xBE\xD3\xA68{\xC6E\x85\x85`@Q` \x01a\x18\xD2\x91\x90a-|V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\r\xF6\x94\x93\x92\x91\x90a/\x03V[_\x80\x80\x80a\x19\x0E\x85\x82a\x1E6V[\x93Pa\x19\x1B\x85`\x14a\x1E\x9AV[a\xFF\xFF\x16\x91Pa\x19C`\x01a\x192`\x03`\x14a,\x14V[a\x19<\x91\x90a,\x14V[\x86\x90a\x1FDV[\x90Pa\x19g`\x01a\x19V`\x03`\x14a,\x14V[a\x19`\x91\x90a,\x14V[\x86\x90a\x1E6V[\x92P\x91\x93P\x91\x93V[_`\x01a\x19\x7F`\x03`\x14a,\x14V[a\x19\x89\x91\x90a,\x14V[`\x14`\x01a\x19\x98`\x03\x83a,\x14V[a\x19\xA2\x91\x90a,\x14V[a\x19\xAC\x91\x90a,\x14V[a\x19\xB6\x91\x90a,\x14V[\x82Q\x10\x15\x90P\x91\x90PV[``a\x1A\n`\x01a\x19\xD4`\x03`\x14a,\x14V[a\x19\xDE\x91\x90a,\x14V[`\x01a\x19\xEC`\x03`\x14a,\x14V[a\x19\xF6\x91\x90a,\x14V[\x84Qa\x1A\x02\x91\x90a,[V[\x84\x91\x90a\x1F\x9EV[\x92\x91PPV[_\x80\x80a\x1A\x1D\x84\x82a\x1E6V[\x92Pa\x1A*\x84`\x14a\x1E\x9AV[a\xFF\xFF\x16\x90Pa\x1AFa\x1A?`\x03`\x14a,\x14V[\x85\x90a\x1E6V[\x91P\x91\x93\x90\x92PV[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x11\x15a\x1A\x89W\x91\x92\x91[P`@\x80Q``\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R\x92\x90\x93\x16` \x83\x01Rb\xFF\xFF\xFF\x16\x91\x81\x01\x91\x90\x91R\x90V[_a\x1A\xC4\x83\x83a\x1DPV[\x90P3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14a\r\xC2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x1A[\x9D\x98[\x1AY\x08\x1C\x1B\xDB\xDB`\xA2\x1B`D\x82\x01R`d\x01a\x02\xF5V[_a\x1B\x1A`\x03`\x14a,\x14V[`\x14a\x19\xA2`\x03\x82a,\x14V[``a\x1A\na\x1B8`\x03`\x14a,\x14V[a\x19\xF6`\x03`\x14a,\x14V[\x80Q`@\x80\x83\x01Q``\x84\x01Q\x91Qcx\xA0Q\xAD`\xE1\x1B\x81R_\x93`\x01`\x01`\xA0\x1B\x03\x16\x92c\xF1@\xA3Z\x92a\x1B\x90\x92\x90\x91\x90`\x04\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xABW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\n\x91\x90a*JV[`@\x80Q`\xE0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R\x82_\x01Q`\x01`\x01`\xA0\x1B\x03\x16c9/7\xE9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1CEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Ci\x91\x90a/5V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xC0\x89\x01R\x16`\xA0\x87\x01R\x15\x15`\x80\x86\x01R``\x85\x01R`@\x84\x01R` \x83\x01R\x81R_a\x1C\xA2\x84\x83a \xAAV[` \x85\x01Q\x85Q`\x80\x85\x01Q`@Qc\xCCV\xB2\xC5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x90\x15\x15`$\x82\x01R\x92\x93P_\x92\x91\x16\x90c\xCCV\xB2\xC5\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\xFFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D#\x91\x90a*JV[\x90Pa\x1D1\x81a'\x10a,[V[a\x1D=\x83a'\x10a/\xA6V[a\x1DG\x91\x90a/\xBDV[\x95\x94PPPPPV[_\x81` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82_\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x10a\x1DuW__\xFD[\x81Q` \x80\x84\x01Q`@\x80\x86\x01Q\x81Q`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x81\x86\x01R\x94\x90\x92\x16\x84\x82\x01Rb\xFF\xFF\xFF\x90\x91\x16``\x80\x85\x01\x91\x90\x91R\x81Q\x80\x85\x03\x82\x01\x81R`\x80\x85\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\x01`\x01`\xF8\x1B\x03\x19`\xA0\x84\x01R\x90\x85\x90\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\xA1\x83\x01R`\xB5\x82\x01R\x7F\x1E\xEDC\xDC\xAA.\xFD\xE0g.\xB5qd\x92\0\xA2\x927\xB7\x95\x8E{\x0F\xBDR\xF7_\xA3[~\xC5,`\xD5\x82\x01R`\xF5\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x93\x92PPPV[_a\x1EB\x82`\x14a,\x14V[\x83Q\x10\x15a\x1E\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RttoAddress_outOfBounds`X\x1B`D\x82\x01R`d\x01a\x02\xF5V[P\x01` \x01Q`\x01``\x1B\x90\x04\x90V[_\x81a\x1E\xA7\x81`\x03a,\x14V[\x10\x15a\x1E\xE9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01RptoUint24_overflow`x\x1B`D\x82\x01R`d\x01a\x02\xF5V[a\x1E\xF4\x82`\x03a,\x14V[\x83Q\x10\x15a\x1F;W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01RstoUint24_outOfBounds``\x1B`D\x82\x01R`d\x01a\x02\xF5V[P\x01`\x03\x01Q\x90V[_a\x1FP\x82`\x01a,\x14V[\x83Q\x10\x15a\x1F\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01RqtoBool_outOfBounds`p\x1B`D\x82\x01R`d\x01a\x02\xF5V[P\x01`\x01\x01Q\x90V[``\x81a\x1F\xAC\x81`\x1Fa,\x14V[\x10\x15a\x1F\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rmslice_overflow`\x90\x1B`D\x82\x01R`d\x01a\x02\xF5V[a\x1F\xF5\x82\x84a,\x14V[\x84Q\x10\x15a 9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rpslice_outOfBounds`x\x1B`D\x82\x01R`d\x01a\x02\xF5V[``\x82\x15\x80\x15a WW`@Q\x91P_\x82R` \x82\x01`@Ra \xA1V[`@Q\x91P`\x1F\x84\x16\x80\x15` \x02\x81\x84\x01\x01\x85\x81\x01\x87\x83\x15` \x02\x84\x8B\x01\x01\x01[\x81\x83\x10\x15a \x90W\x80Q\x83R` \x92\x83\x01\x92\x01a xV[PP\x85\x84R`\x1F\x01`\x1F\x19\x16`@RP[P\x94\x93PPPPV[_\x81`\x80\x01Q\x15a\"\xC1W_a \xFA`@Q\x80`\xA0\x01`@R\x80\x85`@\x01Q\x81R` \x01\x85``\x01Q\x81R` \x01\x85`\x80\x01Q\x15\x15\x81R` \x01\x85_\x01Q\x81R` \x01\x85` \x01Q\x81RPa#5V[\x83Q`@\x85\x01Q\x91\x92P\x90a!\x17\x90g\r\xE0\xB6\xB3\xA7d\0\0a/\xA6V[a!!\x91\x90a/\xBDV[`@\x84\x01R` \x83\x01Q``\x84\x01Qa!B\x90g\r\xE0\xB6\xB3\xA7d\0\0a/\xA6V[a!L\x91\x90a/\xBDV[\x83``\x01\x81\x81RPP__\x84`\xC0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x86``\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14a!\x87W\x84``\x01Q\x85`@\x01Qa!\x92V[\x84`@\x01Q\x85``\x01Q[\x91P\x91P\x84`\xA0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x86``\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14a!\xE2W` \x85\x01Q`@\x87\x01Qa!\xD3\x90g\r\xE0\xB6\xB3\xA7d\0\0a/\xA6V[a!\xDD\x91\x90a/\xBDV[a\"\x05V[\x84Q`@\x87\x01Qa!\xFB\x90g\r\xE0\xB6\xB3\xA7d\0\0a/\xA6V[a\"\x05\x91\x90a/\xBDV[`@\x87\x01\x81\x90R_\x90a\"\x18\x90\x83a,[V[\x90P_\x83a\"^`@Q\x80`\xC0\x01`@R\x80\x85\x81R` \x01\x88\x81R` \x01\x87\x81R` \x01\x8A`\x80\x01Q\x15\x15\x81R` \x01\x8A_\x01Q\x81R` \x01\x8A` \x01Q\x81RPa$6V[a\"h\x91\x90a,[V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x87`\xA0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x89``\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14a\"\x9BW\x87Qa\"\xA1V[\x87` \x01Q[a\"\xAB\x90\x83a/\xA6V[a\"\xB5\x91\x90a/\xBDV[\x95PPPPPPa\x1A\nV[__\x83`\xC0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x85``\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14a\"\xF3W\x83``\x01Q\x84`@\x01Qa\"\xFEV[\x83`@\x01Q\x84``\x01Q[\x91P\x91P\x84`@\x01Q\x81a#\x12\x91\x90a,[V[\x82\x86`@\x01Qa#\"\x91\x90a/\xA6V[a#,\x91\x90a/\xBDV[\x92PPPa\x1A\nV[_\x81`@\x01Q\x15a$ W``\x82\x01Q\x82Q_\x91\x90a#\\\x90g\r\xE0\xB6\xB3\xA7d\0\0a/\xA6V[a#f\x91\x90a/\xBDV[\x90P_\x83`\x80\x01Q\x84` \x01Qg\r\xE0\xB6\xB3\xA7d\0\0a#\x86\x91\x90a/\xA6V[a#\x90\x91\x90a/\xBDV[\x90P_g\r\xE0\xB6\xB3\xA7d\0\0a#\xA6\x83\x85a/\xA6V[a#\xB0\x91\x90a/\xBDV[\x90P_g\r\xE0\xB6\xB3\xA7d\0\0a#\xC6\x84\x80a/\xA6V[a#\xD0\x91\x90a/\xBDV[g\r\xE0\xB6\xB3\xA7d\0\0a#\xE3\x86\x80a/\xA6V[a#\xED\x91\x90a/\xBDV[a#\xF7\x91\x90a,\x14V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0a$\x0C\x82\x84a/\xA6V[a$\x16\x91\x90a/\xBDV[\x96\x95PPPPPPV[` \x82\x01Q\x82Qa\x1A\n\x91\x90a/\xA6V[\x91\x90PV[_\x80[`\xFF\x81\x10\x15a%\xF2W_a$T\x84`@\x01Q\x85_\x01Qa& V[\x90P\x83` \x01Q\x81\x10\x15a%IW_a$t\x85`@\x01Q\x86_\x01Qa&\x9BV[\x82\x86` \x01Qa$\x84\x91\x90a,[V[a$\x96\x90g\r\xE0\xB6\xB3\xA7d\0\0a/\xA6V[a$\xA0\x91\x90a/\xBDV[\x90P\x80_\x03a%.W\x84` \x01Q\x82\x03a$\xBFWPPPP`@\x01Q\x90V[\x84` \x01Qa%\x14`@Q\x80`\xA0\x01`@R\x80\x88`@\x01Q`\x01a$\xE3\x91\x90a,\x14V[\x81R` \x01\x88_\x01Q\x81R` \x01\x88``\x01Q\x15\x15\x81R` \x01\x88`\x80\x01Q\x81R` \x01\x88`\xA0\x01Q\x81RPa#5V[\x11\x15a%*W`@\x85\x01Qa\x1DG\x90`\x01a,\x14V[P`\x01[\x80\x85`@\x01Qa%>\x91\x90a,\x14V[`@\x86\x01RPa%\xE9V[_a%[\x85`@\x01Q\x86_\x01Qa&\x9BV[` \x86\x01Qa%j\x90\x84a,[V[a%|\x90g\r\xE0\xB6\xB3\xA7d\0\0a/\xA6V[a%\x86\x91\x90a/\xBDV[\x90P\x80_\x03a%\xD2W\x84` \x01Q\x82\x14\x80a%\xBEWP\x84` \x01Qa%\xBC`\x01\x87`@\x01Qa%\xB5\x91\x90a,[V[\x87Qa& V[\x10[\x15a%\xCEWPPPP`@\x01Q\x90V[P`\x01[\x80\x85`@\x01Qa%\xE2\x91\x90a,[V[`@\x86\x01RP[P`\x01\x01a$9V[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x02`$\x82\x01Ra\x04/`\xF3\x1B`D\x82\x01R`d\x01a\x02\xF5V[_\x80g\r\xE0\xB6\xB3\xA7d\0\0a&5\x84\x86a/\xA6V[a&?\x91\x90a/\xBDV[\x90P_g\r\xE0\xB6\xB3\xA7d\0\0a&U\x85\x80a/\xA6V[a&_\x91\x90a/\xBDV[g\r\xE0\xB6\xB3\xA7d\0\0a&r\x87\x80a/\xA6V[a&|\x91\x90a/\xBDV[a&\x86\x91\x90a,\x14V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0a\x1D=\x82\x84a/\xA6V[_g\r\xE0\xB6\xB3\xA7d\0\0\x83\x81a&\xB1\x82\x80a/\xA6V[a&\xBB\x91\x90a/\xBDV[a&\xC5\x91\x90a/\xA6V[a&\xCF\x91\x90a/\xBDV[g\r\xE0\xB6\xB3\xA7d\0\0\x80a&\xE3\x85\x80a/\xA6V[a&\xED\x91\x90a/\xBDV[a&\xF8\x86`\x03a/\xA6V[a'\x02\x91\x90a/\xA6V[a'\x0C\x91\x90a/\xBDV[a'\x16\x91\x90a,\x14V[\x93\x92PPPV[`@Q\x80``\x01`@R\x80_\x81R` \x01_\x81R` \x01a'z`@\x80Qa\x01\0\x81\x01\x82R``\x80\x82R_` \x83\x01\x81\x90R\x92\x82\x01\x83\x90R\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x91\x90\x91R\x90V[\x90R\x90V[`@\x80Q`\xA0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91R`\x80\x81\x01a'z`@\x80Qa\x01\0\x81\x01\x82R``\x80\x82R_` \x83\x01\x81\x90R\x92\x82\x01\x83\x90R\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x91\x90\x91R\x90V[__\x83`\x1F\x84\x01\x12a'\xFCW__\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a(\x13W__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a(*W__\xFD[\x92P\x92\x90PV[____``\x85\x87\x03\x12\x15a(DW__\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a(hW__\xFD[a(t\x87\x82\x88\x01a'\xECV[\x95\x98\x94\x97P\x95PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a(\x94W__\xFD[PV[\x805a$1\x81a(\x80V[________`\xC0\x89\x8B\x03\x12\x15a(\xB9W__\xFD[\x885a(\xC4\x81a(\x80V[\x97P` \x89\x015a(\xD4\x81a(\x80V[\x96P`@\x89\x015a(\xE4\x81a(\x80V[\x95P``\x89\x015\x94P`\x80\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a)\x06W__\xFD[a)\x12\x8B\x82\x8C\x01a'\xECV[\x90\x95P\x93PP`\xA0\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a)1W__\xFD[a)=\x8B\x82\x8C\x01a'\xECV[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[_` \x82\x84\x03\x12\x15a)aW__\xFD[\x815a'\x16\x81a(\x80V[\x80\x15\x15\x81\x14a(\x94W__\xFD[\x805a$1\x81a)lV[__`@\x83\x85\x03\x12\x15a)\x95W__\xFD[\x825a)\xA0\x81a(\x80V[\x91P` \x83\x015a)\xB0\x81a)lV[\x80\x91PP\x92P\x92\x90PV[__`@\x83\x85\x03\x12\x15a)\xCCW__\xFD[\x825a)\xD7\x81a(\x80V[\x94` \x93\x90\x93\x015\x93PPPV[_____`\x80\x86\x88\x03\x12\x15a)\xF9W__\xFD[\x855a*\x04\x81a(\x80V[\x94P` \x86\x015\x93P`@\x86\x015\x92P``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*-W__\xFD[a*9\x88\x82\x89\x01a'\xECV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[_` \x82\x84\x03\x12\x15a*ZW__\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Qa\x01\0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a*\x99Wa*\x99a*aV[`@R\x90V[_\x82`\x1F\x83\x01\x12a*\xAEW__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*\xC8Wa*\xC8a*aV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a*\xF7Wa*\xF7a*aV[`@R\x81\x81R\x83\x82\x01` \x01\x85\x10\x15a+\x0EW__\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_` \x82\x84\x03\x12\x15a+:W__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+PW__\xFD[\x82\x01a\x01\0\x81\x85\x03\x12\x15a+bW__\xFD[a+ja*uV[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+\x80W__\xFD[a+\x8C\x86\x82\x85\x01a*\x9FV[\x82RPa+\x9B` \x83\x01a(\x97V[` \x82\x01Ra+\xAC`@\x83\x01a(\x97V[`@\x82\x01Ra+\xBD``\x83\x01a(\x97V[``\x82\x01R`\x80\x82\x81\x015\x90\x82\x01R`\xA0\x80\x83\x015\x90\x82\x01Ra+\xE2`\xC0\x83\x01a)yV[`\xC0\x82\x01Ra+\xF3`\xE0\x83\x01a)yV[`\xE0\x82\x01R\x94\x93PPPPV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x1A\nWa\x1A\na,\0V[`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x81R\x93\x85\x16` \x85\x01R\x91\x90\x93\x16`@\x83\x01R``\x82\x01\x92\x90\x92R\x90\x15\x15`\x80\x82\x01R`\xA0\x01\x90V[\x81\x81\x03\x81\x81\x11\x15a\x1A\nWa\x1A\na,\0V[_` \x82\x84\x03\x12\x15a,~W__\xFD[\x81Qa'\x16\x81a)lV[` \x80\x82R`/\x90\x82\x01R\x7FOnly owner or liquidator can cal`@\x82\x01Rn6\x10:44\xB9\x903:\xB71\xBA4\xB7\xB7`\x89\x1B``\x82\x01R`\x80\x01\x90V[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[\x81\x81\x03_\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a-\x06Wa-\x06a,\0V[P\x92\x91PPV[` \x80\x82R`\x0C\x90\x82\x01Rk\x15S\x90UU\x12\x13\xD4\x92V\x91Q`\xA2\x1B`@\x82\x01R``\x01\x90V[_` \x82\x84\x03\x12\x15a-CW__\xFD[\x81Qa'\x16\x81a(\x80V[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_\x82Qa\x01\0` \x84\x01Ra-\x99a\x01 \x84\x01\x82a-NV[\x90P`\x01\x80`\xA0\x1B\x03` \x85\x01Q\x16`@\x84\x01R`\x01\x80`\xA0\x1B\x03`@\x85\x01Q\x16``\x84\x01R``\x84\x01Qa-\xD9`\x80\x85\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\x80\x84\x01Q`\xA0\x84\x01R`\xA0\x84\x01Q`\xC0\x84\x01R`\xC0\x84\x01Qa.\x01`\xE0\x85\x01\x82\x15\x15\x90RV[P`\xE0\x84\x01Q\x80\x15\x15a\x01\0\x85\x01RP\x93\x92PPPV[\x84\x81R\x83` \x82\x01R`\x01\x80`\xA0\x1B\x03\x83\x16`@\x82\x01R`\x80``\x82\x01R_a$\x16`\x80\x83\x01\x84a-NV[_`\x01`\xFF\x1B\x82\x01a.XWa.Xa,\0V[P_\x03\x90V[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x1A\nWa\x1A\na,\0V[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x1A\nWa\x1A\na,\0V[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R\x85\x15\x15` \x83\x01R`@\x82\x01\x85\x90R\x83\x16``\x82\x01R`\xA0`\x80\x82\x01\x81\x90R_\x90a.\xD6\x90\x83\x01\x84a-NV[\x97\x96PPPPPPPV[__`@\x83\x85\x03\x12\x15a.\xF2W__\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R\x84\x16` \x82\x01R`@\x81\x01\x83\x90R`\x80``\x82\x01\x81\x90R_\x90a$\x16\x90\x83\x01\x84a-NV[_______`\xE0\x88\x8A\x03\x12\x15a/KW__\xFD[\x87Q` \x89\x01Q`@\x8A\x01Q``\x8B\x01Q`\x80\x8C\x01Q\x93\x9AP\x91\x98P\x96P\x94Pa/t\x81a)lV[`\xA0\x89\x01Q\x90\x93Pa/\x85\x81a(\x80V[`\xC0\x89\x01Q\x90\x92Pa/\x96\x81a(\x80V[\x80\x91PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x1A\nWa\x1A\na,\0V[_\x82a/\xD7WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V\xFE\xA2dipfsX\"\x12 \x90\xDF%?\xAA\xD0{\xE9\x8F\x88f \x8C\xEE\xE6\xC0\xAC\xB3\x9BJ\xF3\xBD\x9DY\x94\x9E\x01\xBEx\x8APUdsolcC\0\x08\x1C\x003";
    /// The deployed bytecode of the contract.
    pub static LIQUIDATOR_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Liquidator<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Liquidator<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Liquidator<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Liquidator<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Liquidator<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Liquidator)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Liquidator<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    LIQUIDATOR_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                LIQUIDATOR_ABI.clone(),
                LIQUIDATOR_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `FLASH_MINTER` (0xea939fa6) function
        pub fn flash_minter(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([234, 147, 159, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `USDXL` (0x70c26a5e) function
        pub fn usdxl(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([112, 194, 106, 94], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approvePool` (0x424c265b) function
        pub fn approve_pool(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 76, 38, 91], token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeOperation` (0x0db7b028) function
        pub fn execute_operation(
            &self,
            amount: ::ethers::core::types::U256,
            fee: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([13, 183, 176, 40], (amount, fee, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hook` (0x9a7bff79) function
        pub fn hook(
            &self,
            sender: ::ethers::core::types::Address,
            amount_0_out: ::ethers::core::types::U256,
            amount_1_out: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [154, 123, 255, 121],
                    (sender, amount_0_out, amount_1_out, data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hyperswapV3Factory` (0x3e0d955a) function
        pub fn hyperswap_v3_factory(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([62, 13, 149, 90], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isLiquidator` (0x529a356f) function
        pub fn is_liquidator(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([82, 154, 53, 111], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `kittenPairFactory` (0x08bea127) function
        pub fn kitten_pair_factory(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([8, 190, 161, 39], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `liquidate` (0x2e43c961) function
        pub fn liquidate(
            &self,
            collateral_asset: ::ethers::core::types::Address,
            debt_asset: ::ethers::core::types::Address,
            user: ::ethers::core::types::Address,
            debt_to_cover: ::ethers::core::types::U256,
            swap_path: ::ethers::core::types::Bytes,
            liq_path: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::Address, ::ethers::core::types::I256),
        > {
            self.0
                .method_hash(
                    [46, 67, 201, 97],
                    (
                        collateral_asset,
                        debt_asset,
                        user,
                        debt_to_cover,
                        swap_path,
                        liq_path,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pool` (0x16f0115b) function
        pub fn pool(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([22, 240, 17, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `recover` (0x5705ae43) function
        pub fn recover(
            &self,
            token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([87, 5, 174, 67], (token, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setLiquidator` (0x4453a374) function
        pub fn set_liquidator(
            &self,
            liquidator: ::ethers::core::types::Address,
            enabled: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([68, 83, 163, 116], (liquidator, enabled))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `uniswapV3SwapCallback` (0xfa461e33) function
        pub fn uniswap_v3_swap_callback(
            &self,
            amount_0_delta: ::ethers::core::types::I256,
            amount_1_delta: ::ethers::core::types::I256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 70, 30, 51], (amount_0_delta, amount_1_delta, data))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `LiquidatorSet` event
        pub fn liquidator_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LiquidatorSetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LiquidatorEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Liquidator<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "LiquidatorSet", abi = "LiquidatorSet(address,bool)")]
    pub struct LiquidatorSetFilter {
        #[ethevent(indexed)]
        pub liquidator: ::ethers::core::types::Address,
        pub enabled: bool,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub user: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum LiquidatorEvents {
        LiquidatorSetFilter(LiquidatorSetFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ::ethers::contract::EthLogDecode for LiquidatorEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = LiquidatorSetFilter::decode_log(log) {
                return Ok(LiquidatorEvents::LiquidatorSetFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(LiquidatorEvents::OwnershipTransferredFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for LiquidatorEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::LiquidatorSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<LiquidatorSetFilter> for LiquidatorEvents {
        fn from(value: LiquidatorSetFilter) -> Self {
            Self::LiquidatorSetFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for LiquidatorEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    ///Container type for all input parameters for the `FLASH_MINTER` function with signature `FLASH_MINTER()` and selector `0xea939fa6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "FLASH_MINTER", abi = "FLASH_MINTER()")]
    pub struct FlashMinterCall;
    ///Container type for all input parameters for the `USDXL` function with signature `USDXL()` and selector `0x70c26a5e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "USDXL", abi = "USDXL()")]
    pub struct UsdxlCall;
    ///Container type for all input parameters for the `approvePool` function with signature `approvePool(address)` and selector `0x424c265b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "approvePool", abi = "approvePool(address)")]
    pub struct ApprovePoolCall {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `executeOperation` function with signature `executeOperation(uint256,uint256,bytes)` and selector `0x0db7b028`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "executeOperation",
        abi = "executeOperation(uint256,uint256,bytes)"
    )]
    pub struct ExecuteOperationCall {
        pub amount: ::ethers::core::types::U256,
        pub fee: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `hook` function with signature `hook(address,uint256,uint256,bytes)` and selector `0x9a7bff79`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "hook", abi = "hook(address,uint256,uint256,bytes)")]
    pub struct HookCall {
        pub sender: ::ethers::core::types::Address,
        pub amount_0_out: ::ethers::core::types::U256,
        pub amount_1_out: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `hyperswapV3Factory` function with signature `hyperswapV3Factory()` and selector `0x3e0d955a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "hyperswapV3Factory", abi = "hyperswapV3Factory()")]
    pub struct HyperswapV3FactoryCall;
    ///Container type for all input parameters for the `isLiquidator` function with signature `isLiquidator(address)` and selector `0x529a356f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "isLiquidator", abi = "isLiquidator(address)")]
    pub struct IsLiquidatorCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `kittenPairFactory` function with signature `kittenPairFactory()` and selector `0x08bea127`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "kittenPairFactory", abi = "kittenPairFactory()")]
    pub struct KittenPairFactoryCall;
    ///Container type for all input parameters for the `liquidate` function with signature `liquidate(address,address,address,uint256,bytes,string)` and selector `0x2e43c961`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "liquidate",
        abi = "liquidate(address,address,address,uint256,bytes,string)"
    )]
    pub struct LiquidateCall {
        pub collateral_asset: ::ethers::core::types::Address,
        pub debt_asset: ::ethers::core::types::Address,
        pub user: ::ethers::core::types::Address,
        pub debt_to_cover: ::ethers::core::types::U256,
        pub swap_path: ::ethers::core::types::Bytes,
        pub liq_path: ::std::string::String,
    }
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `pool` function with signature `pool()` and selector `0x16f0115b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "pool", abi = "pool()")]
    pub struct PoolCall;
    ///Container type for all input parameters for the `recover` function with signature `recover(address,uint256)` and selector `0x5705ae43`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "recover", abi = "recover(address,uint256)")]
    pub struct RecoverCall {
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setLiquidator` function with signature `setLiquidator(address,bool)` and selector `0x4453a374`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setLiquidator", abi = "setLiquidator(address,bool)")]
    pub struct SetLiquidatorCall {
        pub liquidator: ::ethers::core::types::Address,
        pub enabled: bool,
    }
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `uniswapV3SwapCallback` function with signature `uniswapV3SwapCallback(int256,int256,bytes)` and selector `0xfa461e33`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "uniswapV3SwapCallback",
        abi = "uniswapV3SwapCallback(int256,int256,bytes)"
    )]
    pub struct UniswapV3SwapCallbackCall {
        pub amount_0_delta: ::ethers::core::types::I256,
        pub amount_1_delta: ::ethers::core::types::I256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum LiquidatorCalls {
        FlashMinter(FlashMinterCall),
        Usdxl(UsdxlCall),
        ApprovePool(ApprovePoolCall),
        ExecuteOperation(ExecuteOperationCall),
        Hook(HookCall),
        HyperswapV3Factory(HyperswapV3FactoryCall),
        IsLiquidator(IsLiquidatorCall),
        KittenPairFactory(KittenPairFactoryCall),
        Liquidate(LiquidateCall),
        Owner(OwnerCall),
        Pool(PoolCall),
        Recover(RecoverCall),
        SetLiquidator(SetLiquidatorCall),
        TransferOwnership(TransferOwnershipCall),
        UniswapV3SwapCallback(UniswapV3SwapCallbackCall),
    }
    impl ::ethers::core::abi::AbiDecode for LiquidatorCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <FlashMinterCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FlashMinter(decoded));
            }
            if let Ok(decoded) = <UsdxlCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Usdxl(decoded));
            }
            if let Ok(decoded) = <ApprovePoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ApprovePool(decoded));
            }
            if let Ok(decoded) = <ExecuteOperationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteOperation(decoded));
            }
            if let Ok(decoded) = <HookCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Hook(decoded));
            }
            if let Ok(decoded) = <HyperswapV3FactoryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HyperswapV3Factory(decoded));
            }
            if let Ok(decoded) = <IsLiquidatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsLiquidator(decoded));
            }
            if let Ok(decoded) = <KittenPairFactoryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::KittenPairFactory(decoded));
            }
            if let Ok(decoded) = <LiquidateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Liquidate(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <PoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Pool(decoded));
            }
            if let Ok(decoded) = <RecoverCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Recover(decoded));
            }
            if let Ok(decoded) = <SetLiquidatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetLiquidator(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <UniswapV3SwapCallbackCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UniswapV3SwapCallback(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LiquidatorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::FlashMinter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Usdxl(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ApprovePool(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteOperation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Hook(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::HyperswapV3Factory(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsLiquidator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::KittenPairFactory(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Liquidate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Recover(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetLiquidator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UniswapV3SwapCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for LiquidatorCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::FlashMinter(element) => ::core::fmt::Display::fmt(element, f),
                Self::Usdxl(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApprovePool(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteOperation(element) => ::core::fmt::Display::fmt(element, f),
                Self::Hook(element) => ::core::fmt::Display::fmt(element, f),
                Self::HyperswapV3Factory(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsLiquidator(element) => ::core::fmt::Display::fmt(element, f),
                Self::KittenPairFactory(element) => ::core::fmt::Display::fmt(element, f),
                Self::Liquidate(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pool(element) => ::core::fmt::Display::fmt(element, f),
                Self::Recover(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetLiquidator(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UniswapV3SwapCallback(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<FlashMinterCall> for LiquidatorCalls {
        fn from(value: FlashMinterCall) -> Self {
            Self::FlashMinter(value)
        }
    }
    impl ::core::convert::From<UsdxlCall> for LiquidatorCalls {
        fn from(value: UsdxlCall) -> Self {
            Self::Usdxl(value)
        }
    }
    impl ::core::convert::From<ApprovePoolCall> for LiquidatorCalls {
        fn from(value: ApprovePoolCall) -> Self {
            Self::ApprovePool(value)
        }
    }
    impl ::core::convert::From<ExecuteOperationCall> for LiquidatorCalls {
        fn from(value: ExecuteOperationCall) -> Self {
            Self::ExecuteOperation(value)
        }
    }
    impl ::core::convert::From<HookCall> for LiquidatorCalls {
        fn from(value: HookCall) -> Self {
            Self::Hook(value)
        }
    }
    impl ::core::convert::From<HyperswapV3FactoryCall> for LiquidatorCalls {
        fn from(value: HyperswapV3FactoryCall) -> Self {
            Self::HyperswapV3Factory(value)
        }
    }
    impl ::core::convert::From<IsLiquidatorCall> for LiquidatorCalls {
        fn from(value: IsLiquidatorCall) -> Self {
            Self::IsLiquidator(value)
        }
    }
    impl ::core::convert::From<KittenPairFactoryCall> for LiquidatorCalls {
        fn from(value: KittenPairFactoryCall) -> Self {
            Self::KittenPairFactory(value)
        }
    }
    impl ::core::convert::From<LiquidateCall> for LiquidatorCalls {
        fn from(value: LiquidateCall) -> Self {
            Self::Liquidate(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for LiquidatorCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PoolCall> for LiquidatorCalls {
        fn from(value: PoolCall) -> Self {
            Self::Pool(value)
        }
    }
    impl ::core::convert::From<RecoverCall> for LiquidatorCalls {
        fn from(value: RecoverCall) -> Self {
            Self::Recover(value)
        }
    }
    impl ::core::convert::From<SetLiquidatorCall> for LiquidatorCalls {
        fn from(value: SetLiquidatorCall) -> Self {
            Self::SetLiquidator(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for LiquidatorCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UniswapV3SwapCallbackCall> for LiquidatorCalls {
        fn from(value: UniswapV3SwapCallbackCall) -> Self {
            Self::UniswapV3SwapCallback(value)
        }
    }
    ///Container type for all return fields from the `FLASH_MINTER` function with signature `FLASH_MINTER()` and selector `0xea939fa6`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct FlashMinterReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `USDXL` function with signature `USDXL()` and selector `0x70c26a5e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct UsdxlReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `executeOperation` function with signature `executeOperation(uint256,uint256,bytes)` and selector `0x0db7b028`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ExecuteOperationReturn {
        pub success: bool,
    }
    ///Container type for all return fields from the `hyperswapV3Factory` function with signature `hyperswapV3Factory()` and selector `0x3e0d955a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct HyperswapV3FactoryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `isLiquidator` function with signature `isLiquidator(address)` and selector `0x529a356f`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IsLiquidatorReturn(pub bool);
    ///Container type for all return fields from the `kittenPairFactory` function with signature `kittenPairFactory()` and selector `0x08bea127`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct KittenPairFactoryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `liquidate` function with signature `liquidate(address,address,address,uint256,bytes,string)` and selector `0x2e43c961`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct LiquidateReturn {
        pub final_token: ::ethers::core::types::Address,
        pub final_gain: ::ethers::core::types::I256,
    }
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `pool` function with signature `pool()` and selector `0x16f0115b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct PoolReturn(pub ::ethers::core::types::Address);
}
