#[macro_use]
extern crate magic_crypt;
use magic_crypt::MagicCryptTrait;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::path::Path;

//core funciton to run

pub fn run(conf: Config) -> Result<(), String> {
    match conf.operation.as_str() {
        "init" => {
            if conf.entry_name.len() > 1 {
                let current_directory =
                    env::current_dir().expect("error getting current directory!");
                let file_path = current_directory.join(conf.entry_name+".txt");
                print!(
                    "{}",
                    file_path
                        .to_str()
                        .expect("error converting to &str")
                        .to_string()
                );
                init_pasm(file_path);
            } else {
                eprintln!("please specify the file name !");
            }
        }

        "write" => {
            if conf.entry_name.len()>1 {
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

                write(details); //here is fname is filename
            } else {
                eprintln!("please specify the file name!");
            }
        }
        "display" => {
            if conf.entry_name.len()>1 {
                println!("Displaying Result : \n");
                display();
            } else {
                eprintln!("please specify the file name !");
            }
        }
        "find" => {
            if conf.entry_name.len()>1 {
                find(conf.entry_name); 
            } 
            else{
                eprintln!("please specify the file name !");
            }

            //onf.entry_name is name of entry
        }
        "delete" => {
            if conf.entry_name.len() > 1 {
                delete(conf.entry_name); //conf.entry_name is name of entry
            } else {
                eprintln!("please specify the file name !");
            }
        }

        "edit" => {
            if conf.entry_name.len()>1{

            find(conf.entry_name.clone());
            println!("\n");
            println!("Type the new Entries : \n");
            let mut new_content: Vec<String> = Vec::new();
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
            new_content.push(name.to_string());

            println!("\nnew sitename >>");
            io::stdin()
                .read_line(&mut site)
                .expect("error reading sitename!");
            let site = site.trim();
            new_content.push(site.to_string());

            println!("\nnew user name >>");
            io::stdin()
                .read_line(&mut uname)
                .expect("error reading username!");
            let uname: &str = uname.trim();
            new_content.push(uname.to_string());

            println!("\nnew password >>");
            io::stdin()
                .read_line(&mut pword)
                .expect("error reading password!");
            let pword = pword.trim();
            new_content.push(pword.to_string());

            println!("\nwrite new note: >>");
            io::stdin()
                .read_line(&mut note)
                .expect("error reading note!");
            let note: &str = note.trim();
            new_content.push(note.to_string());

            edit(conf.entry_name, &new_content);
            }
            else{
                eprintln!("please specify the file name !");
            }
        }
        "help" => {
            if !conf.entry_name.is_empty(){
                help();
            }
            else{
                eprintln!("help args doesnt require extra args!");
            }
        }
        _=>{eprint!("syntax error !");}
    }
    Ok(())
}

//basic prerequisite fxns

pub fn encrypt_string<'a>(data:String)->String{
    let mcrypt = new_magic_crypt!("magickey", 256);
    let binding = mcrypt.encrypt_str_to_base64(&data);
    binding
}


//utility fxns
pub fn save_filename(filename : &str){
    
}

pub fn init_pasm(current_directory: PathBuf) {
    save_filename(current_directory.to_str().expect("error converting path to &str"));
    fs::File::create_new(current_directory).expect("failed to create new file !");
}

pub fn write(details: Details) {
    let home_dir = env::home_dir().expect("Failed to get home directory");
    let file_name = "pasmuser0.txt";
    let filepath = home_dir.join(".config").join("pasm").join(file_name);
    let parent_dir = filepath.parent().expect("failed to get parent directory");
    fs::create_dir_all(parent_dir).expect("failed to create directory");

    let detail_str = serde_json::to_string(&details).expect("error converting to string");
    let crypt_text = encrypt_string(detail_str);
    // let mcrypt = new_magic_crypt!("magickey", 256);
    // let binding = mcrypt.encrypt_str_to_base64(detail_str);
    // let crypt_text = binding.as_str();
    let mut file = fs::File::options()
        .append(true)
        .create(true)
        .open(&filepath)
        .expect("error opening file in append mode!");
    writeln!(file, "{}", crypt_text.as_str()).expect("error writing to file!");
    println!("Wrote to a file at '~/.config/pasm' sucessfully!");
}

pub fn find(name: String) {
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
            let line_struct: Details =
                serde_json::from_str(decrypt_text).expect("Failed to convert to Struct!");
            println!("name : {}", line_struct.name);
            println!("site : {}", line_struct.site);
            println!("uname : {}", line_struct.uname);
            println!("pword : {}", line_struct.pword);
            println!("note : {}", line_struct.note);
            println!("\n");
        }
        // print!("{}", line);
    }
}

pub fn delete(name: String) {
    let home_dir = env::home_dir().expect("Failed to get home directory");
    // let file_name = "pasmuser0.txt".to_string();
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

pub fn display() {
    let home_dir = env::home_dir().expect("Failed to get home directory");
    let file_name = "pasmuser0.txt";
    let filepath = home_dir.join(".config").join("pasm").join(file_name);

    let file = fs::read_to_string(&filepath).expect("failed to read file!");
    for line in file.lines() {
        // println!("{line}");
        let mcrypt = new_magic_crypt!("magickey", 256);
        let binding = mcrypt.decrypt_base64_to_string(line).unwrap();
        let decrypt_text = binding.as_str();
        let line_struct: Details =
            serde_json::from_str(decrypt_text).expect("Failed to convert to Struct!");
        println!("name : {}", line_struct.name);
        println!("site : {}", line_struct.site);
        println!("uname : {}", line_struct.uname);
        println!("pword : {}", line_struct.pword);
        println!("note : {}", line_struct.note);
        println!("\n");
    }
    // print!("{}", line);
}

pub fn edit(name: String, new_content: &Vec<String>) {
    let home_dir = env::home_dir().expect("Failed to get home directory");
    // let file_name = "pasmuser0.txt".to_string();
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
            let mut line_struct: Details =
                serde_json::from_str(decrypt_text).expect("failed to convert to struct!");
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
                serde_json::to_string(&line_struct).expect("failed to convert to string!");
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

//edit verification

pub fn verify() -> Result<(), String> {
    let mut result = String::new();
    println!("are you sure you want to make changes to the file ? (y/n)");
    result = io::stdin()
        .read_line(&mut result)
        .expect("error reading result!")
        .to_string();
    if result.trim().to_lowercase() == "y" || result.trim().to_lowercase() == "yes" {
        Ok(())
    } else {
        Err("no changes made !".to_string())
    }
}

//args passed to the pasm command

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
        let entry_name = if args.get(2).is_some() {
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

//user detail
#[derive(Serialize, Deserialize)]
pub struct Details {
    pub name: String,
    pub site: String,
    pub uname: String,
    pub pword: String,
    pub note: String,
}


//file list
