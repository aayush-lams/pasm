use serde::{Deserialize, Serialize};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;

pub fn run(conf: Config) -> Result<(), String> {
    match conf.operation.as_str() {
        "write" => {
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
            write(details, conf.filename);
        }
        "display" => {
            println!("Displaying Result : \n");
            display(conf.filename);
        }
        "find" => {
            let mut name = String::new();
            println!("\ntype the name to find >> ");
            io::stdin()
                .read_line(&mut name)
                .expect("failed to read line!");
            find(name, conf.filename);
        }
        "delete" => {
            let mut input = String::new();
            println!("name to delete >> ");
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read name!");
            delete(input, conf.filename);
        }
        "remove" => {
            if let Err(e) = fs::remove_file(conf.filename) {
                eprintln!("Failed to delete file: {}", e);
            } else {
                println!("File deleted successfully");
            }
        }
        "edit" => {
            let mut input = String::new();
            println!("type the name of entry to edit >> ");
            io::stdin()
                .read_line(&mut input)
                .expect("failed to input name of entry !");
            println!("\n");
            find(input.clone(), conf.filename.clone());
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

            edit(input.clone(), &newContent, conf.filename.clone());
        }
        "help" => {
            help(conf.filename);
        }
        _ => {
            println!("not a valid command !");
        }
    }
    Ok(())
}

pub fn write(details: Details, filename: String) {
    let detailStr = serde_json::to_string(&details).expect("error converting to string");
    let mut file = fs::File::options()
        .append(true)
        .create(true)
        .open(filename)
        .expect("error opening file in append mode!");
    writeln!(file, "{}", detailStr).expect("error writing to file!");
    println!("Wrote to a file sucessfully!");
}

pub fn find(name: String, filename: String) {
    // let name = "\"".to_owned()+&name + "\"";
    let name = name.trim();
    let file = fs::read_to_string(&filename).expect("failed to read file!");
    println!("Displaying Result : \n");
    for line in file.lines() {
        // println!("{line}");
        if line.to_lowercase().contains(&name.to_lowercase()) {
            let lineStruct: Details =
                serde_json::from_str(line).expect("Failed to convert to Struct!");
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

pub fn delete(name: String, filename: String) {
    let file = fs::read_to_string(&filename).expect("failed to read file");
    let mut newfile = fs::File::create("dummy.txt").expect("failed to create dummy.txt");
    let name = name.trim();
    for line in file.lines() {
        if !line.to_lowercase().contains(&name) {
            writeln!(newfile, "{}", line).expect("Failed to write to newfile");
        }
    }
    fs::rename("dummy.txt", filename).expect("Failed to rename file");
    println!("Deleted {} from file sucessfully !", name);
}

pub fn display(filename: String) {
    let file = fs::read_to_string(&filename).expect("failed to read file!");
    for line in file.lines() {
        // println!("{line}");
        let lineStruct: Details = serde_json::from_str(line).expect("Failed to convert to Struct!");
        println!("name : {}", lineStruct.name);
        println!("site : {}", lineStruct.site);
        println!("uname : {}", lineStruct.uname);
        println!("pword : {}", lineStruct.pword);
        println!("note : {}", lineStruct.note);
        println!("\n");
    }
    // print!("{}", line);
}

pub fn edit(name: String, newContent: &Vec<String>, filename: String) {
    let file = fs::read_to_string(&filename).expect("failed to read file");
    let mut newfile = fs::File::create("dummy.txt").expect("failed to create dummy.txt");
    let name = name.trim();
    for line in file.lines() {
        if line.to_lowercase().contains(&name) {
            let mut lineStruct: Details =
                serde_json::from_str(line).expect("failed to convert to struct!");
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
            writeln!(newfile, "{}", linestr).expect("Failed to write to newfile");
        } else {
            writeln!(newfile, "{}", line).expect("Failed to write to newfile");
        }
    }
    fs::rename("dummy.txt", filename).expect("Failed to rename file");
    println!("edited {} from file sucessfully !", name);
}

pub fn help(filename: String) {
    println!("Usage: pasm -- [operation] [option2]\n");
        println!("Operation:");
        println!("\tdelete : Delete particular entry from file");
        println!("\tdisplay : Display all entries");
        println!("\tedit : Edit particular entries");
        println!("\tfind : Find and display particular entries");
        println!("\thelp : Display help");
        println!("\tremove : Remove the file");
        println!("\twrite : Write new entry, creates new if file doesnot exist\n");
        println!("Option2:");
        println!("\tfilename: filename or complete path from root");
        println!("\t. : used with help to display all materials");

}
pub struct Config {
    pub operation: String,
    pub filename: String,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 3 {
            return Err("Not enough arguments!");
        }
        let operation = args[1].clone();
        let filename = args[2].clone();
        Ok(Config {
            operation,
            filename,
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
