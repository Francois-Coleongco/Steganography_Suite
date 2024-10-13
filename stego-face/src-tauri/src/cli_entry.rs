use aes_gcm::{
    aead::{generic_array::GenericArray, Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm,
};
use pbkdf2::pbkdf2_hmac;
use rand::RngCore;
use sha2::Sha256;
use zeroize::Zeroize;

mod stego;

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

fn authenticate_derive_init(mut master_password: String) -> ([u8; 32], [u8; 16]) {
    let salt = generate_salt();

    let key = derive_key(master_password.trim().as_bytes(), &salt); // master_password the only

    master_password.zeroize(); // erase the memory of the user's master password from memory once
                               // all uses of the master_password (aka the key derivation) are complete.

    (key, salt) // feed salt into another function to save to the img file along with the data
} // ran when adding an entry

fn authenticate_derive(mut master_password: String, salt: [u8; 16]) -> [u8; 32] {
    let key = derive_key(master_password.trim().as_bytes(), &salt); // master_password the only

    master_password.zeroize();
    // get password in text
    //
    // use salt to derive a key
    //
    key // return the key
} // ran when reading an entry

fn encrypt(
    key: &[u8; 32],
    mut data: String,
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

    println!("key ;;;;; {:?}", key);
    let cipher = Aes256Gcm::new(key.into());

    let nonce = Aes256Gcm::generate_nonce(OsRng);
    let ciphertext = cipher
        .encrypt(&nonce, data.as_bytes())
        .expect("couldn't convert to ciphertext");

    println!("nonce in encrypt: {:?}", nonce);

    data.zeroize();

    (ciphertext, nonce)
}

fn decryptt(
    key: &[u8; 32],
    nonce: &GenericArray<u8, <Aes256Gcm as AeadCore>::NonceSize>,
    ciphertext: &Vec<u8>,
) -> String {
    // encrypted_data is originally bytes from a file that are converted to a string.
    //
    // remember to zeroize key and decrypted data variable after all decryption proccesses are
    // complete<F12>
    let cipher = Aes256Gcm::new(key.into());
    let plaintext_as_utf8 = cipher
        .decrypt(nonce, ciphertext.as_ref())
        .expect("")
        .to_ascii_lowercase();

    println!("{:?}", plaintext_as_utf8);

    println!("key ;;;;; {:?}", key);

    let final_msg = String::from_utf8(plaintext_as_utf8).expect("");

    final_msg
}

pub fn add_entry(master_password: String, data: String, file_path: String) {
    let (mut key, master_salt) = authenticate_derive_init(master_password); // write the salt to the file first
                                                                            // when writing data make sure to zeroize it after
                                                                            //  save the salt as the second line after the password
    let (ciphertext, nonce) = encrypt(&key, data);

    println!("ciphertext len: {}", ciphertext.len());

    println!("ciphertext from encrypt: {:?}", ciphertext);
    //  encrypt with 256 aes (key derived is 32 bytes aka 32 bytes = 8 bits/byte * 32 bytes = 256 bits)

    // writing the nonce first, so when stego decodes, the nonce will be the first 12 bytes
    // that were reconstructed
    stego::encoder(master_salt, nonce, ciphertext, file_path);
    key.zeroize();
}

fn read_entry(master_password: String, data: Vec<u8>) -> String {
    let salt: &[u8; 16] = &data[0..16]
        .try_into()
        .expect("couldn't convert salt slice to salt arr");

    let key = authenticate_derive(master_password, *salt);

    let nonce: GenericArray<u8, <Aes256Gcm as AeadCore>::NonceSize> =
        GenericArray::clone_from_slice(&data[16..28]);

    println!("{}", nonce.len());

    println!("nonce from read_entry: {:?}", nonce);

    // let ciphertext = &data[12..]

    println!("{:?}", &data.len());

    let decrypted_data = decryptt(&key, &nonce, &data[28..].to_vec());

    println!("{}", decrypted_data);
    decrypted_data
}

pub fn read_entry_handler(master_password: String, file_path: &String) -> String {
    let data = stego::decoder(file_path);

    return read_entry(master_password, data);
}
