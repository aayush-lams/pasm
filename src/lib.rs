#[macro_use]
extern crate magic_crypt;
use magic_crypt::MagicCryptTrait;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::fs;
use std::io;
use std::env;
use std::io::Write;
use std::path::Path;

pub fn run(conf: Config) -> Result<(), String> {
    match conf.operation.as_str() {
        "write" => {
            if conf.entry_name.is_empty(){
                let mut name = String::new();
                let mut site = String::new();
                let mut uname = String::new();
                let mut pword = String::new();
                let mut note = String::new();

                println!("\nname of entry >>");
                io::stdin()
                    .read_line(&mut name)
                    .expect("error reading name!");

                println!("\nsitename >>");
                io::stdin()
                    .read_line(&mut site)
                    .expect("error reading sitename!");

                println!("\nuser name >>");
                io::stdin()
                    .read_line(&mut uname)
                    .expect("error reading username!");

                println!("\npassword >>");
                io::stdin()
                    .read_line(&mut pword)
                    .expect("error reading password!");

                println!("\nwrite short note: >>");
                io::stdin()
                    .read_line(&mut note)
                    .expect("error reading note!");
                let details = Details {
                    name: name.trim().to_string(),
                    site: site.trim().to_string(),
                    uname: uname.trim().to_string(),
                    pword: pword.trim().to_string(),
                    note: note.trim().to_string(),
                };

                // println!("{}{}{}{}{}",details[0], details[1], details[2], details[3], details[4])
                write(details); //here is fname is filename
            } else {
                eprintln!("syntax error!");
            }
        }
        "display" => {
            if conf.entry_name.is_empty() {
                println!("Displaying Result : \n");
                display();
            } else {
                eprintln!("syntax error!");
            }
        }
        "find" => {
            find(conf.entry_name);//onf.entry_name is name of entry
        }
        "delete" => {
            if conf.entry_name.len()>1 {
            delete(conf.entry_name);//conf.entry_name is name of entry
                }
            else {
                eprintln!("syntax error!");
            }
        }

        "edit" => {
            find(conf.entry_name.clone());
            println!("\n");
            println!("Type the new Entries : \n");
            let mut newContent: Vec<String> = Vec::new();
            let mut name = String::new();
            let mut site = String::new();
            let mut uname = String::new();
            let mut pword = String::new();
            let mut note = String::new();

            println!("\nnew name of entry >>");
            io::stdin()
                .read_line(&mut name)
                .expect("error reading name!");
            let name = name.trim();
            newContent.push(name.to_string());

            println!("\nnew sitename >>");
            io::stdin()
                .read_line(&mut site)
                .expect("error reading sitename!");
            let site = site.trim();
            newContent.push(site.to_string());

            println!("\nnew user name >>");
            io::stdin()
                .read_line(&mut uname)
                .expect("error reading username!");
            let uname: &str = uname.trim();
            newContent.push(uname.to_string());

            println!("\nnew password >>");
            io::stdin()
                .read_line(&mut pword)
                .expect("error reading password!");
            let pword = pword.trim();
            newContent.push(pword.to_string());

            println!("\nwrite new note: >>");
            io::stdin()
                .read_line(&mut note)
                .expect("error reading note!");
            let note: &str = note.trim();
            newContent.push(note.to_string());

            edit(conf.entry_name, &newContent);
        }
        "help" => {
            help();
        }
        _ => {
            println!("not a valid command !");
        }
    }
    Ok(())
}

pub fn write(details: Details){
    let home_dir = env::home_dir().expect("Failed to get home directory");
    let file_name = "pasmuser0.txt";
    let filepath = home_dir.join(".config").join("pasm").join(file_name);
    let parent_dir = filepath.parent().expect("failed to get parent directory");
    //fs::create_dir_all(parent_dir).expect("failed to create directory");

    let detailStr = serde_json::to_string(&details).expect("error converting to string");
    let mcrypt = new_magic_crypt!("magickey", 256);
    let binding = mcrypt.encrypt_str_to_base64(detailStr);
    let crypt_text = binding.as_str();
    let mut file = fs::File::options()
        .append(true)
        .create(true)
        .open(&filepath)
        .expect("error opening file in append mode!");
    writeln!(file, "{}", crypt_text).expect("error writing to file!");
    println!("Wrote to a file at '~/.config/pasm' sucessfully!");
}

pub fn find(name: String){
    let home_dir = env::home_dir().expect("Failed to get home directory");
    let file_name = "pasmuser0.txt";
    let filepath = home_dir.join(".config").join("pasm").join(file_name);

    // let name = "\"".to_owned()+&name + "\"";
    let name = name.trim();
    let file = fs::read_to_string(&filepath).expect("failed to read file!");
    println!("Displaying Result : \n");
    for line in file.lines() {
        // println!("{line}");
        let mcrypt = new_magic_crypt!("magickey", 256);
        let binding = mcrypt.decrypt_base64_to_string(line).unwrap();
        let decrypt_text = binding.as_str();
        if decrypt_text.to_lowercase().contains(&name.to_lowercase()) {
            let lineStruct: Details =
                serde_json::from_str(decrypt_text).expect("Failed to convert to Struct!");
            println!("name : {}", lineStruct.name);
            println!("site : {}", lineStruct.site);
            println!("uname : {}", lineStruct.uname);
            println!("pword : {}", lineStruct.pword);
            println!("note : {}", lineStruct.note);
            println!("\n");
        }
        // print!("{}", line);
    }
}

