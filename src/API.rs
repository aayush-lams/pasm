use magic_crypt::MagicCryptTrait;
use magic_crypt::new_magic_crypt;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json;
use std::fs;
use std::io;
use dirs;
use std::io::Write;
use std::path::Path;


fn strvec_to_detailsvec(vec: Vec<String>) -> Vec<Details> {
    let mut details_container = Vec::new();
   for i in vec {
       let json_data:Details = serde_json::from_str(&i).expect("error converting to string");
       details_container.push(json_data);
   }
    return details_container;
}

pub fn write(details: Details){
    let home_dir = dirs::home_dir().expect("Failed to get home directory");
    let file_name = "pasmuser0.txt";
    let filepath = home_dir.join(".config").join("pasm").join(file_name);
    let parent_dir = filepath.parent().expect("failed to get parent directory");
    fs::create_dir_all(parent_dir).expect("failed to create directory");

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
    println!("Wrote to a file sucessfully!");
}

pub fn find(name: String)->Vec<String> {
    let home_dir = dirs::home_dir().expect("Failed to get home directory");
    let file_name = "pasmuser0.txt";
    let filepath = home_dir.join(".config").join("pasm").join(file_name);

    // let name = "\"".to_owned()+&name + "\"";
    let name = name.trim();
    let file = fs::read_to_string(&filepath).expect("failed to read file!");
    let mut entry_vec = Vec::new();
    for line in file.lines() {
        // println!("{line}");
        let mcrypt = new_magic_crypt!("magickey", 256);
        let binding = mcrypt.decrypt_base64_to_string(line).unwrap();
        let decrypt_text = binding.as_str();
        if decrypt_text.to_lowercase().contains(&name.to_lowercase()) {
            // let lineStruct :Details= serde_json::from_str(decrypt_text).expect("Failed to convert to Struct!");
            entry_vec.push(decrypt_text.to_string());
        }

        // print!("{}", line);
    }
    return entry_vec;
}

pub fn display()->Vec<String>{
    let home_dir = dirs::home_dir().expect("Failed to get home directory");
    let file_name = "pasmuser0.txt";
    let filepath = home_dir.join(".config").join("pasm").join(file_name);

    let file = fs::read_to_string(&filepath).expect("failed to read file!");
    let mut entry_vec = Vec::new();

    for line in file.lines() {
        // println!("{line}");
        let mcrypt = new_magic_crypt!("magickey", 256);
        let binding = mcrypt.decrypt_base64_to_string(line).unwrap();
        let decrypt_text = binding.as_str();
        // let lineStruct: Details = serde_json::from_str(decrypt_text).expect("Failed to convert to Struct!");
            entry_vec.push(decrypt_text.to_string());
    }
    // print!("{}", line);
    return entry_vec;
}

pub fn delete(name: String){
    let home_dir = dirs::home_dir().expect("Failed to get home directory");
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



pub fn edit(name: String, newContent: &Vec<String> ) {
    let home_dir = dirs::home_dir().expect("Failed to get home directory");
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


