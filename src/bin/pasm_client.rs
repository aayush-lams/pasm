use pasm::client::auth::master;
use pasm::client::{cli::commands::CliCommand, response::display};

/// Entry point for the pasm CLI client.
///
/// Parses CLI arguments into a `CliCommand`, then either:
/// - Runs `Login`/`Logout` directly (no session needed)
/// - Checks for an active session, exits with an error if none found
/// - Executes the command with the session's api_key and encr_key
///
/// # Panics
/// Exits the process with code 1 on unknown commands or missing session.
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cmd_args: Vec<String> = args.into_iter().skip(1).collect();

    if cmd_args.is_empty() {
        print!("{}", CliCommand::usage());
        return;
    }

    let command = match CliCommand::from_args(&cmd_args) {
        Ok(cmd) => cmd,
        Err(e) => {
            display::print_error(&format!("Error: {e}"));
            eprint!("\n{}", CliCommand::usage());
            std::process::exit(1);
        }
    };

    if matches!(command, CliCommand::Login | CliCommand::Logout | CliCommand::Help) {
        let result = command.execute("", "");
        display::print_response(&result);
        return;
    }

    let (api_key, encr_key) = match master::get_session_keys() {
        Some((ak, ek)) => (ak, ek),
        None => {
            display::print_error("not logged in. run `pasm_client login` first");
            std::process::exit(1);
        }
    };

    let result = command.execute(&api_key, &encr_key);
    display::print_response(&result);
}
