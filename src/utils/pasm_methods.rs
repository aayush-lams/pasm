use std::env;
use std::path::Path;
use colored::*;
use sled::Db;
use crate::utils::{
    types::Details,
    helper_fxns::{
        encrypt_string,
        save_to_db,
        decrypt_string
    },
};

//core funciton to run
pub fn init_pasm(file_name: String, passkey: String) {
    let home_dir = env::var("HOME").expect(&"failed to get home directory".red().bold().to_string());
    let file_name = file_name.as_str();
    let filepath = Path::new(&home_dir)
        .join(".config")
        .join("pasm")
        .join(file_name.to_lowercase());
    let db: Db = sled::open(filepath).expect(&"failed to open db!".red().bold().to_string());
    db.insert("files".as_bytes(), file_name.as_bytes())
        .expect(&"failed to insert keep track of new file".red().bold().to_string());
    db.insert("passkey".as_bytes(), passkey.as_bytes())
        .expect(&"failed to insert the passkey!".red().bold().to_string());
    println!("{}","database point set sucessfully!".green().bold());
}

pub fn write_pasm(details: Details, file_name: String, passkey: String) {
    let detail_str = serde_json::to_string(&details).expect(&"error converting to string".red().bold().to_string());
    let crypt_text = encrypt_string(detail_str, passkey.clone());
    save_to_db("credentials".to_string(), crypt_text.to_string(), file_name);
    println!("{}", "written to a file sucessfully!".green().bold());
}

pub fn find_pasm(name: String, file_name: String, passkey: String) {
    let name = name.trim();
    let home_dir = env::var("HOME").expect(&"failed to get home directory".red().bold().to_string());
    let filepath = Path::new(&home_dir)
        .join(".config")
        .join("pasm")
        .join(file_name.to_lowercase());
    let db: Db = sled::open(filepath).expect(&"failed to open database!".red().bold().to_string());
    println!("{}","Displaying Result >>".black().bold().on_green());
    println!();
    if let Some(creds) = db
        .get("credentials".to_string().as_bytes())
        .expect(&"error getting value from database!".red().bold().to_string())
    {
        let retrieved_value =
            String::from_utf8(creds.to_vec()).expect(&"error converting utf8 to String!".red().bold().to_string());
        for i in retrieved_value.split_whitespace() {
            let decrypt_text = decrypt_string(i.to_string(), passkey.clone());
            let line_struct: Details =
                serde_json::from_str(&decrypt_text).expect(&"Failed to convert to Struct!".red().bold().to_string());
            if line_struct.name == name {
                println!("{}{}","name : ".blue(), line_struct.name.bright_green());
            println!("{}{}","site : ".blue(), line_struct.site.bright_green());
            println!("{}{}","uname : ".blue(), line_struct.uname.bright_green());
            println!("{}{}","pword : ".blue(), line_struct.pword.bright_green());
            println!("{}{}","note : ".blue(), line_struct.note.bright_green());
                println!("\n");
            }
        }
    }

    // print!("{}", line);
}

pub fn delete_pasm(name: String, file_name: String, passkey: String) {
    let name = name.trim();
    let home_dir = env::var("HOME").expect(&"failed to get home directory".red().bold().to_string());
    let filepath = Path::new(&home_dir)
        .join(".config")
        .join("pasm")
        .join(file_name.to_lowercase());
    let db: Db = sled::open(filepath).expect(&"failed to open database!".red().bold().to_string());
    if let Some(creds) = db
        .get("credentials".to_string().as_bytes())
        .expect(&"error getting value from database!".red().bold().to_string())
    {
        let retrieved_value =
            String::from_utf8(creds.to_vec()).expect(&"error converting utf8 to String!".red().bold().to_string());
        let mut new_data = String::new();
        for i in retrieved_value.split_whitespace() {
            let decrypt_text = decrypt_string(i.to_string(), passkey.clone());
            let line_struct: Details =
                serde_json::from_str(&decrypt_text).expect(&"Failed to convert to Struct!".red().bold().to_string());
            if !(line_struct.name == name) {
                new_data.push(' ');
                new_data.push_str(i);
            }
        }
        db.insert("credentials".to_string(), new_data.as_bytes())
            .expect(&"failed to write in database!".red().bold().to_string());
    }
    println!("{}", "Deleted entry succesfully".green().bold());
}

