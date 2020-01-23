extern crate passwords;
use passwords::{hasher, analyzer, scorer, PasswordGenerator};
use std::convert::TryInto;

pub fn generate(length: u32, save: bool) -> Result<String, &'static str> {
    let pg = PasswordGenerator {
        length: length.try_into().unwrap(),
        numbers: true,
        lowercase_letters: true,
        uppercase_letters: true,
        symbols: true,
        strict: true,
    };
    let pw = pg.generate_one()?;

    // TODO: save the password if -s flag is true
    // if save {
    //     let s_h = salt_and_hash(String::from(&pw)).unwrap();
    //     println!("salt: {:?}\nhashed: {:?}", s_h.0, s_h.1);
    // }

    Ok(pw)
}

pub fn check(password: String) -> Result<f64, &'static str> {
    let score = scorer::score(&analyzer::analyze(password));

    Ok(score)
}

// fn salt_and_hash(pw: String) -> Result<([u8; 16], [u8; 24]), &'static str> {
//     let salt = hasher::gen_salt();
//     let hashed = hasher::bcrypt(10, &salt, &pw).unwrap();
//     Ok((salt, hashed))
// }

// fn unhash(pw: &str, salt: [u8; 16], hashed: [u8; 24]) {
//     let pw = hasher::identify_bcrypt(10, &salt, &pw, &hashed);
//     println!("{:?}", pw)
// }