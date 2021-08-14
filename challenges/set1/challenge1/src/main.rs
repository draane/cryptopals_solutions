use std::io;

// TODO: replace these crates with my own implementations
use hex::FromHex;
use base64;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Please insert a hex string");

    let input: String = String::from(input.trim());

    println!("{}", hex2base64(input));
}

fn hex2base64 (hexstr: String) -> String {
    let bytes: Vec<u8> = Vec::from_hex(hexstr).expect("Please insert a valid hex string");

    base64::encode(&bytes)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_test() {
        let hexstr = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
        
        assert_eq!(hex2base64(hexstr), String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"));
    }
}
