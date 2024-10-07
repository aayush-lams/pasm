use pasm::Config;
use std::env;
use std::process;
fn main() {
    //std::process::Command::new("clear").status().expect("error clearning screen");
    let args: Vec<String> = env::args().collect();
    let query = Config::new(&args).unwrap_or_else(|someErr| {
        eprintln!("problem parsing arguments: {someErr}");
        process::exit(1);
    });
    if let Err(err) = pasm::run(query) {
        eprintln!("application error!: {err}");
        process::exit(1);
    }
}
