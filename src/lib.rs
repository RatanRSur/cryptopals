pub fn hex_to_bytes(hex: &str) -> Vec<u8> {
    // Make vector of bytes from octets
    let mut bytes = Vec::new();
    for i in 0..(hex.len() / 2) {
        let res = u8::from_str_radix(&hex[2 * i..2 * i + 2], 16);
        match res {
            Ok(v) => bytes.push(v),
            Err(e) => println!("Problem with hex: {}", e),
        };
    }
    bytes
}

pub fn hex_to_base64(hex: &str) -> String {
    base64::encode(&hex_to_bytes(hex)) // now convert from Vec<u8> to b64-encoded String
}

pub fn xor(a: &str, b: &str) -> String {
    let a_bytes = hex_to_bytes(a);
    let b_bytes = hex_to_bytes(b);
    hex::encode(
        a_bytes
            .iter()
            .zip(b_bytes)
            .map(|(a_byte, b_byte)| a_byte ^ b_byte)
            .collect::<Vec<u8>>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hex_to_base64() {
        assert_eq!(hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"), "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    }

    #[test]
    fn test_xor() {
        assert_eq!(
            xor(
                "1c0111001f010100061a024b53535009181c",
                "686974207468652062756c6c277320657965"
            ),
            "746865206b696420646f6e277420706c6179"
        )
    }
}