pub fn display_pasm(file_name: String, passkey: String) {
    let home_dir = env::var("HOME").expect(&"failed to get home directory".red().bold().to_string());
    let filepath = Path::new(&home_dir)
        .join(".config")
        .join("pasm")
        .join(file_name.to_lowercase());
    let db: Db = sled::open(filepath).expect(&"failed to open database!".red().bold().to_string());
    if let Some(creds) = db
        .get("credentials".as_bytes())
        .expect(&"error getting value from database!".red().bold().to_string())
    {
        let retrieved_value =
            String::from_utf8(creds.to_vec()).expect(&"error converting utf8 to String!".red().bold().to_string());
        for i in retrieved_value.split_whitespace() {
            let decrypt_text = decrypt_string(i.to_string(), passkey.clone());
            let line_struct: Details =
                serde_json::from_str(&decrypt_text).expect(&"Failed to convert to Struct!".red().bold().to_string());
            println!("{}{}","name : ".blue(), line_struct.name.bright_green());
            println!("{}{}","site : ".blue(), line_struct.site.bright_green());
            println!("{}{}","uname : ".blue(), line_struct.uname.bright_green());
            println!("{}{}","pword : ".blue(), line_struct.pword.bright_green());
            println!("{}{}","note : ".blue(), line_struct.note.bright_green());
            println!("\n");
        }
    }
    // print!("{}", line);
}

pub fn edit_pasm(
    file_arg: String,
    new_content: &Vec<String>,
    file_name: String,
    passkey: String,
) {
    let name = file_name.trim();
    let home_dir = env::var("HOME").expect(&"failed to get home directory".red().bold().to_string());
    let filepath = Path::new(&home_dir)
        .join(".config")
        .join("pasm")
        .join(name.to_lowercase());
    let file_arg = file_arg.as_str().trim().to_string();
    let db: Db = sled::open(filepath).expect(&"failed to open database!".red().bold().to_string());
    if let Some(creds) = db
        .get("credentials".to_string().as_bytes())
        .expect(&"error getting value from database!".red().bold().to_string())
    {
        let retrieved_value =
            String::from_utf8(creds.to_vec()).expect(&"error converting utf8 to String!".red().bold().to_string());
        let mut new_data = String::new();
        for i in retrieved_value.split_whitespace() {
            let decrypt_text = decrypt_string(i.to_string(), passkey.clone());
            let mut line_struct: Details =
                serde_json::from_str(&decrypt_text).expect(&"Failed to convert to Struct!".red().bold().to_string());
            if line_struct.name == file_arg {
                if new_content[0] != "" {
                    line_struct.name = new_content[0].to_string();
                }
                if new_content[1] != "" {
                    line_struct.site = new_content[1].to_string();
                }
                if new_content[2] != "" {
                    line_struct.uname = new_content[2].to_string();
                }
                if new_content[3] != "" {
                    line_struct.pword = new_content[3].to_string();
                }
                if new_content[4] != "" {
                    line_struct.note = new_content[4].to_string();
                }
                let linestr =
                    serde_json::to_string(&line_struct).expect(&"failed to convert to string!".red().bold().to_string());
                let binding: String = encrypt_string(linestr, passkey.clone());
                new_data.push(' ');
                new_data.push_str(&binding);
            } else {
                new_data.push(' ');
                new_data.push_str(i);
            }
        }
        db.insert("credentials".as_bytes(), new_data.as_bytes())
            .expect(&"failed to write in database!".red().bold().to_string());
    }
}

pub fn help_pasm() {
    println!("{}","Usage: pasm [operation] [name of entry]\n".purple().bold());
    println!("{}","Operation:");
    println!("{}","\tinit : initiates new database for the user : takes [name]".purple().bold());
    println!("{}","\tdelete : Delete particular entry from file : takes [name]".purple().bold());
    println!("{}","\tdisplay : Display all entries : takes [name]".purple().bold());
    println!("{}","\tedit : Edit particular entries : takes [name]".purple().bold());
    println!("{}","\tfind : Find and display particular entries : takes [name]".purple().bold());
    println!("{}","\thelp : Display help : takes".purple().bold());
    println!("{}","\twrite : Write new entry, creates new if file doesnot exist \n".purple().bold());
    println!("{}","\tjust type the operation and interact with the database \n".purple().bold());
}
