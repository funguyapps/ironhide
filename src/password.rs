use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::process;

extern crate bcrypt;
use bcrypt::{ DEFAULT_COST, hash, verify };

pub struct Password {
    pub plaintext: String,
    pub hashed: String,
    is_valid: bool
}

impl Password {
    pub fn new_validated(prompt: &str) -> Self {
        let plaintext = crate::input_password(prompt);

        let mut password = Password { plaintext, hashed: String::new(), is_valid: false };
        password.validate()
            .expect("Unable to validate password.");

        match password.is_valid {
            true => return password,
            false => {
                eprintln!("Incorrect password");
                process::exit(0)
            }
        }
    }

    /// Used to create a new password on first sign-in
    /// 
    /// Should not be used to validate existing sign-in as
    /// it assumes the plaintext password is always valid
    pub fn new(plaintext: String) -> Self {
        let hashed = hash(&plaintext, DEFAULT_COST).unwrap();
        Password { plaintext, hashed, is_valid: true }
    }

    fn validate(&mut self) -> Result<(), Box<dyn Error>> {
        let mut f = File::open(".usr.txt")?;
        let mut hashed = String::new();
        f.read_to_string(&mut hashed)?;

        // using the bcrypt crate's verify, compare the inputted password
        // to the saved one
        let valid = verify(&self.plaintext, hashed.as_str())?;
        self.is_valid = valid;
        self.hashed = hashed;
        Ok(())
    }

    pub fn save_to_file(&self) -> Result<usize, Box<dyn Error>> {
        let hashed = hash(&self.plaintext, DEFAULT_COST)?;
        let mut f = File::create(".usr.txt")?;

        // have to write to file as an array of bytes
        // this complicated line passes a reference to 
        // that array of bytes, created from the hashed
        // password of type String
        Ok(f.write(&hashed.as_str().as_bytes()[..])?)
    }
}