use std::io;

// TODO: replace these crates with my own implementations
use hex::FromHex;

fn main() {
    let mut input = String::new();

    print!("Insert first string: ");    
    io::stdin()
        .read_line(&mut input)
        .expect("Please insert a hex string");
        
    let str1: String = String::from(input.trim());
    
    
    print!("Insert second string: ");    
    io::stdin()
        .read_line(&mut input)
        .expect("Please insert a hex string");
        
    let str2: String = String::from(input.trim());
    
    let bytes2: Vec<u8> = Vec::from_hex().expect("Please insert a valid hex string");

    println!("{}", compute_xor(input));
}


fn compute_xor(str1: String, str2: String) -> String {
    let bytes1: Vec<u8> = Vec::from_hex(str1).expect("Invalid valid hex string: {}", str1);
    let bytes2: Vec<u8> = Vec::from_hex(str2).expect("Invalid valid hex string: {}", str2);
    
    xor(bytes1, bytes2)::to_hex()
}


fn xor (b1: &Vec<u8>, b2: &Vec<u8>) -> Vec<u8> {
    assert_eq!(b1.len(), b2.len(), "Different len"); 

    let mut res = Vec::new();
    
    for index in (0..b1.len()-1) {
        res.push(b1[index] ^ b2[index]);
    }

    res
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_test() {
        let hexstr1 = String::from("1c0111001f010100061a024b53535009181c");        
        let hexstr1 = String::from("686974207468652062756c6c277320657965");
        
        assert_eq!(compute_xor(hexstr1, hexstr2), String::from("746865206b696420646f6e277420706c6179"));
    }
}
