use crate::utils::types::Details;
use magic_crypt::MagicCryptTrait;
use std::io;
use colored::*;
use sled::Db;
use std::env;
use std::path::Path;

pub fn verify(database: String) -> Result<String, String> {
    let home_dir = env::var("HOME").expect(&"failed to get home directory".red().bold());
    let filepath = Path::new(&home_dir)
        .join(".config")
        .join("pasm")
        .join(database.to_lowercase());
    let db: Db = sled::open(filepath).expect(&"failed to open database!".red().bold().to_string());

    let mut passkey = String::new();
    println!("{}","type the passkey to open the database >>>".magenta().bold());
    io::stdin()
        .read_line(&mut passkey)
        .expect(&"error reading result!".red().bold().to_string())
        .to_string();
    if let Some(some_key) = db
        .get("passkey".as_bytes())
        .expect(&"failed to get key : passkey for file".red().bold().to_string())
    {
        let some_new_key =
            String::from_utf8(some_key.to_vec()).expect(&"failed to convert to String".red().bold().to_string());
        if passkey.trim() == some_new_key {
            Ok(passkey)
        } else {
            Err("verification failed!".red().bold().to_string())
        }
    } else {
        Err("filed to get the key for file".red().bold().to_string())
    }
}

pub fn decrypt_string<'a>(data: String, password: String) -> String {
    let mcrypt = new_magic_crypt!(password.as_str(), 256);
    let binding = mcrypt
        .decrypt_base64_to_string(&data)
        .expect(&"error converting base64 to String!".red().bold().to_string());
    binding
}

pub fn encrypt_string<'a>(data: String, password: String) -> String {
    let mcrypt = new_magic_crypt!(password.as_str(), 256);
    let binding = mcrypt.encrypt_str_to_base64(&data);
    binding
}

pub fn ui_to_details() -> Details {
    let mut name = String::new();
    let mut site = String::new();
    let mut uname = String::new();
    let mut pword = String::new();
    let mut note = String::new();

    println!("{}","\nname of entry : ".bright_green().bold());
    io::stdin()
        .read_line(&mut name)
        .expect(&"error reading name!".red().bold().to_string());

    println!("{}","\nsitename : ".bright_green().bold());
    io::stdin()
        .read_line(&mut site)
        .expect(&"error reading sitename!".red().bold().to_string());

    println!("{}","\nuser name : ".bright_green().bold());
    io::stdin()
        .read_line(&mut uname)
        .expect(&"error reading username!".red().bold().to_string());

    println!("{}","\npassword :".bright_green().bold());
    io::stdin()
        .read_line(&mut pword)
        .expect(&"error reading password!".red().bold().to_string());

    println!("{}","\nwrite short note :".bright_green().bold());
    io::stdin()
        .read_line(&mut note)
        .expect(&"error reading notes!".red().bold().to_string());
    let details = Details {
        name: name.trim().to_string(),
        site: site.trim().to_string(),
        uname: uname.trim().to_string(),
        pword: pword.trim().to_string(),
        note: note.trim().to_string(),
    };
    details
}

pub fn ui_to_vec() -> Vec<String> {
    let mut new_content: Vec<String> = Vec::new();
    let mut name = String::new();
    let mut site = String::new();
    let mut uname = String::new();
    let mut pword = String::new();
    let mut note = String::new();

    println!("{}","\nnew name of entry >>".black().bold().on_green());
    io::stdin()
        .read_line(&mut name)
        .expect(&"error reading name!".red().bold().to_string());
    let name = name.trim();
    new_content.push(name.to_string());

    println!("{}","\nnew sitename >>".black().bold().on_green());
    io::stdin()
        .read_line(&mut site)
        .expect(&"error reading sitename!".red().bold().to_string());
    let site = site.trim();
    new_content.push(site.to_string());

    println!("{}","\nnew user name >>".black().bold().on_green());
    io::stdin()
        .read_line(&mut uname)
        .expect(&"error reading username!".red().bold().to_string());
    let uname: &str = uname.trim();
    new_content.push(uname.to_string());

    println!("{}","\nnew password >>".black().bold().on_green());
    io::stdin()
        .read_line(&mut pword)
        .expect(&"error reading password!".red().bold().to_string());
    let pword = pword.trim();
    new_content.push(pword.to_string());

    println!("{}","\nnew note >>".black().bold().on_green());
    io::stdin()
        .read_line(&mut note)
        .expect(&"error reading notes!".red().bold().to_string());
    let note: &str = note.trim();
    new_content.push(note.to_string());//very lon gini t?
    new_content
}

pub fn save_to_db(key: String, value: String, database: String) {
    let home_dir = env::var("HOME").expect(&"failed to get home directory".red().bold().to_string());
    let filepath = Path::new(&home_dir)
        .join(".config")
        .join("pasm")
        .join(database.to_lowercase());
    let db: Db = sled::open(filepath).expect(&"failed to open database!".red().bold().to_string());
    if let Some(current_value) = db.get(key.as_bytes()).expect(&"failed to get value!".red().bold().to_string()) {
        let mut retrieved_data =
            String::from_utf8(current_value.to_vec()).expect(&"error converting utf8 to Strong".red().bold().to_string());
        retrieved_data.push(' ');
        retrieved_data.push_str(&value);
        db.insert(key.as_bytes(), retrieved_data.as_bytes())
            .expect(&"failed to insert details!".red().bold().to_string());
    } else {
        db.insert(key.as_bytes(), value.as_bytes())
            .expect(&"failed to insert details".red().bold().to_string());
    }
}

pub fn ask_passkey<F1, F2>(ask_ui: F1, working_fn: F2)
where
    F1: Fn(),
    F2: Fn(),
{
    ask_ui();
    working_fn();
}

