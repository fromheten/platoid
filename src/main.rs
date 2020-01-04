extern crate base_x;
extern crate sha3;
use crate::sha3::Digest;
use std::io::Read;
fn main() {
    // Get the input as a Vec<u8>
    let mut input_mut: Vec<u8> = Vec::new();
    use std::io;
    for byte in io::stdin().bytes() {
        input_mut.push(
            byte.unwrap()
        );
    }
    // Hash it
    let mut hasher = sha3::Sha3_256::new();
    hasher.input(input_mut);
    let input: Vec<u8> = hasher
        .result()
        .as_slice()
        .to_vec();
    // Convert it to base "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
    let encoded = base_x::encode(
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789",
        &input
    );
    // Concat ID String
    let output = format!("plato0:{}", encoded);
    // Print it
    println!("{}", &output);
}
