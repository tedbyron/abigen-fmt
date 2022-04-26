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

    let contract_call = contract_instance.takes_ints_returns_bool(42_u32, 10_u16);

    let encoded = format!(
        "{}{}",
        encode(contract_call.encoded_selector),
        encode(contract_call.encoded_args)
    );

    assert_eq!("0000000003b568d4000000000000002a000000000000000a", encoded);
    dbg!(encoded);
}
