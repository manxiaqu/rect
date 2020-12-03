mod params;
pub mod tx;
mod utils;
use clap::{App, Arg};
use core::str::FromStr;
use secp256k1::SecretKey;
use web3::types::{Address, TransactionParameters, U256};

#[tokio::main]
async fn main() -> Result<(), ()> {
    let matches = App::new("rect")
        .about("Cmd tool interact with ethereum")
        .version("0.1.0")
        .author("manxiaqu")
        .arg(
            Arg::new("rpc")
                .about("Url of ethereum rpc node")
                .default_value(params::RPC_LOCAL)
                .long("rpc")
                .value_name("rpc")
                .global(true),
        )
        .subcommand(
            App::new("tx")
                .about("send tx")
                .arg(
                    Arg::new("priv")
                        .about("The private key used to send transaction")
                        .long("priv")
                        .value_name("priv")
                        .validator(utils::validate_private_key),
                )
                .arg(
                    Arg::new("to")
                        .about("Receiver address")
                        .long("to")
                        .value_name("to")
                        .validator(utils::validate_address),
                )
                .arg(
                    Arg::new("value")
                        .about("Amount sent to receiver")
                        .long("value")
                        .value_name("value"),
                ),
        )
        .get_matches();
    match matches.subcommand() {
        Some(("tx", tx_matches)) => {
            let rpc_url = tx_matches.value_of("rpc").unwrap();
            let privatekey = utils::strip_hex_prefix(tx_matches.value_of("priv").unwrap());
            let to = utils::strip_hex_prefix(tx_matches.value_of("to").unwrap());
            let value = tx_matches.value_of("value").unwrap();

            let transport = web3::transports::Http::new(rpc_url).unwrap();
            let tx_manager = tx::Tx::new(web3::Web3::new(transport));

            let secret_key = SecretKey::from_str(&privatekey).unwrap();
            let tx = TransactionParameters {
                to: Some(Address::from_str(&to).unwrap()),
                value: U256::from_str(value).unwrap(),
                ..TransactionParameters::default()
            };
            tx_manager
                .sign_and_send_raw_transaction(tx, &secret_key)
                .unwrap();
        }
        _ => unreachable!(),
    }
    Ok(())
}
