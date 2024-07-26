use pbkdf2::{pbkdf2_hmac};
use sha2::Sha256;
use rand::RngCore;

fn derive_key(password: &[u8], salt: &[u8]) -> [u8; 32] {
    let mut key = [0u8; 32];
    pbkdf2_hmac::<Sha256>(password, salt, 100_000, &mut key);
    return key
}


fn generate_salt() -> [u8; 16] {
    let mut salt = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut salt);
    salt
}





fn main() {

    let mut master_password = String::new();

    std::io::stdin().read_line(&mut master_password).expect("couldn't read input");

    println!("{}", master_password);

    let key = derive_key(master_password.trim().as_bytes(), &generate_salt());

    println!("{:?}", key);

    save the salt to a file in the same dir as the password. both files will have the same name but differing extensions.

    the salt will be unique. one for each password to be stored encrypted

    password1.txt

    password1.salt

    encrypt with 256 aes (key derived is 32 bytes aka 32 bytes = 8 bits/byte * 32 bytes = 256 bits)


}
