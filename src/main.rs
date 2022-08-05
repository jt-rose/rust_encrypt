use encrypt::encryptor::{rot13, Encryptable};
use std::io;

fn main() {
    println!("Input the string you'd like to encrypt: ");

    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("could not read user input");

    println!(
        "Your encrypted string is: {}",
        rot13::Rot13(user_input).encrypt()
    );
}
