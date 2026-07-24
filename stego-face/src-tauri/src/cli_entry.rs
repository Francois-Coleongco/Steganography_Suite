use aes_gcm::{
    aead::{generic_array::GenericArray, AeadCore, KeyInit, OsRng, Aead},
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
    let key = derive_key(master_password.trim().as_bytes(), &salt);
    master_password.zeroize();
    (key, salt)
}

fn authenticate_derive(mut master_password: String, salt: [u8; 16]) -> [u8; 32] {
    let key = derive_key(master_password.trim().as_bytes(), &salt);
    master_password.zeroize();
    key
}

fn encrypt(
    key: &[u8; 32],
    mut data: String,
) -> (
    Vec<u8>,
    GenericArray<u8, <Aes256Gcm as AeadCore>::NonceSize>,
) {
    let cipher = Aes256Gcm::new(key.into());
    let nonce = Aes256Gcm::generate_nonce(OsRng);
    let ciphertext = cipher
        .encrypt(&nonce, data.as_bytes())
        .expect("encryption failed");
    data.zeroize();
    (ciphertext, nonce)
}

fn decrypt(
    key: &[u8; 32],
    nonce: &GenericArray<u8, <Aes256Gcm as AeadCore>::NonceSize>,
    ciphertext: &[u8],
) -> String {
    let cipher = Aes256Gcm::new(key.into());
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .expect("decryption failed");
    String::from_utf8(plaintext).expect("plaintext is not valid UTF-8")
}

pub fn add_entry(master_password: String, data: String, file_path: String) -> Result<String, String> {
    println!("[add_entry] file_path: {}", file_path);
    if !file_path.ends_with(".png") {
        return Err(format!("File must be a .png, got: {}", file_path));
    }

    let (mut key, master_salt) = authenticate_derive_init(master_password);
    println!("[add_entry] key derived, salt generated");
    let (ciphertext, nonce) = encrypt(&key, data);
    println!("[add_entry] encrypted, ciphertext len: {}", ciphertext.len());
    stego::encoder(master_salt, nonce, ciphertext, file_path.clone())?;
    key.zeroize();

    let mut stego_path = file_path;
    if let Some(pos) = stego_path.rfind(".") {
        stego_path.insert_str(pos, ".stego");
    }
    println!("[add_entry] done, stego_path: {}", stego_path);
    Ok(stego_path)
}

fn read_entry(master_password: String, data: Vec<u8>) -> Result<String, String> {
    println!("[read_entry] raw data len: {}", data.len());
    let salt: &[u8; 16] = &data[0..16]
        .try_into()
        .map_err(|_| "Invalid salt length".to_string())?;

    let key = authenticate_derive(master_password, *salt);
    println!("[read_entry] key derived from salt");

    let nonce: GenericArray<u8, <Aes256Gcm as AeadCore>::NonceSize> =
        GenericArray::clone_from_slice(&data[16..28]);

    let decrypted = decrypt(&key, &nonce, &data[28..]);
    println!("[read_entry] decrypted, len: {}", decrypted.len());
    Ok(decrypted)
}

pub fn read_entry_handler(master_password: String, file_path: &str) -> Result<String, String> {
    println!("[read_entry_handler] file_path: {}", file_path);
    let data = stego::decoder(file_path)?;
    println!("[read_entry_handler] decoded {} bytes from image", data.len());
    read_entry(master_password, data)
}
