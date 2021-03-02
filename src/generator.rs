use rand::prelude::*;

const ALPHABET: &'static str = "abcdefghijlmnopqrstuvwxyz";
const NUMBERS: &'static str = "1234567890";
const SYMBOLS: &'static str = "!@#$%^&*-_";

struct PasswordOptions {
    length: usize,
    lowercase: bool,
    uppercase: bool,
    numbers: bool,
    symbols: bool
}

impl PasswordOptions {
    fn new() -> Self {
        let length = get_length();
        let lowercase = get_bool("Use lowercase?");
        let uppercase = get_bool("Use uppercase?");
        let numbers = get_bool("Use numbers?");
        let symbols = get_bool("Use symbols?");

        PasswordOptions { length, lowercase, uppercase, numbers, symbols }
    }
}

pub fn run() {
    println!();

    let options = PasswordOptions::new();

    let mut possible_chars = String::new();
    if options.lowercase { 
        possible_chars.push_str(ALPHABET); 
    }
    if options.uppercase { 
        possible_chars.push_str(ALPHABET.to_ascii_uppercase().as_str()); 
    }
    if options.numbers { 
        possible_chars.push_str(NUMBERS); 
    }
    if options.symbols { 
        possible_chars.push_str(SYMBOLS); 
    }

    // make sure there are at least some available characters
    if possible_chars.is_empty() {
        crate::print_red("You have to include one kind of possible character.", "");
        // have to restart the process of finding out options completely, so recall run
        run();
        // after that call to run completes, we want to exit this call immediately
        return;
    }

    let mut password = String::new();
    let mut rng = thread_rng();

    // for each char in specified length, get a random char from
    // possible chars and append it to the final password
    for _ in 0..options.length {
        let index: usize = rng.gen_range(0, possible_chars.len());
        password.push_str(&possible_chars[index..index+1]);
    }

    // save to clipboard
    crate::save_to_clipboard(password);

    crate::print_green("Saved password to clipboard!", "");
    println!();
}

fn get_length() -> usize{
    loop {
        let length = crate::input("How long would you like the password to be? (8-24) ");

        match length.parse::<usize>() {
            Ok(i) => {
                if i < 8 || i > 24 {
                    crate::print_red("Must be within the range 8-24", "");
                    continue;
                }
                return i
            },
            Err(_) => {
                crate::print_red("Must be a number.", "");
                continue;
            }
        }
    }
}

fn get_bool(prompt: &str) -> bool {
    loop {
        let prompt = format!("{} (yes/no) ", prompt);
        let yes_no = crate::input(prompt.as_str());

        match yes_no.to_ascii_lowercase().as_str() {
            "yes" | "y" => {
                return true;
            }
            "no" | "n" => {
                return false;
            }
            _ => {
                crate::print_red("Invalid input. Try yes or no.", "");
                continue;
            }
        }
    }
}