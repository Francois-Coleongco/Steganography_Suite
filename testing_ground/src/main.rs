use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;
use rand::RngCore;
use std::fs::File;
use std::io::Write;

fn derive_key(password: &[u8], salt: &[u8]) -> [u8; 32] {
    let mut key = [0u8; 32];
    pbkdf2_hmac::<Sha256>(password, salt, 100_000, &mut key);
    return key;
}


fn generate_salt() -> [u8; 16] {
    let mut salt = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut salt);
    return salt;
}


fn authenticate() -> String {

    let mut master_password = String::new();

    std::io::stdin().read_line(&mut master_password).expect("couldn't read input");

    println!("{}", master_password);

    return master_password;


}



fn add_entry(data_name: String, data: String, password: String) {

    let salt = &generate_salt();

    let key = derive_key(password.trim().as_bytes(), salt);

    println!("salt: {:?}", salt);

    println!("{:?}", key);

  //  save the salt as the second line after the password

    let mut file = File::create(data_name).expect("couldn't create file");
  //  the salt will be unique. one for each password to be stored encrypted

    file.write_all(&key).expect("Unable to write data");

  //  encrypt with 256 aes (key derived is 32 bytes aka 32 bytes = 8 bits/byte * 32 bytes = 256 bits)



}

fn main() {

    let master_password = authenticate();

    

}
