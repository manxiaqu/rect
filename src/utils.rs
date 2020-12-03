use hex;

/// remove hex prefix `0x` or `0X` in string
pub fn strip_hex_prefix(ori: &str) -> String {
    if ori.starts_with("0x") || ori.starts_with("0X") {
        return ori.get(2..).unwrap().to_string();
    }

    ori.to_string()
}

/// check a hex string is private key or not(only 32 bytes allowed)
pub fn validate_private_key(val: &str) -> Result<(), String> {
    validate_hex_of_len(&val, 32).map_err(|err| format!("invalid private key: {:?}", err))?;

    Ok(())
}

/// check a hex string is address or not
pub fn validate_address(val: &str) -> Result<(), String> {
    validate_hex_of_len(&val, 20).map_err(|err| format!("invalid address: {:?}", err))?;

    Ok(())
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
