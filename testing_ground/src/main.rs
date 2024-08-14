use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;
use rand::RngCore;
use std::fs::OpenOptions;
use std::io::Write;

fn derive_key(password: &[u8], salt: &[u8]) -> [u8; 32] {
    let mut key = [0u8; 32];
    pbkdf2_hmac::<Sha256>(password, salt, 600_000, &mut key);
    return key;
}


fn generate_salt() -> [u8; 16] {
    let mut salt = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut salt);
    return salt;
}


fn authenticate_derive() -> ([u8; 32], [u8; 16]) {

    let mut master_password = String::new();
    
    print!("enter master password: ");

    std::io::stdout().flush().expect("couldn't flush 3");
    
    std::io::stdin().read_line(&mut master_password).expect("couldn't read input");

    println!("{}", master_password);
    
    let salt = generate_salt();

    let key = derive_key(master_password.trim().as_bytes(), &salt);

    return (key, salt)

}



fn add_entry(data_name: &str, data: String, password: &str) {


    let (key, salt) = authenticate_derive();

    println!("data: {}", data);
   
    println!("salt: {:?}", salt);

    println!("{:?}", key);

  //  save the salt as the second line after the password


    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(data_name)
    .expect("couldn't create file data_name");

  //  the salt will be unique. one for each password to be stored encrypted

    file.write_all(salt).expect("Unable to write data");

    // ENCRYPT IT FIRST BEFORE YOU WRITE TO FILE VVV

    

    file.write_all(data.as_bytes()).expect("Unable to write data");
  //  encrypt with 256 aes (key derived is 32 bytes aka 32 bytes = 8 bits/byte * 32 bytes = 256 bits)



}

fn add_entry_handler() {
    
    let master_password = authenticate();

    let mut data_name = String::new();

    let mut data = String::new();

    print!("name your entry: ");

    std::io::stdout().flush().expect("couldn't flush 1");

    std::io::stdin().read_line(&mut data_name).expect("couldn't read input for entry name");

    print!("\nprovide data: ");
    
    std::io::stdout().flush().expect("couldn't flush 2");
    
    std::io::stdin().read_line(&mut data).expect("couldn't read input for entry data");

    println!("\n ---- o ---- o ---- o ---- \n adding entry");

    add_entry(data_name.trim(), data, master_password.trim());

}

fn main() {

    println!("welcome to stego-rs\nplease select an option:\n\n1 = add entry\n2 = read entry\n3 = delete entry");

    let mut option = String::new();

    std::io::stdin().read_line(&mut option).expect("couldn't read selected option");

    match option.trim() {
        "1" => add_entry_handler(),
        "2" => println!("read entry | selected"),
        "3" => println!("delete entry | selected"),
        _ => println!("that isn't an option")
    }

    



}
