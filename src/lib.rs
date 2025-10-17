#[macro_use]
extern crate magic_crypt;
pub mod utils;
use utils::{
    pasm_methods,
    helper_fxns,
};
use std::io;
use colored::*;
//hol aits th e test info
pub fn run(conf: utils::types::Config) -> Result<(), String> {
    match conf.operation.as_str() {
        "init" => {
            if conf.file_arg.len() > 1 {
                let mut passkey: String = String::new();
                println!("{}", "Please type your locker password.** Note that its your last defence for the data, so choose wisely! **".magenta().bold());
                io::stdin()
                    .read_line(&mut passkey)
                    .expect(&"failed to read passkey!".red().bold().to_string());
                let passkey = passkey.trim();
                pasm_methods::init_pasm(conf.file_arg, passkey.to_string());
            } else {
                eprintln!("{}","please specify the file name !".red().bold());
            }
        }

        "write" => {
            if conf.file_arg.len() > 1 {
                let ispasskeycorrect = helper_fxns::verify(conf.file_arg.clone());
                match ispasskeycorrect {
                    Ok(passkey) => {
                        let details = helper_fxns::ui_to_details();
                        pasm_methods::write_pasm(details, conf.file_arg.to_lowercase(), passkey);
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
                let ispasskeycorrect = helper_fxns::verify(conf.file_arg.clone());
                match ispasskeycorrect {
                    Ok(passkey) => {
                        println!("{}", "Displaying Result >>> ".black().bold().on_green());
                        println!();
                        pasm_methods::display_pasm(conf.file_arg.to_lowercase(), passkey);
                    }
                    Err(e) => println!("{}{}","Error : ".red().bold(), e.red().bold()),
                }
            } else {
                eprintln!("{}","please specify the file name!".red().bold());
            }
        }
        "find" => {
            if conf.file_arg.len() > 1 {
                let ispasskeycorrect = helper_fxns::verify(conf.file_arg.clone());
                match ispasskeycorrect {
                    Ok(passkey) => {
                        let mut e_name = String::new();
                        println!("{}","Type the name of entry to find >>>".black().bold().on_green());
                        io::stdin()
                            .read_line(&mut e_name)
                            .expect(&"failed to read entry name!".red().bold().to_string());
                        println!();
                        pasm_methods::find_pasm(e_name, conf.file_arg.to_lowercase(), passkey);
                    }
                    Err(e) => println!("{}{}","Error : ".red().bold(), e.red().bold()),
                }
            } else {
                eprintln!("{}","please specify the file name!".red().bold());
            }
        }
        "delete" => {
            if conf.file_arg.len() > 1 {
                let ispasskeycorrect = helper_fxns::verify(conf.file_arg.clone());
                match ispasskeycorrect {
                    Ok(passkey) => {
                        println!("{}","Type the name of entry to delete >>>".black().bold().on_green());
                        let mut e_name = String::new();
                        io::stdin()
                            .read_line(&mut e_name)
                            .expect(&"failed to read entry name!".red().bold().to_string());
                        pasm_methods::delete_pasm(e_name, conf.file_arg, passkey); //conf.file_arg is name of entry
                    }
                    Err(e) => println!("{}{}","Error : ".red().bold(), e.red().bold()),
                }
            } else {
                eprintln!("{}","please specify the file name!".red().bold());
            }
        }

        "edit" => {//this edits th efile
            if conf.file_arg.len() > 1 {
                let ispasskeycorrect = helper_fxns::verify(conf.file_arg.clone());
                match ispasskeycorrect {
                    Ok(passkey) => {
                        println!("{}", "Type the name of entry to edit >>>".black().bold().on_green());
                        let mut e_name = String::new();
                        io::stdin()
                            .read_line(&mut e_name)
                            .expect(&"failed to read entry name!".red().bold().to_string());
                        pasm_methods::find_pasm(e_name.clone(), conf.file_arg.clone(), passkey.clone());
                        println!("\n");
                        println!("{}","Type the new Entries >>>".black().bold().on_green());

                        let new_content = helper_fxns::ui_to_vec();
                        pasm_methods::edit_pasm(e_name, &new_content, conf.file_arg, passkey);
                    }
                    Err(e) => println!("{}{}","Error : ".red().bold(), e.red().bold()),
                }
            } else {
                eprintln!("{}","please specify the file name!".red().bold());
            }
        }
        "help" => {
                if conf.file_arg.is_empty()&& (!conf.operation.is_empty()){
                    pasm_methods::help_pasm();
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

