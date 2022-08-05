use encrypt::encryptor::{rot13, rsa, Cipher};
use std::io;

fn main() {
    println!("Input the string you'd like to encrypt: ");

    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("could not read user input");

    let encrypted_input = rsa::Rsa::new(user_input).expect("");
    let encrypted_string = encrypted_input.encrypted_string().expect("");

    println!(
        "Your encrypted string is: {}",
        encrypted_string
    );

    let decrypted_string = encrypted_input.original_string().expect("");
    println!(
        "Your original string is: {}",
        decrypted_string
    );
}
