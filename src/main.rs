mod args;
mod cmd;
mod errors;
mod params;
pub mod tx;
mod utils;
use args::Arg as TraitArg;
use clap::App;
use cmd::Command;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let matches = App::new("rect")
        .about("Cmd tool interact with ethereum")
        .version("0.1.0")
        .author("manxiaqu")
        .arg(args::RPCArg::new())
        .subcommand(cmd::SendTxCmd::new())
        .get_matches();

    match matches.subcommand() {
        Some(("tx", tx_matches)) => {
            let r = cmd::SendTxCmd::run(&tx_matches);
            if r.is_err() {
                log::error!("send tx failed: {:?}", r.unwrap_err());
            }
        }
        // print help instead
        _ => unreachable!(),
    }
    Ok(())
}
