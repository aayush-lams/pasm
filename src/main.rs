use pasm::utils::types::Config;
use std::env;
use std::process;
use colored::*;
fn main() {
    //std::process::Command::new("clear").status().expect("error clearning screen");
    let args: Vec<String> = env::args().collect();
    if args.len()==2 || args.len()==3{
    let query = Config::new(&args).unwrap_or_else(|some_err| {
        eprintln!("problem parsing arguments: {some_err}");
        process::exit(1);
    });
    if let Err(err) = pasm::run(query) {
        eprintln!("application error!: {err}");// erro ri snic eini t?
        process::exit(1);
    }}
    else{
        eprintln!("{}","Syntax error! type help for more info.".red().bold());
    }
}
