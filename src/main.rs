use fuels_abigen_macro::abigen;
use fuels_rs::prelude::*;
use hex::encode;

#[tokio::main]
async fn main() {
    abigen!(
        SimpleContract,
        r#"
        [
            {
                "type": "contract",
                "inputs": [
                    {
                        "name": "arg",
                        "type": "u32"
                    },
                    {
                        "name": "second_arg",
                        "type": "u16"
                    }
                ],
                "name": "takes_ints_returns_bool",
                "outputs": [
                    {
                        "name": "",
                        "type": "bool"
                    }
                ]
            }
        ]
        "#
    );

    let (provider, wallet) = setup_test_provider_and_wallet().await;
    let contract_instance = SimpleContract::new(
        "0x0c0000000000000000000000000000000000000000000000000000000000000b1a".to_string(),
        provider,
        wallet,
    );

    // 0x0000000003b568d4
    let contract_call = contract_instance.takes_ints_returns_bool(42_u32, 10_u16);

    let encoded = format!(
        "{}{}",
        encode(contract_call.encoded_selector),
        encode(contract_call.encoded_args)
    );

    assert_eq!("0000000003b568d4000000000000002a000000000000000a", encoded);
    dbg!(encoded);
}

// macro expansion of abigen!
// --------------------------------------------------
// pub use simplecontract_mod::*;
// #[allow(clippy :: too_many_arguments)]
// mod simplecontract_mod {
//     #![allow(clippy :: enum_variant_names)]
//     #![allow(dead_code)]
//     #![allow(unused_imports)]
//     use fuel_tx::{ContractId, Address};
//     use fuels_rs::contract::contract::{Contract, ContractCall};
//     use fuels_rs::signers::{provider::Provider, LocalWallet};
//     use std::str::FromStr;
//     use fuels_rs::core::{
//         EnumSelector, ParamType, Tokenizable, Token,
//     };
//     pub struct SimpleContract {
//         contract_id: ContractId,
//         provider: Provider,
//         wallet: LocalWallet,
//     }
//     impl SimpleContract {
//         pub fn new(contract_id: String, provider: Provider,
//             wallet: LocalWallet) -> Self {
//             let contract_id =
//                 ContractId::from_str(&contract_id).unwrap();
//             Self { contract_id, provider, wallet }
//         }
//         #[doc =
//         "Calls the contract's `takes_ints_returns_bool` (0x0000000003b568d4) function"]
//         pub fn takes_ints_returns_bool(&self, arg: u32,
//             second_arg: u16) -> ContractCall<bool> {
//             Contract::method_hash(&self.provider, self.contract_id,
//                     &self.wallet, [0, 0, 0, 0, 3, 181, 104, 212],
//                     &[ParamType::Bool],
//                     &[arg.into_token(),
//                                 second_arg.into_token()]).expect("method not found (this should never happen)")
//         }
//     }
// }
