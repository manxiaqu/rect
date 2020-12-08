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

    fn parse_get(matches: &ArgMatches) -> Option<Self::Out>;
}

macro_rules! create_arg {
    ($flag: ident, $out: ident, $init: expr, $get: expr) => {
        pub struct $flag {}

        impl<'help> Arg<'help> for $flag {
            type Out = $out;

            fn new() -> ClapArg<'help> {
                $init
            }

            fn parse_get(matches: &ArgMatches) -> Option<Self::Out> {
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
    |ms: &ArgMatches| ms
        .value_of("rpc")
        .map(|v| web3::transports::Http::new(v).unwrap())
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
        ms.value_of("to")
            .map(|k| utils::strip_hex_prefix(k))
            .map(|v| Address::from_str(&v).unwrap())
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
    }
);

create_arg!(
    ValueArg,
    U256,
    ClapArg::new("value")
        .about("Value sent to receiver")
        .long("value")
        .value_name("value"),
    |ms: &ArgMatches| ms.value_of("value").map(|v| utils::str_to_u256(v).unwrap())
);

create_arg!(
    DataArg,
    Bytes,
    ClapArg::new("data")
        .about("extra data(optional for calling contract)")
        .long("data")
        .value_name("data"),
    |ms: &ArgMatches| {
        log::debug!("data is {:?}", ms.value_of("data"));
        // if it's a hex string, then decode it
        ms.value_of("data").map(|s| {
            if s.starts_with("0x") || s.starts_with("0X") {
                if let Some(hex_data) = hex::decode(utils::strip_hex_prefix(s)).ok() {
                    return Bytes::from(hex_data);
                }
            }

            // use original data
            Bytes::from(s)
        })
    }
);

create_arg!(
    GasArg,
    U256,
    ClapArg::new("gas")
        .about("gas limit used for tx(optional)")
        .long("gas")
        .value_name("gas"),
    |ms: &ArgMatches| ms.value_of("gas").map(|v| utils::str_to_u256(v).unwrap())
);
