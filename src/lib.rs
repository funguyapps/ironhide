use std::io;
use std::io::prelude::*;
use std::fmt::Display;
use std::path::Path;
use std::process;
use console::{ style, Term };

extern crate rpassword;
use rpassword::read_password_from_tty;

extern crate clipboard;
use clipboard::{ ClipboardContext, ClipboardProvider };

mod password;
mod generator;
mod database;

pub fn signin() -> password::Password {
    match check_first_signin() {
        // it is the first sign-in, so prompt to create password
        true => {
            println!("Welcome to ironhide, the better password manager.");
            println!("It looks like this is your first time using ironhide, so let's get you up and running.");
            println!("Remember that your password for ironhide is your first line of defense against attackers, so choose something unique and strong.");
            println!("Enjoy ironhide!");
            let plaintext = input_password("What would you like your password to be? ");
            let password = password::Password::new(plaintext);
            password.save_to_file()
                .expect("Unable to save password to file.");

            database::create();

            password
        },
        // it is not the first sign-in, so validate the password
        false => {
            let password = password::Password::new_validated("sign in: ");

            password
        },
    }
}

pub fn welcome() {
    Term::stdout().clear_screen()
        .expect("Unable to clear screen");

    let ironhide = r"
 _                          _       _       _        
(_)  _ __    ___    _ __   | |__   (_)   __| |   ___ 
| | | '__|  / _ \  | '_ \  | '_ \  | |  / _` |  / _ \
| | | |    | (_) | | | | | | | | | | | | (_| | |  __/
|_| |_|     \___/  |_| |_| |_| |_| |_|  \__,_|  \___|
    ";
    print_red(ironhide, "");
    println!("The better password manager.");
    println!("");
}

pub fn show_options() {
    print_green("new-password (nwpw)", " - generate a new secure password");
    print_green("   new-login (nwlg)", " - create a new saved login");
    print_green("          list (ls)", " - list all saved login service names and usernames");
    print_green("          copy (cp)", " - copies the password for the specified service name to the clipboard");
    print_green("          edit (ed)", " - edit the username and password for the given service");
    print_green("        delete (rm)", " - delete the login for the given service");
    print_green("           help (h)", " - shows this help message");
    print_green("           quit (q)", " - exits ironhide");
    println!();
}

pub fn main_loop(password: &password::Password) {
    loop {
        let prompt = format!("{} > ", style("ironhide").red());
        let cmd = input(prompt.as_str());

        match cmd.as_str() {
            "quit" | "q" => {
                Term::stdout().clear_screen()
                    .expect("Unable to clear screen");        
                process::exit(0);
            },
            "help" | "h" | "?" => show_options(),
            "new-password" | "nwpw" => {
                generator::run();
            },
            "list" | "ls" => {
                database::list_all(&password.plaintext);
            },
            "copy" | "cp" => {
                database::get(&password.plaintext);
            },
            "edit" | "ed" => {
                database::edit(&password.plaintext);
            }
            "delete" | "del" | "rm" => {
                database::delete(&password.plaintext);
            }
            "new-login" | "nwlg" => {
                database::new_login(&password.plaintext);
            }
            _ => {
                println!("Unknown command. Try help to see the possible options.");
            }
        }
    }
}

// * Helper Functions 
pub fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush()
        .expect("Unable to flush stream.");
    let mut in_str = String::new();

    io::stdin()
        .read_line(&mut in_str)
        .expect("Failed to read line");

    in_str = in_str.trim_end().to_string();

    if in_str.is_empty() {
        return input(prompt);
    }

    in_str
}

pub fn print_green<T: Display, N: Display>(green_arg: T, regular_arg: N) {
    println!("{} {}", style(green_arg).green(), regular_arg);
}

pub fn print_red<T: Display, N: Display>(red_arg: T, regular_arg: N) {
    println!("{} {}", style(red_arg).red(), regular_arg);
}

pub fn save_to_clipboard(data: String) {
    let mut context: ClipboardContext = ClipboardProvider::new().unwrap();
    context.set_contents(data.to_owned()).unwrap();
}

/// Returns false if the credentials file exists (it is NOT the first sign-in)
/// 
/// Returns true otherwise (it IS the first sign-in)
pub fn check_first_signin() -> bool {
    // if the file exists, it is not the first sign-in
    // this means the results of Path.exists() must be
    // inverted to see if it is the first sign in
    !Path::new(".usr.txt").exists()
}

pub fn input_password(prompt: &str) -> String {
    // this uses the external crate rpassword's implementation
    read_password_from_tty(Some(prompt))
        .expect("Unable to read user input")
}