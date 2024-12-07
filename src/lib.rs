#[macro_use]
extern crate magic_crypt;
use magic_crypt::MagicCryptTrait;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use sled::Db;
use std::env;
use std::io;
use std::path::Path;
use colored::*;
//core funciton to run

pub fn run(conf: Config) -> Result<(), String> {
    match conf.operation.as_str() {
        "init" => {
            if conf.file_arg.len() > 1 {
                let mut passkey: String = String::new();
                println!("{}", "Please type your locker password.** Note that its you last defence for the data, so choose wisely! **".magenta().bold().on_blue());
                io::stdin()
                    .read_line(&mut passkey)
                    .expect(&"failed to read passkey!".red().bold().to_string());
                let passkey = passkey.trim();
                init_pasm(conf.file_arg, passkey.to_string());
            } else {
                eprintln!("{}","please specify the file name !".red().bold());
            }
        }

        "write" => {
            if conf.file_arg.len() > 1 {
                let ispasskeycorrect = verify(conf.file_arg.clone());
                match ispasskeycorrect {
                    Ok(passkey) => {
                        let details = ui_to_details();
                        write_pasm(details, conf.file_arg.to_lowercase(), passkey);
                        //here is fname is filename
                    }
                    Err(e) => println!("{}{}","Error : ".red().bold(), e.red().bold()),
                }
            } else {
                eprintln!("{}","please specify the file name!".red().bold());
            }
        }
        "display" => {
            if conf.file_arg.len() > 1 {
                let ispasskeycorrect = verify(conf.file_arg.clone());
                match ispasskeycorrect {
                    Ok(passkey) => {
                        println!("{}", "Displaying Result >>> ".black().bold().on_green());
                        println!();
                        display_pasm(conf.file_arg.to_lowercase(), passkey);
                    }
                    Err(e) => println!("{}{}","Error : ".red().bold(), e.red().bold()),
                }
            } else {
                eprintln!("{}","please specify the file name!".red().bold());
            }
        }
        "find" => {
            if conf.file_arg.len() > 1 {
                let ispasskeycorrect = verify(conf.file_arg.clone());
                match ispasskeycorrect {
                    Ok(passkey) => {
                        let mut e_name = String::new();
                        println!("{}","Type the name of entry to find >>>".black().bold().on_green());
                        io::stdin()
                            .read_line(&mut e_name)
                            .expect(&"failed to read entry name!".red().bold().to_string());
                        println!();
                        find_pasm(e_name, conf.file_arg.to_lowercase(), passkey);
                    }
                    Err(e) => println!("{}{}","Error : ".red().bold(), e.red().bold()),
                }
            } else {
                eprintln!("{}","please specify the file name!".red().bold());
            }
        }
        "delete" => {
            if conf.file_arg.len() > 1 {
                let ispasskeycorrect = verify(conf.file_arg.clone());
                match ispasskeycorrect {
                    Ok(passkey) => {
                        println!("{}","Type the name of entry to delete >>>".black().bold().on_green());
                        let mut e_name = String::new();
                        io::stdin()
                            .read_line(&mut e_name)
                            .expect(&"failed to read entry name!".red().bold().to_string());
                        delete_pasm(e_name, conf.file_arg, passkey); //conf.file_arg is name of entry
                    }
                    Err(e) => println!("{}{}","Error : ".red().bold(), e.red().bold()),
                }
            } else {
                eprintln!("{}","please specify the file name!".red().bold());
            }
        }

        "edit" => {
            if conf.file_arg.len() > 1 {
                let ispasskeycorrect = verify(conf.file_arg.clone());
                match ispasskeycorrect {
                    Ok(passkey) => {
                        println!("{}", "Type the name of entry to edit >>>".black().bold().on_green());
                        let mut e_name = String::new();
                        io::stdin()
                            .read_line(&mut e_name)
                            .expect(&"failed to read entry name!".red().bold().to_string());
                        find_pasm(e_name.clone(), conf.file_arg.clone(), passkey.clone());
                        println!("\n");
                        println!("{}","Type the new Entries >>>".black().bold().on_green());

                        let new_content = ui_to_vec();
                        edit_pasm(e_name, &new_content, conf.file_arg, passkey);
                    }
                    Err(e) => println!("{}{}","Error : ".red().bold(), e.red().bold()),
                }
            } else {
                eprintln!("{}","please specify the file name!".red().bold());
            }
        }
        "help" => {
                if conf.file_arg.is_empty()&& (!conf.operation.is_empty()){
                    help_pasm();
                }
                else{
                    eprintln!("{}","Help operation doesnot take any parameters !".red().bold());
                }
        }
        _ => {
            eprint!("{}","syntax error !".red().bold());
        }
    }
    Ok(())
}

//#utility fxns

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


pub fn descrypt_string<'a>(data: String, password: String) -> String {
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


pub fn ask_passkey<F1, F2>(ask_ui: F1, working_fn: F2)
where
    F1: Fn(),
    F2: Fn(),
{
    ask_ui();
    working_fn();
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
    new_content.push(note.to_string());
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

//#working fxns

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
            let decrypt_text = descrypt_string(i.to_string(), passkey.clone());
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
            let decrypt_text = descrypt_string(i.to_string(), passkey.clone());
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
            let decrypt_text = descrypt_string(i.to_string(), passkey.clone());
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
            let decrypt_text = descrypt_string(i.to_string(), passkey.clone());
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
    println!("{}","\tdelete : Delete particular entry from file : takes [name]".purple().bold());
    println!("{}","\tdisplay : Display all entries".purple().bold());
    println!("{}","\tedit : Edit particular entries : takes [name]".purple().bold());
    println!("{}","\tfind : Find and display particular entries : takes [name]".purple().bold());
    println!("{}","\thelp : Display help : takes".purple().bold());
    println!("{}","\twrite : Write new entry, creates new if file doesnot exist \n".purple().bold());
}

//args passed to the pasm command

pub struct Config {
    pub operation: String,
    pub file_arg: String,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        if args.len() > 3 && args.len() < 2 {
            return Err("Not enough arguments!".red().bold().to_string());
        }
        let operation = args[1].clone();
        let file_arg = if args.get(2).is_some() {
            args[2].clone()
        } else {
            "".to_string()
        };

        Ok(Config {
            operation,
            file_arg,
        })
    }
}

//#user detail
#[derive(Serialize, Deserialize)]
pub struct Details {
    pub name: String,
    pub site: String,
    pub uname: String,
    pub pword: String,
    pub note: String,
}

//file list
