use aead::generic_array::GenericArray;
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, AesGcm, Key, Nonce,
};
use pbkdf2::pbkdf2_hmac;
use rand::RngCore;
use sha2::Sha256;
use std::fs::OpenOptions;
use std::io::Write;
use zeroize::Zeroize;

fn derive_key(password: &[u8], salt: &[u8]) -> [u8; 32] {
    let mut key = [0u8; 32];
    pbkdf2_hmac::<Sha256>(password, salt, 600_000, &mut key);
    key
}

fn generate_salt() -> [u8; 16] {
    let mut salt = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut salt);
    salt
}

fn authenticate_derive() -> [u8; 32] {
    let mut master_password = String::new();

    print!("enter master password: ");

    std::io::stdout().flush().expect("couldn't flush 3");

    std::io::stdin()
        .read_line(&mut master_password)
        .expect("couldn't read input");

    println!("{}", master_password);

    let salt = generate_salt();

    let key = derive_key(master_password.trim().as_bytes(), &salt); // master_password the only
                                                                    // thing that needs to be zeroized here because the as_bytes() function turns the output of
                                                                    // master_password.trim() into a slice which is a mem reference

    master_password.zeroize(); // erase the memory of the user's master password from memory once
                               // all uses of the master_password (aka the key derivation) are complete.

    key
}

fn encrypt(
    key: &[u8; 32],
    data: &String,
) -> (
    Vec<u8>,
    GenericArray<u8, <Aes256Gcm as AeadCore>::NonceSize>,
) {
    // encrypt data
    //
    // data is originally a string from the user and is passed to this function.
    //
    // remember to zeroize the key and data variable (plain text) after all encryption processes
    // are compelte
    //

    let cipher = Aes256Gcm::new(key.into());

    let nonce = Aes256Gcm::generate_nonce(OsRng);

    let ciphertext = cipher
        .encrypt(&nonce, data.as_bytes())
        .expect("couldn't convert to ciphertext");

    (ciphertext, nonce)
}

fn decrypt(
    key: [u8; 32],
    nonce: GenericArray<u8, <Aes256Gcm as AeadCore>::NonceSize>,
    encrypted_data: String,
) {
    // encrypted_data is originally bytes from a file that are converted to a string.
    //
    // remember to zeroize key and decrypted data variable after all decryption proccesses are
    // complete<F12>
    let cipher = Aes256Gcm::new(&key.into());
    let plaintext = cipher.decrypt(&nonce, encrypted_data.as_bytes());

    println!("{:?}", plaintext);
}

fn add_entry(data_name: &str, data: &mut String) {
    let key = authenticate_derive();

    // when writing data make sure to zeroize it after
    //  save the salt as the second line after the password

    let (ciphertext, nonce) = encrypt(&key, &data.to_string());

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(data_name)
        .expect("couldn't create file data_name");

    //  the salt will be unique. one for each password to be stored encrypted

    // ENCRYPT IT FIRST BEFORE YOU WRITE TO FILE VVV

    file.write_all(data.as_bytes())
        .expect("Unable to write data");
    //  encrypt with 256 aes (key derived is 32 bytes aka 32 bytes = 8 bits/byte * 32 bytes = 256 bits)
    //
    //
    data.zeroize();
    data_name.zeroize();
}

fn add_entry_handler() {
    let mut data_name = String::new();

    let mut data = String::new();

    print!("name your entry: ");

    std::io::stdout().flush().expect("couldn't flush 1");

    std::io::stdin()
        .read_line(&mut data_name)
        .expect("couldn't read input for entry name");

    print!("\nprovide data: ");

    std::io::stdout().flush().expect("couldn't flush 2");

    std::io::stdin()
        .read_line(&mut data)
        .expect("couldn't read input for entry data");

    add_entry(&data_name, &mut data);
}

fn main() {
    println!("welcome to stego-rs\nplease select an option:\n\n1 = add entry\n2 = read entry\n3 = delete entry");

    let mut option = String::new();

    std::io::stdin()
        .read_line(&mut option)
        .expect("couldn't read selected option");

    match option.trim() {
        "1" => add_entry_handler(),
        "2" => println!("read entry | selected"),
        "3" => println!("delete entry | selected"),
        _ => println!("that isn't an option"),
    }
}
