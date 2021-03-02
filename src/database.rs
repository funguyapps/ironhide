use rusqlite::{ NO_PARAMS, Connection };
use chacha20poly1305::{ ChaCha20Poly1305, Key, Nonce };
use chacha20poly1305::aead::{ Aead, NewAead };

const NONCE: &'static str = "unique nonce";

pub fn list_all(password: &String) {
    println!();
    let connection = Connection::open(".data.db").unwrap();

    let cipher = construct_cipher(password);

    let mut query = connection.prepare("SELECT a, b FROM logins").unwrap();
    let query_iter = query.query_map(NO_PARAMS, |row| {
        let service = row.get(0).unwrap();
        let service = decrypt(&cipher, service);

        let username = row.get(1).unwrap();
        let username = decrypt(&cipher, username);

        crate::print_green(service, format!("username: {}", username));
        Ok(())
    }).unwrap();

    for row in query_iter {
        row.unwrap();
    }
    println!();
}

pub fn get(password: &String) {
    println!();
    let service = crate::input("Search for what service? ");
    let connection = Connection::open(".data.db").unwrap();

    let cipher = construct_cipher(password);

    let service = encrypt(&cipher, service);

    let sql = format!("SELECT c FROM logins WHERE a ='{}'", service);

    let mut query = connection.prepare(sql.as_str()).unwrap();
    let query_iter = query.query_map(NO_PARAMS, |row| {
        let pw = row.get(0).unwrap();
        let pw = decrypt(&cipher, pw);

        crate::save_to_clipboard(pw);

        crate::print_green("Password copied to clipbard!", "");
        Ok(())
    }).unwrap();

    let mut results = false;
    for row in query_iter {
        row.unwrap();
        results = true;
    }
    if !results {
        crate::print_red("Unable to find a saved login for that service.", "");
    }
    println!();
}

pub fn delete(password: &String) {
    println!();

    let service = crate::input("Delete what service? ");
    let connection = Connection::open(".data.db").unwrap();

    let cipher = construct_cipher(password);

    let service = encrypt(&cipher, service);

    let sql = format!("DELETE FROM logins WHERE a ='{}'", service);
    let num_affected = connection.execute(sql.as_str(), NO_PARAMS).unwrap_or(0);

    if num_affected == 0 {
        crate::print_red("Unable to find a saved login for that service.", "");
    }
    else {
        crate::print_green("Deleted login!", "");
    }

    println!();
}

pub fn edit(password: &String) {
    println!();

    let service = crate::input("Update what service? ");
    let username = crate::input("new username: ");
    let pw = crate::input_password("new password: ");

    let connection = Connection::open(".data.db").unwrap();

    let cipher = construct_cipher(password);

    let service = encrypt(&cipher, service);
    let username = encrypt(&cipher, username);
    let pw = encrypt(&cipher, pw);

    let sql = format!("UPDATE logins SET b = '{}', c = '{}' WHERE a ='{}'", username, pw, service);
    let num_affected = connection.execute(sql.as_str(), NO_PARAMS).unwrap_or(0);

    if num_affected == 0 {
        crate::print_red("Unable to find a saved login for that service.", "");
    }
    else {
        crate::print_green("Updated login!", "");
    }

    println!();
}

pub fn new_login(password: &String) {
    println!();

    let service = crate::input("Create a login for what service? ");
    let username = crate::input("username: ");
    let pw = crate::input_password("password: ");

    let cipher = construct_cipher(password);

    let service = encrypt(&cipher, service);
    let username = encrypt(&cipher, username);
    let pw = encrypt(&cipher, pw);

    let sql = format!("INSERT INTO logins VALUES ({:?}, {:?}, {:?});", service, username, pw);

    let connection = Connection::open(".data.db").unwrap();
    match connection.execute(sql.as_str(), NO_PARAMS) {
        Ok(_) => crate::print_green("Login saved!", ""),
        Err(_) => crate::print_red("Unable to save login.", "The service name must be unique")
    }
    
    println!();
}

/// Create a database and its requisite table on first sign-in.
pub fn create() {
    let connection = Connection::open(".data.db").unwrap();
    connection.execute("CREATE TABLE logins (a TEXT UNIQUE, b TEXT, c TEXT);", NO_PARAMS).expect("Unable to access db");
}

fn encrypt(cipher: &ChaCha20Poly1305, input: String) -> String {
    let nonce = Nonce::from_slice(NONCE.as_bytes());
    let input = cipher.encrypt(nonce, input.as_bytes().as_ref())
        .expect("Encryption failure");
    let input = convert_vec_to_str(input);

    input
}

fn decrypt(cipher: &ChaCha20Poly1305, input: String) -> String {
    let nonce = Nonce::from_slice(NONCE.as_bytes());
    let input = convert_str_to_vec(input);
    let input = cipher.decrypt(nonce, input.as_ref())
        .expect("Decryption failure");
    let input = String::from_utf8(input).unwrap();

    input
}

/// Formats a Vector to be saved in the database
fn convert_vec_to_str(vec: Vec<u8>) -> String {
    let mut string = format!("{:?}", vec);
    string = string[1..string.len() - 1].to_string();
    string
}   

/// Converts the String version of the String in the database to a usable Vector
fn convert_str_to_vec(string: String) -> Vec<u8> {
    let strs: Vec<&str> = string.split(", ").collect();
    let mut utf8: Vec<u8> = vec!();

    for s in strs {
        utf8.push(s.parse().unwrap());
    }

    utf8
}

fn construct_cipher(key: &String) -> ChaCha20Poly1305 {
    // we want to generate a 32 byte key to encrypt the saved data
    // the plaintext password for ironhide could be much more
    // or much less. Therefore, we iterate over the string
    // and stop once we get 32 bytes, repeating as necessary
    // We can't use the hashed version as that is freely
    // visible on the filesystem next to the database
    let mut key_plaintext = String::new();

    let mut i = 0;
    for _ in 0..32 {
        if i >= key.len() {
            i = 0;
        }

        key_plaintext.push(key.as_bytes()[i] as char);

        i += 1;
    }

    let key = Key::from_slice(&key_plaintext.as_bytes()); // 32 bytes
    let cipher = ChaCha20Poly1305::new(key);

    cipher
}