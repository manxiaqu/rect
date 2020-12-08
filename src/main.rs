mod args;
mod params;
pub mod tx;
mod utils;
use args::Arg as TraitArg;
use clap::App;
use web3::types::{Bytes, TransactionParameters, U256};

#[tokio::main]
async fn main() -> Result<(), ()> {
    let matches = App::new("rect")
        .about("Cmd tool interact with ethereum")
        .version("0.1.0")
        .author("manxiaqu")
        .arg(args::RPCArg::new())
        .subcommand(
            App::new("tx")
                .about("send tx")
                .arg(args::PrivateKeyArg::new())
                .arg(args::ReceiverArg::new())
                .arg(args::ValueArg::new())
                .arg(args::DataArg::new())
                .arg(args::GasArg::new()),
        )
        .get_matches();
    match matches.subcommand() {
        Some(("tx", tx_matches)) => {
            let transport = args::RPCArg::parse_get(&tx_matches).unwrap();
            let tx_manager = tx::Tx::new(web3::Web3::new(transport));

            let secret_key = args::PrivateKeyArg::parse_get(&tx_matches).unwrap();
            let tx = TransactionParameters {
                to: args::ReceiverArg::parse_get(&tx_matches),
                value: args::ValueArg::parse_get(&tx_matches).unwrap_or(U256::default()),
                gas: args::GasArg::parse_get(&tx_matches).unwrap_or(100000u64.into()),
                data: args::DataArg::parse_get(&tx_matches).unwrap_or(Bytes::default()),
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
