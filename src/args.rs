use crate::errors::Error;
use crate::params;
use crate::utils;
use clap::{Arg as ClapArg, ArgMatches};
use core::str::FromStr;
use secp256k1::SecretKey;
use web3::transports::Http;
use web3::types::{Address, Bytes, U256};

/// Arg represents a single param flag in cmd
pub trait Arg<'help> {
    type Out;

    fn new() -> ClapArg<'help>;

    fn parse_get(matches: &ArgMatches) -> Result<Self::Out, Error>;
}

macro_rules! create_arg {
    ($flag: ident, $out: ident, $init: expr, $get: expr) => {
        pub struct $flag {}

        impl<'help> Arg<'help> for $flag {
            type Out = $out;

            fn new() -> ClapArg<'help> {
                $init
            }

            fn parse_get(matches: &ArgMatches) -> Result<Self::Out, Error> {
                $get(matches)
            }
        }
    };
}

create_arg!(
    RPCArg,
    Http,
    ClapArg::new("rpc")
        .about("Url of ethereum rpc node")
        .default_value(params::RPC_LOCAL)
        .long("rpc")
        .value_name("rpc")
        .global(true),
    |ms: &ArgMatches| web3::transports::Http::new(ms.value_of("rpc").unwrap())
        .map_err(|err| err.into())
);

create_arg!(
    ReceiverArg,
    Address,
    ClapArg::new("to")
        .about("Receiver address")
        .long("to")
        .value_name("to")
        .validator(utils::validate_address),
    |ms: &ArgMatches| {
        if let Some(to) = ms.value_of("to") {
            let to = utils::strip_hex_prefix(to);
            Ok(Address::from_str(&to).unwrap())
        } else {
            return Ok(Address::default());
        }
    }
);

create_arg!(
    PrivateKeyArg,
    SecretKey,
    ClapArg::new("priv")
        .about("The private key used to send transaction")
        .long("priv")
        .value_name("priv")
        .validator(utils::validate_private_key),
    |ms: &ArgMatches| {
        ms.value_of("priv")
            .map(|k| utils::strip_hex_prefix(k))
            .map(|s| SecretKey::from_str(&s).unwrap())
            .ok_or(Error::Unexpected)
    }
);

create_arg!(
    ValueArg,
    U256,
    ClapArg::new("value")
        .about("Value sent to receiver")
        .long("value")
        .value_name("value"),
    |ms: &ArgMatches| {
        if let Some(v) = ms.value_of("value") {
            return utils::str_to_u256(&v)
                .map_err(|err| Error::InvalidValue(v.into(), "value".into(), err));
        } else {
            return Ok(U256::default());
        }
    }
);

create_arg!(
    DataArg,
    Bytes,
    ClapArg::new("data")
        .about("extra data(optional for calling contract)")
        .long("data")
        .value_name("data"),
    |ms: &ArgMatches| {
        // if it's a hex string, then decode it
        if let Some(data) = ms.value_of("data") {
            if data.starts_with("0x") || data.starts_with("0X") {
                if let Some(hex_data) = hex::decode(utils::strip_hex_prefix(data)).ok() {
                    return Ok(Bytes::from(hex_data));
                } else {
                    return Err(Error::InvalidValue(
                        format!("hex string({:?})", data),
                        "data".into(),
                        "Invalid hex string".into(),
                    ));
                }
            }
        }

        // use original data
        Ok(vec![].into())
    }
);

create_arg!(
    GasArg,
    U256,
    ClapArg::new("gas")
        .about("gas limit used for tx(optional)")
        .long("gas")
        .value_name("gas"),
    |ms: &ArgMatches| {
        if let Some(gas) = ms.value_of("gas") {
            return utils::str_to_u256(gas)
                .map_err(|e| Error::InvalidValue(gas.into(), "gas".into(), e));
        } else {
            return Ok(100000u64.into());
        }
    }
);