pub fn delete(name: String){
    let home_dir = env::home_dir().expect("Failed to get home directory");
    let file_name = "pasmuser0.txt".to_string();
    let filepath = home_dir.join(".config").join("pasm").join("pasmuser0.txt");

    let file = fs::read_to_string(&filepath).expect("failed to read file");
    let nfile = home_dir.join(".config").join("dummy.txt");
    let mut newfile = fs::File::create(&nfile).expect("failed to create dummy.txt");
    let name = name.trim();
    for line in file.lines() {
        let mcrypt = new_magic_crypt!("magickey", 256);
        let binding = mcrypt.decrypt_base64_to_string(line).unwrap();
        let decrypt_text = binding.as_str();

        if !decrypt_text.to_lowercase().contains(&name) {
            writeln!(newfile, "{}", line).expect("Failed to write to newfile");
        }
    }
    fs::rename(nfile, filepath).expect("Failed to rename file");
    println!("Deleted {} from file sucessfully !", name);
}

pub fn display(){
    let home_dir = env::home_dir().expect("Failed to get home directory");
    let file_name = "pasmuser0.txt";
    let filepath = home_dir.join(".config").join("pasm").join(file_name);



    let file = fs::read_to_string(&filepath).expect("failed to read file!");
    for line in file.lines() {
        // println!("{line}");
        let mcrypt = new_magic_crypt!("magickey", 256);
        let binding = mcrypt.decrypt_base64_to_string(line).unwrap();
        let decrypt_text = binding.as_str();
        let lineStruct: Details =
            serde_json::from_str(decrypt_text).expect("Failed to convert to Struct!");
        println!("name : {}", lineStruct.name);
        println!("site : {}", lineStruct.site);
        println!("uname : {}", lineStruct.uname);
        println!("pword : {}", lineStruct.pword);
        println!("note : {}", lineStruct.note);
        println!("\n");
    }
    // print!("{}", line);
}

pub fn edit(name: String, newContent: &Vec<String> ) {
    let home_dir = env::home_dir().expect("Failed to get home directory");
let file_name = "pasmuser0.txt".to_string();
let filepath = home_dir.join(".config").join("pasm").join("pasmuser0.txt");



    let file = fs::read_to_string(&filepath).expect("failed to read file");
    let nfile = home_dir.join(".config").join("dummy.txt");
    let mut newfile = fs::File::create(&nfile).expect("failed to create dummy.txt");
    let name = name.trim();
    for line in file.lines() {
        let mcrypt = new_magic_crypt!("magickey", 256);
        let binding = mcrypt.decrypt_base64_to_string(line).unwrap();
        let decrypt_text = binding.as_str();
        if decrypt_text.to_lowercase().contains(&name) {
            let mut lineStruct: Details =
                serde_json::from_str(decrypt_text).expect("failed to convert to struct!");
            if newContent[0] != "" {
                lineStruct.name = newContent[0].to_string();
            }
            if newContent[1] != "" {
                lineStruct.site = newContent[1].to_string();
            }
            if newContent[2] != "" {
                lineStruct.uname = newContent[2].to_string();
            }
            if newContent[3] != "" {
                lineStruct.pword = newContent[3].to_string();
            }
            if newContent[4] != "" {
                lineStruct.note = newContent[4].to_string();
            }
            let linestr = serde_json::to_string(&lineStruct).expect("failed to convert to string!");
            let binding = mcrypt.encrypt_str_to_base64(linestr);
            let crypt_text = binding.as_str();
            writeln!(newfile, "{}", crypt_text).expect("Failed to write to newfile");
        } else {
            writeln!(newfile, "{}", line).expect("Failed to write to newfile");
        }
    }
    fs::rename(nfile, filepath).expect("Failed to rename file");
    println!("edited {} from file sucessfully !", name);
}

pub fn help() {
    println!("Usage: pasm [operation] [name of entry]\n");
    println!("Operation:");
    println!("\tdelete : Delete particular entry from file : takes [name]");
    println!("\tdisplay : Display all entries");
    println!("\tedit : Edit particular entries : takes [name]");
    println!("\tfind : Find and display particular entries : takes [name]");
    println!("\thelp : Display help : takes");
    println!("\twrite : Write new entry, creates new if file doesnot exist \n");
}


pub fn verify()->Result<(), String>{
    let mut result = String::new();
    println!("are you sure you want to make changes to the file ? (y/n)");
    result = io::stdin()
                .read_line(&mut result)
                .expect("error reading result!").to_string();
    if result.trim().to_lowercase() == "y" || result.trim().to_lowercase() == "yes" {
        Ok(())
    }
    else {
        Err("no changes made !".to_string())
    }
}
pub struct Config {
    pub operation: String,
    pub entry_name: String,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() > 3 && args.len() < 2 {
            return Err("Not enough arguments!");
        }
        let operation = args[1].clone();
        let entry_name = if args.get(2).is_some(){
            args[2].clone()
        } else {
            "".to_string()
        };

        Ok(Config {
            operation,
            entry_name,
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct Details {
    pub name: String,
    pub site: String,
    pub uname: String,
    pub pword: String,
    pub note: String,
}
