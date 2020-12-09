extern crate derive_more;
use derive_more::Display;
use web3::Error as Web3Error;

#[derive(Debug, Display)]
pub enum Error {
    /// invalid rpc url or service is unreachable
    #[display(fmt = "server is unreachable")]
    RpcUnreachable,
    /// send tx failed
    TxFailed,
    /// invalid params value
    #[display(fmt = "{} is not a valid value for {}, err: {:?}", _0, _1, _2)]
    InvalidValue(String, String, String),
    #[display(fmt = "others: {}", _0)]
    Others(String),
    /// unexpected error, always means a bug
    Unexpected,
}

impl From<Web3Error> for Error {
    fn from(err: Web3Error) -> Self {
        match err {
            Web3Error::Unreachable => Error::RpcUnreachable,
            _ => Error::Others(format!("{:?}", err)),
        }
    }
}
