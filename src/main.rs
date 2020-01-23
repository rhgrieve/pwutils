mod utils;

extern crate clipboard;
use clipboard::{ClipboardProvider, ClipboardContext};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "pwutils", about = "a collection of password utilities")]
enum PW {
    Gen {
        /// set password length
        #[structopt(short, long, default_value = "8")]
        length: u32,
        /// automatically copy generated password to clipboard
        #[structopt(short, long)]
        copy: bool,
        /// hashes and saves the password to .txt file
        #[structopt(short, long)]
        save: bool,
    },
    Check {
        password: String,
    }
}

fn main() {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    match PW::from_args() {
        PW::Gen { length, copy, save } => {
            let pw = utils::generate(length, save).unwrap();
            if copy {
                match ctx.set_contents(pw.to_owned()) {
                    Ok(_) => println!("Password copied to clipboard: {}", pw),
                    Err(_) => println!("Could not copy password to clipboard"),
                }
            } else {
                println!("Password generated: {}", pw);
            }

        },
        PW::Check { password } => {
            let score = utils::check(password).unwrap();
            let strength = match score.round() as u64 {
                0..=19 => "very dangerous",
                20..=39 => "dangerous",
                40..=59 => "very weak",
                60..=79 => "weak",
                80..=89 => "good",
                90..=94 => "strong",
                95..=98 => "very strong",
                99..=100 => "invulnerable",
                _ => "weird",
            };
            println!("password strength: {}", strength)
        }
    }
}
