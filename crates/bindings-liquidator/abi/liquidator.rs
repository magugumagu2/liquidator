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
                    ::std::borrow::ToOwned::to_owned("liquidate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("liquidate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("collateral"),
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
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "liquidatedCollateralAmount",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("receiveAToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swapPath"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("collateralGain"),
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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15`\x0EW__\xFD[P_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x90\x81\x17\x82U`@Q\x90\x91\x82\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90\x82\x90\xA3Pa\x12\x0B\x80a\0\\_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0zW_5`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0XW\x80c\x8D\xA5\xCB[\x14a\0\xDEW\x80c\xE8\xAE\xD5'\x14a\0\xF0W\x80c\xF2\xFD\xE3\x8B\x14a\x01\x11W\x80c\xFAF\x1E3\x14a\x01$W__\xFD[\x80c\x16\xF0\x11[\x14a\0~W\x80cBL&[\x14a\0\xB6W\x80cW\x05\xAEC\x14a\0\xCBW[__\xFD[a\0\x99s2F{C\xBF\xA6rs\xFC}\xDD\xA0\x99\x9E\xE9\xA1/*\xAA\x08\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xC9a\0\xC46`\x04a\x0COV[a\x017V[\0[a\0\xC9a\0\xD96`\x04a\x0CoV[a\x01\xEFV[_Ta\0\x99\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\x03a\0\xFE6`\x04a\x0C\xF7V[a\x02\xC3V[`@Q\x90\x81R` \x01a\0\xADV[a\0\xC9a\x01\x1F6`\x04a\x0COV[a\x04\x84V[a\0\xC9a\x0126`\x04a\r\x9FV[a\x04\xF7V[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01iW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01`\x90a\r\xEEV[`@Q\x80\x91\x03\x90\xFD[`@Qc\t^\xA7\xB3`\xE0\x1B\x81Rs2F{C\xBF\xA6rs\xFC}\xDD\xA0\x99\x9E\xE9\xA1/*\xAA\x08`\x04\x82\x01R_\x19`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x01\xC7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xEB\x91\x90a\x0E\x14V[PPV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01`\x90a\r\xEEV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x02UW`@Q3\x90\x82\x15a\x08\xFC\x02\x90\x83\x90_\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x02PW=__>=_\xFD[PPPV[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x02\x9FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02P\x91\x90a\x0E\x14V[_\x80T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01`\x90a\r\xEEV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R_\x90`\x01`\x01`\xA0\x1B\x03\x8C\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x031W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03U\x91\x90a\x0E/V[\x90Pa\x04\x03\x88`@Q\x80a\x01\0\x01`@R\x80\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x8E`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8D`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8C`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8B\x81R` \x01\x8A\x81R` \x01\x89`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x88\x15\x15\x81RPa\x06\x9AV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x81\x90`\x01`\x01`\xA0\x1B\x03\x8D\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04GW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04k\x91\x90a\x0E/V[a\x04u\x91\x90a\x0EZV[\x9B\x9APPPPPPPPPPPV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01`\x90a\r\xEEV[_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x82U`@Q\x90\x913\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PV[_a\x05\x04\x82\x84\x01\x84a\x0FIV[\x90P___a\x05\x15\x84_\x01Qa\x07\xC7V[\x92P\x92P\x92Pa\x05Cs\xCC\xF1v\x9D\x87\x13\t\x91rd.\xB5]\xDF\xFC\x0CZDO\xE9a\x05>\x85\x85\x85a\x08\x06V[a\x08pV[` \x84\x01Q`@\x80\x86\x01Q``\x87\x01Q`\x80\x88\x01Q`\xE0\x89\x01Q\x93Qb\xA7\x18\xA9`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x95\x86\x16`\x04\x82\x01R\x92\x85\x16`$\x84\x01R\x93\x16`D\x82\x01R`d\x81\x01\x92\x90\x92R\x15\x15`\x84\x82\x01Rs2F{C\xBF\xA6rs\xFC}\xDD\xA0\x99\x9E\xE9\xA1/*\xAA\x08\x90b\xA7\x18\xA9\x90`\xA4\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x05\xCBW__\xFD[PZ\xF1\x15\x80\x15a\x05\xDDW=__>=_\xFD[PPPP__\x89\x13a\x05\xEFW\x87a\x05\xF1V[\x88[\x90Pa\x05\xFF\x85_\x01Qa\x08\xC4V[\x15a\x06\x1BW\x84Qa\x06\x0F\x90a\x08\xFDV[\x85Ra\x06\x1B\x81\x86a\x06\x9AV[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R\x92\x93P\x83\x92`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x06jW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x8E\x91\x90a\x0E\x14V[PPPPPPPPPPV[___a\x06\xA9\x84_\x01Qa\x07\xC7V[\x91\x94P\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x90\x83\x16\x10_a\x06\xE8s\xCC\xF1v\x9D\x87\x13\t\x91rd.\xB5]\xDF\xFC\x0CZDO\xE9a\x06\xE3\x86\x88\x87a\x08\x06V[a\t4V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16c\x12\x8A\xCB\x080\x84a\x07\x04\x8Ba\x10\x1FV[\x86a\x07-Wa\x07(`\x01s\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D&a\x109V[a\x07=V[a\x07=d\x01\0\x02v\xA3`\x01a\x10XV[\x8B`@Q` \x01a\x07N\x91\x90a\x10\xA5V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07}\x95\x94\x93\x92\x91\x90a\x11HV[`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x07\x98W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xBC\x91\x90a\x11\x8DV[PPPPPPPPPV[_\x80\x80a\x07\xD4\x84\x82a\n\x1AV[\x92Pa\x07\xE1\x84`\x14a\n~V[a\xFF\xFF\x16\x90Pa\x07\xFDa\x07\xF6`\x03`\x14a\x11\xAFV[\x85\x90a\n\x1AV[\x91P\x91\x93\x90\x92PV[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x11\x15a\x08@W\x91\x92\x91[P`@\x80Q``\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R\x92\x90\x93\x16` \x83\x01Rb\xFF\xFF\xFF\x16\x91\x81\x01\x91\x90\x91R\x90V[_a\x08{\x83\x83a\t4V[\x90P3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14a\x02PW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x1A[\x9D\x98[\x1AY\x08\x1C\x1B\xDB\xDB`\xA2\x1B`D\x82\x01R`d\x01a\x01`V[_a\x08\xD1`\x03`\x14a\x11\xAFV[`\x14a\x08\xDE`\x03\x82a\x11\xAFV[a\x08\xE8\x91\x90a\x11\xAFV[a\x08\xF2\x91\x90a\x11\xAFV[\x82Q\x10\x15\x90P\x91\x90PV[``a\t.a\t\x0E`\x03`\x14a\x11\xAFV[a\t\x1A`\x03`\x14a\x11\xAFV[\x84Qa\t&\x91\x90a\x11\xC2V[\x84\x91\x90a\x0B(V[\x92\x91PPV[_\x81` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82_\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x10a\tYW__\xFD[\x81Q` \x80\x84\x01Q`@\x80\x86\x01Q\x81Q`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x81\x86\x01R\x94\x90\x92\x16\x84\x82\x01Rb\xFF\xFF\xFF\x90\x91\x16``\x80\x85\x01\x91\x90\x91R\x81Q\x80\x85\x03\x82\x01\x81R`\x80\x85\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\x01`\x01`\xF8\x1B\x03\x19`\xA0\x84\x01R\x90\x85\x90\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\xA1\x83\x01R`\xB5\x82\x01R\x7F\xE3O\x19\x9B\x19\xB2\xB4\xF4\x7FhD&\x19\xD5UR}$Ox\xA3)~\xA8\x93%\xF8C\xF8{\x8BT`\xD5\x82\x01R`\xF5\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x93\x92PPPV[_a\n&\x82`\x14a\x11\xAFV[\x83Q\x10\x15a\nnW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RttoAddress_outOfBounds`X\x1B`D\x82\x01R`d\x01a\x01`V[P\x01` \x01Q`\x01``\x1B\x90\x04\x90V[_\x81a\n\x8B\x81`\x03a\x11\xAFV[\x10\x15a\n\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01RptoUint24_overflow`x\x1B`D\x82\x01R`d\x01a\x01`V[a\n\xD8\x82`\x03a\x11\xAFV[\x83Q\x10\x15a\x0B\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01RstoUint24_outOfBounds``\x1B`D\x82\x01R`d\x01a\x01`V[P\x01`\x03\x01Q\x90V[``\x81a\x0B6\x81`\x1Fa\x11\xAFV[\x10\x15a\x0BuW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rmslice_overflow`\x90\x1B`D\x82\x01R`d\x01a\x01`V[a\x0B\x7F\x82\x84a\x11\xAFV[\x84Q\x10\x15a\x0B\xC3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rpslice_outOfBounds`x\x1B`D\x82\x01R`d\x01a\x01`V[``\x82\x15\x80\x15a\x0B\xE1W`@Q\x91P_\x82R` \x82\x01`@Ra\x0C+V[`@Q\x91P`\x1F\x84\x16\x80\x15` \x02\x81\x84\x01\x01\x85\x81\x01\x87\x83\x15` \x02\x84\x8B\x01\x01\x01[\x81\x83\x10\x15a\x0C\x1AW\x80Q\x83R` \x92\x83\x01\x92\x01a\x0C\x02V[PP\x85\x84R`\x1F\x01`\x1F\x19\x16`@RP[P\x94\x93PPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0CJW__\xFD[\x91\x90PV[_` \x82\x84\x03\x12\x15a\x0C_W__\xFD[a\x0Ch\x82a\x0C4V[\x93\x92PPPV[__`@\x83\x85\x03\x12\x15a\x0C\x80W__\xFD[a\x0C\x89\x83a\x0C4V[\x94` \x93\x90\x93\x015\x93PPPV[\x80\x15\x15\x81\x14a\x0C\xA4W__\xFD[PV[\x805a\x0CJ\x81a\x0C\x97V[__\x83`\x1F\x84\x01\x12a\x0C\xC2W__\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\xD9W__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x0C\xF0W__\xFD[\x92P\x92\x90PV[_________a\x01\0\x8A\x8C\x03\x12\x15a\r\x10W__\xFD[a\r\x19\x8Aa\x0C4V[\x98Pa\r'` \x8B\x01a\x0C4V[\x97Pa\r5`@\x8B\x01a\x0C4V[\x96P``\x8A\x015\x95P`\x80\x8A\x015\x94Pa\rQ`\xA0\x8B\x01a\x0C4V[\x93P`\xC0\x8A\x015a\ra\x81a\x0C\x97V[\x92P`\xE0\x8A\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r|W__\xFD[a\r\x88\x8C\x82\x8D\x01a\x0C\xB2V[\x91P\x80\x93PP\x80\x91PP\x92\x95\x98P\x92\x95\x98P\x92\x95\x98V[____``\x85\x87\x03\x12\x15a\r\xB2W__\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\xD6W__\xFD[a\r\xE2\x87\x82\x88\x01a\x0C\xB2V[\x95\x98\x94\x97P\x95PPPPV[` \x80\x82R`\x0C\x90\x82\x01Rk\x15S\x90UU\x12\x13\xD4\x92V\x91Q`\xA2\x1B`@\x82\x01R``\x01\x90V[_` \x82\x84\x03\x12\x15a\x0E$W__\xFD[\x81Qa\x0Ch\x81a\x0C\x97V[_` \x82\x84\x03\x12\x15a\x0E?W__\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03_\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x0EyWa\x0Eya\x0EFV[P\x92\x91PPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Qa\x01\0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0E\xB8Wa\x0E\xB8a\x0E\x80V[`@R\x90V[_\x82`\x1F\x83\x01\x12a\x0E\xCDW__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\xE7Wa\x0E\xE7a\x0E\x80V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0F\x16Wa\x0F\x16a\x0E\x80V[`@R\x81\x81R\x83\x82\x01` \x01\x85\x10\x15a\x0F-W__\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_` \x82\x84\x03\x12\x15a\x0FYW__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0FoW__\xFD[\x82\x01a\x01\0\x81\x85\x03\x12\x15a\x0F\x81W__\xFD[a\x0F\x89a\x0E\x94V[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F\x9FW__\xFD[a\x0F\xAB\x86\x82\x85\x01a\x0E\xBEV[\x82RPa\x0F\xBA` \x83\x01a\x0C4V[` \x82\x01Ra\x0F\xCB`@\x83\x01a\x0C4V[`@\x82\x01Ra\x0F\xDC``\x83\x01a\x0C4V[``\x82\x01R`\x80\x82\x81\x015\x90\x82\x01R`\xA0\x80\x83\x015\x90\x82\x01Ra\x10\x01`\xC0\x83\x01a\x0C4V[`\xC0\x82\x01Ra\x10\x12`\xE0\x83\x01a\x0C\xA7V[`\xE0\x82\x01R\x94\x93PPPPV[_`\x01`\xFF\x1B\x82\x01a\x103Wa\x103a\x0EFV[P_\x03\x90V[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\t.Wa\t.a\x0EFV[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\t.Wa\t.a\x0EFV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_\x82Qa\x01\0` \x84\x01Ra\x10\xC2a\x01 \x84\x01\x82a\x10wV[\x90P`\x01\x80`\xA0\x1B\x03` \x85\x01Q\x16`@\x84\x01R`\x01\x80`\xA0\x1B\x03`@\x85\x01Q\x16``\x84\x01R``\x84\x01Qa\x11\x02`\x80\x85\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\x80\x84\x01Q`\xA0\x84\x01R`\xA0\x84\x01Q`\xC0\x84\x01R`\xC0\x84\x01Qa\x111`\xE0\x85\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\xE0\x84\x01Q\x80\x15\x15a\x01\0\x85\x01RP\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R\x85\x15\x15` \x83\x01R`@\x82\x01\x85\x90R\x83\x16``\x82\x01R`\xA0`\x80\x82\x01\x81\x90R_\x90a\x11\x82\x90\x83\x01\x84a\x10wV[\x97\x96PPPPPPPV[__`@\x83\x85\x03\x12\x15a\x11\x9EW__\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[\x80\x82\x01\x80\x82\x11\x15a\t.Wa\t.a\x0EFV[\x81\x81\x03\x81\x81\x11\x15a\t.Wa\t.a\x0EFV\xFE\xA2dipfsX\"\x12 ;\x1C\xF2\x8F\xBEx \x1D/\xEA\xD8\rDc(\xFEj\x0E2*\xF7\xEDn{=i\x98Y\x82R\x86\xFDdsolcC\0\x08\x1C\x003";
    /// The bytecode of the contract.
    pub static LIQUIDATOR_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0zW_5`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0XW\x80c\x8D\xA5\xCB[\x14a\0\xDEW\x80c\xE8\xAE\xD5'\x14a\0\xF0W\x80c\xF2\xFD\xE3\x8B\x14a\x01\x11W\x80c\xFAF\x1E3\x14a\x01$W__\xFD[\x80c\x16\xF0\x11[\x14a\0~W\x80cBL&[\x14a\0\xB6W\x80cW\x05\xAEC\x14a\0\xCBW[__\xFD[a\0\x99s2F{C\xBF\xA6rs\xFC}\xDD\xA0\x99\x9E\xE9\xA1/*\xAA\x08\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xC9a\0\xC46`\x04a\x0COV[a\x017V[\0[a\0\xC9a\0\xD96`\x04a\x0CoV[a\x01\xEFV[_Ta\0\x99\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\x03a\0\xFE6`\x04a\x0C\xF7V[a\x02\xC3V[`@Q\x90\x81R` \x01a\0\xADV[a\0\xC9a\x01\x1F6`\x04a\x0COV[a\x04\x84V[a\0\xC9a\x0126`\x04a\r\x9FV[a\x04\xF7V[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01iW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01`\x90a\r\xEEV[`@Q\x80\x91\x03\x90\xFD[`@Qc\t^\xA7\xB3`\xE0\x1B\x81Rs2F{C\xBF\xA6rs\xFC}\xDD\xA0\x99\x9E\xE9\xA1/*\xAA\x08`\x04\x82\x01R_\x19`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x01\xC7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xEB\x91\x90a\x0E\x14V[PPV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01`\x90a\r\xEEV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x02UW`@Q3\x90\x82\x15a\x08\xFC\x02\x90\x83\x90_\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x02PW=__>=_\xFD[PPPV[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x02\x9FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02P\x91\x90a\x0E\x14V[_\x80T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01`\x90a\r\xEEV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R_\x90`\x01`\x01`\xA0\x1B\x03\x8C\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x031W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03U\x91\x90a\x0E/V[\x90Pa\x04\x03\x88`@Q\x80a\x01\0\x01`@R\x80\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x8E`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8D`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8C`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8B\x81R` \x01\x8A\x81R` \x01\x89`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x88\x15\x15\x81RPa\x06\x9AV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x81\x90`\x01`\x01`\xA0\x1B\x03\x8D\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04GW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04k\x91\x90a\x0E/V[a\x04u\x91\x90a\x0EZV[\x9B\x9APPPPPPPPPPPV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01`\x90a\r\xEEV[_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x82U`@Q\x90\x913\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PV[_a\x05\x04\x82\x84\x01\x84a\x0FIV[\x90P___a\x05\x15\x84_\x01Qa\x07\xC7V[\x92P\x92P\x92Pa\x05Cs\xCC\xF1v\x9D\x87\x13\t\x91rd.\xB5]\xDF\xFC\x0CZDO\xE9a\x05>\x85\x85\x85a\x08\x06V[a\x08pV[` \x84\x01Q`@\x80\x86\x01Q``\x87\x01Q`\x80\x88\x01Q`\xE0\x89\x01Q\x93Qb\xA7\x18\xA9`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x95\x86\x16`\x04\x82\x01R\x92\x85\x16`$\x84\x01R\x93\x16`D\x82\x01R`d\x81\x01\x92\x90\x92R\x15\x15`\x84\x82\x01Rs2F{C\xBF\xA6rs\xFC}\xDD\xA0\x99\x9E\xE9\xA1/*\xAA\x08\x90b\xA7\x18\xA9\x90`\xA4\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x05\xCBW__\xFD[PZ\xF1\x15\x80\x15a\x05\xDDW=__>=_\xFD[PPPP__\x89\x13a\x05\xEFW\x87a\x05\xF1V[\x88[\x90Pa\x05\xFF\x85_\x01Qa\x08\xC4V[\x15a\x06\x1BW\x84Qa\x06\x0F\x90a\x08\xFDV[\x85Ra\x06\x1B\x81\x86a\x06\x9AV[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R\x92\x93P\x83\x92`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x06jW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x8E\x91\x90a\x0E\x14V[PPPPPPPPPPV[___a\x06\xA9\x84_\x01Qa\x07\xC7V[\x91\x94P\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x90\x83\x16\x10_a\x06\xE8s\xCC\xF1v\x9D\x87\x13\t\x91rd.\xB5]\xDF\xFC\x0CZDO\xE9a\x06\xE3\x86\x88\x87a\x08\x06V[a\t4V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16c\x12\x8A\xCB\x080\x84a\x07\x04\x8Ba\x10\x1FV[\x86a\x07-Wa\x07(`\x01s\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D&a\x109V[a\x07=V[a\x07=d\x01\0\x02v\xA3`\x01a\x10XV[\x8B`@Q` \x01a\x07N\x91\x90a\x10\xA5V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07}\x95\x94\x93\x92\x91\x90a\x11HV[`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x07\x98W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xBC\x91\x90a\x11\x8DV[PPPPPPPPPV[_\x80\x80a\x07\xD4\x84\x82a\n\x1AV[\x92Pa\x07\xE1\x84`\x14a\n~V[a\xFF\xFF\x16\x90Pa\x07\xFDa\x07\xF6`\x03`\x14a\x11\xAFV[\x85\x90a\n\x1AV[\x91P\x91\x93\x90\x92PV[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x11\x15a\x08@W\x91\x92\x91[P`@\x80Q``\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R\x92\x90\x93\x16` \x83\x01Rb\xFF\xFF\xFF\x16\x91\x81\x01\x91\x90\x91R\x90V[_a\x08{\x83\x83a\t4V[\x90P3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14a\x02PW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x1A[\x9D\x98[\x1AY\x08\x1C\x1B\xDB\xDB`\xA2\x1B`D\x82\x01R`d\x01a\x01`V[_a\x08\xD1`\x03`\x14a\x11\xAFV[`\x14a\x08\xDE`\x03\x82a\x11\xAFV[a\x08\xE8\x91\x90a\x11\xAFV[a\x08\xF2\x91\x90a\x11\xAFV[\x82Q\x10\x15\x90P\x91\x90PV[``a\t.a\t\x0E`\x03`\x14a\x11\xAFV[a\t\x1A`\x03`\x14a\x11\xAFV[\x84Qa\t&\x91\x90a\x11\xC2V[\x84\x91\x90a\x0B(V[\x92\x91PPV[_\x81` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82_\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x10a\tYW__\xFD[\x81Q` \x80\x84\x01Q`@\x80\x86\x01Q\x81Q`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x81\x86\x01R\x94\x90\x92\x16\x84\x82\x01Rb\xFF\xFF\xFF\x90\x91\x16``\x80\x85\x01\x91\x90\x91R\x81Q\x80\x85\x03\x82\x01\x81R`\x80\x85\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\x01`\x01`\xF8\x1B\x03\x19`\xA0\x84\x01R\x90\x85\x90\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\xA1\x83\x01R`\xB5\x82\x01R\x7F\xE3O\x19\x9B\x19\xB2\xB4\xF4\x7FhD&\x19\xD5UR}$Ox\xA3)~\xA8\x93%\xF8C\xF8{\x8BT`\xD5\x82\x01R`\xF5\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x93\x92PPPV[_a\n&\x82`\x14a\x11\xAFV[\x83Q\x10\x15a\nnW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RttoAddress_outOfBounds`X\x1B`D\x82\x01R`d\x01a\x01`V[P\x01` \x01Q`\x01``\x1B\x90\x04\x90V[_\x81a\n\x8B\x81`\x03a\x11\xAFV[\x10\x15a\n\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01RptoUint24_overflow`x\x1B`D\x82\x01R`d\x01a\x01`V[a\n\xD8\x82`\x03a\x11\xAFV[\x83Q\x10\x15a\x0B\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01RstoUint24_outOfBounds``\x1B`D\x82\x01R`d\x01a\x01`V[P\x01`\x03\x01Q\x90V[``\x81a\x0B6\x81`\x1Fa\x11\xAFV[\x10\x15a\x0BuW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rmslice_overflow`\x90\x1B`D\x82\x01R`d\x01a\x01`V[a\x0B\x7F\x82\x84a\x11\xAFV[\x84Q\x10\x15a\x0B\xC3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rpslice_outOfBounds`x\x1B`D\x82\x01R`d\x01a\x01`V[``\x82\x15\x80\x15a\x0B\xE1W`@Q\x91P_\x82R` \x82\x01`@Ra\x0C+V[`@Q\x91P`\x1F\x84\x16\x80\x15` \x02\x81\x84\x01\x01\x85\x81\x01\x87\x83\x15` \x02\x84\x8B\x01\x01\x01[\x81\x83\x10\x15a\x0C\x1AW\x80Q\x83R` \x92\x83\x01\x92\x01a\x0C\x02V[PP\x85\x84R`\x1F\x01`\x1F\x19\x16`@RP[P\x94\x93PPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0CJW__\xFD[\x91\x90PV[_` \x82\x84\x03\x12\x15a\x0C_W__\xFD[a\x0Ch\x82a\x0C4V[\x93\x92PPPV[__`@\x83\x85\x03\x12\x15a\x0C\x80W__\xFD[a\x0C\x89\x83a\x0C4V[\x94` \x93\x90\x93\x015\x93PPPV[\x80\x15\x15\x81\x14a\x0C\xA4W__\xFD[PV[\x805a\x0CJ\x81a\x0C\x97V[__\x83`\x1F\x84\x01\x12a\x0C\xC2W__\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\xD9W__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x0C\xF0W__\xFD[\x92P\x92\x90PV[_________a\x01\0\x8A\x8C\x03\x12\x15a\r\x10W__\xFD[a\r\x19\x8Aa\x0C4V[\x98Pa\r'` \x8B\x01a\x0C4V[\x97Pa\r5`@\x8B\x01a\x0C4V[\x96P``\x8A\x015\x95P`\x80\x8A\x015\x94Pa\rQ`\xA0\x8B\x01a\x0C4V[\x93P`\xC0\x8A\x015a\ra\x81a\x0C\x97V[\x92P`\xE0\x8A\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r|W__\xFD[a\r\x88\x8C\x82\x8D\x01a\x0C\xB2V[\x91P\x80\x93PP\x80\x91PP\x92\x95\x98P\x92\x95\x98P\x92\x95\x98V[____``\x85\x87\x03\x12\x15a\r\xB2W__\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\xD6W__\xFD[a\r\xE2\x87\x82\x88\x01a\x0C\xB2V[\x95\x98\x94\x97P\x95PPPPV[` \x80\x82R`\x0C\x90\x82\x01Rk\x15S\x90UU\x12\x13\xD4\x92V\x91Q`\xA2\x1B`@\x82\x01R``\x01\x90V[_` \x82\x84\x03\x12\x15a\x0E$W__\xFD[\x81Qa\x0Ch\x81a\x0C\x97V[_` \x82\x84\x03\x12\x15a\x0E?W__\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03_\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x0EyWa\x0Eya\x0EFV[P\x92\x91PPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Qa\x01\0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0E\xB8Wa\x0E\xB8a\x0E\x80V[`@R\x90V[_\x82`\x1F\x83\x01\x12a\x0E\xCDW__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\xE7Wa\x0E\xE7a\x0E\x80V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0F\x16Wa\x0F\x16a\x0E\x80V[`@R\x81\x81R\x83\x82\x01` \x01\x85\x10\x15a\x0F-W__\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_` \x82\x84\x03\x12\x15a\x0FYW__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0FoW__\xFD[\x82\x01a\x01\0\x81\x85\x03\x12\x15a\x0F\x81W__\xFD[a\x0F\x89a\x0E\x94V[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F\x9FW__\xFD[a\x0F\xAB\x86\x82\x85\x01a\x0E\xBEV[\x82RPa\x0F\xBA` \x83\x01a\x0C4V[` \x82\x01Ra\x0F\xCB`@\x83\x01a\x0C4V[`@\x82\x01Ra\x0F\xDC``\x83\x01a\x0C4V[``\x82\x01R`\x80\x82\x81\x015\x90\x82\x01R`\xA0\x80\x83\x015\x90\x82\x01Ra\x10\x01`\xC0\x83\x01a\x0C4V[`\xC0\x82\x01Ra\x10\x12`\xE0\x83\x01a\x0C\xA7V[`\xE0\x82\x01R\x94\x93PPPPV[_`\x01`\xFF\x1B\x82\x01a\x103Wa\x103a\x0EFV[P_\x03\x90V[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\t.Wa\t.a\x0EFV[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\t.Wa\t.a\x0EFV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_\x82Qa\x01\0` \x84\x01Ra\x10\xC2a\x01 \x84\x01\x82a\x10wV[\x90P`\x01\x80`\xA0\x1B\x03` \x85\x01Q\x16`@\x84\x01R`\x01\x80`\xA0\x1B\x03`@\x85\x01Q\x16``\x84\x01R``\x84\x01Qa\x11\x02`\x80\x85\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\x80\x84\x01Q`\xA0\x84\x01R`\xA0\x84\x01Q`\xC0\x84\x01R`\xC0\x84\x01Qa\x111`\xE0\x85\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\xE0\x84\x01Q\x80\x15\x15a\x01\0\x85\x01RP\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R\x85\x15\x15` \x83\x01R`@\x82\x01\x85\x90R\x83\x16``\x82\x01R`\xA0`\x80\x82\x01\x81\x90R_\x90a\x11\x82\x90\x83\x01\x84a\x10wV[\x97\x96PPPPPPPV[__`@\x83\x85\x03\x12\x15a\x11\x9EW__\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[\x80\x82\x01\x80\x82\x11\x15a\t.Wa\t.a\x0EFV[\x81\x81\x03\x81\x81\x11\x15a\t.Wa\t.a\x0EFV\xFE\xA2dipfsX\"\x12 ;\x1C\xF2\x8F\xBEx \x1D/\xEA\xD8\rDc(\xFEj\x0E2*\xF7\xEDn{=i\x98Y\x82R\x86\xFDdsolcC\0\x08\x1C\x003";
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
        ///Calls the contract's `approvePool` (0x424c265b) function
        pub fn approve_pool(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 76, 38, 91], token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `liquidate` (0xe8aed527) function
        pub fn liquidate(
            &self,
            collateral: ::ethers::core::types::Address,
            debt_asset: ::ethers::core::types::Address,
            user: ::ethers::core::types::Address,
            debt_to_cover: ::ethers::core::types::U256,
            liquidated_collateral_amount: ::ethers::core::types::U256,
            liquidator: ::ethers::core::types::Address,
            receive_a_token: bool,
            swap_path: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash(
                    [232, 174, 213, 39],
                    (
                        collateral,
                        debt_asset,
                        user,
                        debt_to_cover,
                        liquidated_collateral_amount,
                        liquidator,
                        receive_a_token,
                        swap_path,
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
            OwnershipTransferredFilter,
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
    ///Container type for all input parameters for the `liquidate` function with signature `liquidate(address,address,address,uint256,uint256,address,bool,bytes)` and selector `0xe8aed527`
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
        abi = "liquidate(address,address,address,uint256,uint256,address,bool,bytes)"
    )]
    pub struct LiquidateCall {
        pub collateral: ::ethers::core::types::Address,
        pub debt_asset: ::ethers::core::types::Address,
        pub user: ::ethers::core::types::Address,
        pub debt_to_cover: ::ethers::core::types::U256,
        pub liquidated_collateral_amount: ::ethers::core::types::U256,
        pub liquidator: ::ethers::core::types::Address,
        pub receive_a_token: bool,
        pub swap_path: ::ethers::core::types::Bytes,
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
        ApprovePool(ApprovePoolCall),
        Liquidate(LiquidateCall),
        Owner(OwnerCall),
        Pool(PoolCall),
        Recover(RecoverCall),
        TransferOwnership(TransferOwnershipCall),
        UniswapV3SwapCallback(UniswapV3SwapCallbackCall),
    }
    impl ::ethers::core::abi::AbiDecode for LiquidatorCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <ApprovePoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ApprovePool(decoded));
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
                Self::ApprovePool(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Liquidate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Recover(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::ApprovePool(element) => ::core::fmt::Display::fmt(element, f),
                Self::Liquidate(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pool(element) => ::core::fmt::Display::fmt(element, f),
                Self::Recover(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UniswapV3SwapCallback(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<ApprovePoolCall> for LiquidatorCalls {
        fn from(value: ApprovePoolCall) -> Self {
            Self::ApprovePool(value)
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
    ///Container type for all return fields from the `liquidate` function with signature `liquidate(address,address,address,uint256,uint256,address,bool,bytes)` and selector `0xe8aed527`
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
        pub collateral_gain: ::ethers::core::types::I256,
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
