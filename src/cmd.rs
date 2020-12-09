use crate::args::*;
use crate::errors::Error;
use crate::tx;
use clap::{App, ArgMatches};
use web3::types::{Bytes, TransactionParameters, U256};

pub trait Command<'help> {
    fn new() -> App<'help>;

    fn run(matches: &ArgMatches) -> Result<(), Error>;
}

#[derive(Debug)]
pub struct SendTxCmd {}

impl<'help> Command<'help> for SendTxCmd {
    fn new() -> App<'help> {
        App::new("tx").about("send tx").args(&[
            PrivateKeyArg::new(),
            ReceiverArg::new(),
            ValueArg::new(),
            DataArg::new(),
            GasArg::new(),
        ])
    }

    fn run(ms: &ArgMatches) -> Result<(), Error> {
        let transport = RPCArg::parse_get(&ms)?;
        let tx_manager = tx::Tx::new(web3::Web3::new(transport));

        let secret_key = PrivateKeyArg::parse_get(&ms)?;
        let tx = TransactionParameters {
            to: ReceiverArg::parse_get(&ms).ok(),
            value: ValueArg::parse_get(&ms)?,
            gas: GasArg::parse_get(&ms)?,
            data: DataArg::parse_get(&ms)?,
            ..TransactionParameters::default()
        };
        tx_manager.sign_and_send_raw_transaction(tx, &secret_key)?;

        Ok(())
    }
}
