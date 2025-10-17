use colored::*;
use serde_derive::Deserialize;
use serde_derive::Serialize;

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
