use core::str::FromStr;
use hex;
use web3::types::U256;

/// remove hex prefix `0x` or `0X` in string
pub fn strip_hex_prefix(ori: &str) -> String {
    if ori.starts_with("0x") || ori.starts_with("0X") {
        return ori.get(2..).unwrap().to_string();
    }

    ori.to_string()
}

/// convert str to u256
///
/// the str can be number 100 or hex format 0x100
pub fn str_to_u256(v: &str) -> Option<U256> {
    if v.starts_with("0x") || v.starts_with("0X") {
        return U256::from_str(&strip_hex_prefix(v)).ok();
    }

    // treat as a number
    v.parse::<u128>().map(|n| n.into()).ok()
}

/// validate hex string with expect bytes length
///
/// the `0x` or `0X` will be ignored when counting length
/// of hex string
fn validate_hex_of_len(val: &str, len: usize) -> Result<(), String> {
    let mut start_index: usize = 0;
    if val.starts_with("0x") || val.starts_with("0X") {
        start_index = 2;
    }
    let str_bytes = hex::decode(
        val.get(start_index..)
            .ok_or(format!("invalid hex string: bytes length isn't {}", len))?,
    )
    .map_err(|err| format!("invalid hex string: {:?}", err))?;

    if str_bytes.len() != len {
        return Err(format!("invalid hex string: bytes length isn't {}", len));
    }

    Ok(())
}

macro_rules! create_hex_validator {
    ($fn_name: ident, $len: expr, $msg: literal) => {
        pub fn $fn_name(val: &str) -> Result<(), String> {
            validate_hex_of_len(&val, $len).map_err(|err| format!("{}: {:?}", $msg, err))?;

            Ok(())
        }
    };
}

create_hex_validator!(validate_private_key, 32, "invalid private key");
create_hex_validator!(validate_address, 20, "invalid address");
create_hex_validator!(validate_hash, 32, "invalid hash");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_hex_of_len_test() {
        validate_hex_of_len(
            "0x90b1498fcac1911f91cd650dd4091b36d32e728eb8c5be611af35e3d3e04dd7d",
            32,
        )
        .unwrap();
        validate_hex_of_len(
            "90b1498fcac1911f91cd650dd4091b36d32e728eb8c5be611af35e3d3e04dd7d",
            32,
        )
        .unwrap();
        validate_hex_of_len("0xAB4b65661c2E6061321ab68Dd2741132F41D3d19", 20).unwrap();
        validate_hex_of_len("AB4b65661c2E6061321ab68Dd2741132F41D3d19", 20).unwrap();
    }

    #[test]
    fn strip_hex_prefix_test() {
        assert_eq!(strip_hex_prefix("0x00"), "00");
        assert_eq!(strip_hex_prefix("0X00"), "00");
        assert_eq!(strip_hex_prefix("0X"), "");
        assert_eq!(strip_hex_prefix("0x"), "");
    }
}
