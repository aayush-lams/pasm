use crate::client::auth::master;
use crate::client::curl::requests;
use crate::client::entry;

/// All CLI subcommands supported by pasm_client.
///
/// Each variant maps to a command string from CLI args. Variants that require
/// additional data carry associated fields (e.g., `Find { name }`).
///
/// Commands that manage credentials (`Create`, `Find`, `List`, `Delete`,
/// `Amend`) require an active session (api_key + encr_key). `Login` and
/// `Logout` work without a session.
pub enum CliCommand {
    /// Log in with master password — creates or verifies password, sets up session.
    Login,
    /// Log out — clears the local session file.
    Logout,
    /// Create a new entry via interactive prompts (encrypts before upload).
    Create,
    /// Find an entry by name and display decrypted details.
    Find { name: String },
    /// List all entries with their decrypted details.
    List,
    /// Delete an entry by name.
    Delete { name: String },
    /// Amend (create or overwrite) an entry via interactive prompts.
    Amend,
    /// Register the current api_key with the server (auto-done on first login).
    Register,
    /// Replace the current api_key with a new one (server-side key rotation).
    /// Takes the new key as a CLI argument.
    UpdateAuth { new_key: String },
    /// Remove the current user and all their data from the server.
    RemoveAuth,
    /// List all registered users (auth keys) on the server.
    ListUsers,
    /// Show this help message.
    Help,
}

impl CliCommand {
    /// Parses CLI arguments into the corresponding `CliCommand`.
    ///
    /// # Arguments
    /// * `args` - Argument strings excluding the binary name.
    ///   E.g., `["find", "github"]` for `Find { name: "github" }`.
    ///
    /// # Returns
    /// `Ok(CliCommand)` if the first argument matches a known command and
    /// any required positional arguments are present. Returns `Err(String)`
    /// with a description on failure.
    pub fn from_args(args: &[String]) -> Result<Self, String> {
        let cmd = args.first().ok_or("missing command")?;
        match cmd.as_str() {
            "login" => Ok(CliCommand::Login),
            "logout" => Ok(CliCommand::Logout),
            "create" => Ok(CliCommand::Create),
            "find" => {
                let name = args.get(1).ok_or("find requires a name")?;
                Ok(CliCommand::Find { name: name.clone() })
            }
            "list" => Ok(CliCommand::List),
            "delete" => {
                let name = args.get(1).ok_or("delete requires a name")?;
                Ok(CliCommand::Delete { name: name.clone() })
            }
            "amend" => Ok(CliCommand::Amend),
            "register" => Ok(CliCommand::Register),
            "update-auth" => {
                let new_key = args.get(1).ok_or("update-auth requires a new key")?;
                Ok(CliCommand::UpdateAuth {
                    new_key: new_key.clone(),
                })
            }
            "remove-auth" => Ok(CliCommand::RemoveAuth),
            "list-users" => Ok(CliCommand::ListUsers),
            "help" | "-h" | "--help" => Ok(CliCommand::Help),
            _ => Err(format!("unknown command: {cmd}")),
        }
    }

    /// Executes the command, returning a human-readable result string.
    ///
    /// `Login` and `Logout` ignore the key arguments (typically passed as
    /// empty strings). All other commands use `api_key` for API
    /// authentication and `encr_key` for entry-level encryption/decryption.
    ///
    /// # Arguments
    /// * `api_key` - Bearer token for server API calls
    /// * `encr_key` - AES-256 key for entry encrypt/decrypt operations
    ///
    /// # Returns
    /// A displayable result message. May be an error message on failure.
    pub fn execute(&self, api_key: &str, encr_key: &str) -> String {
        match self {
            CliCommand::Login => master::login(),
            CliCommand::Logout => master::logout(),
            CliCommand::Create => entry::ops::create(api_key, encr_key),
            CliCommand::Find { name } => entry::ops::find(api_key, encr_key, name),
            CliCommand::List => entry::ops::list(api_key, encr_key),
            CliCommand::Delete { name } => entry::ops::delete(api_key, name),
            CliCommand::Amend => entry::ops::amend(api_key, encr_key),
            CliCommand::Register => requests::register_auth(api_key),
            CliCommand::UpdateAuth { new_key } => requests::update_auth(api_key, new_key),
            CliCommand::RemoveAuth => requests::remove_auth(api_key),
            CliCommand::ListUsers => requests::list_users(api_key),
            CliCommand::Help => Self::usage(),
        }
    }

    /// Returns the help text listing all available commands.
    pub fn usage() -> String {
        concat!(
            "Usage: pasm_client <command> [args]\n",
            "       pasm_client help\n",
            "       pasm_client -h | --help\n",
            "\n",
            "Session management:\n",
            "  login              Login with master password\n",
            "  logout             Log out (clear session)\n",
            "\n",
            "Entry management (requires login):\n",
            "  create             Create an entry (interactive)\n",
            "  find <name>        Find and display an entry\n",
            "  list               List all entries\n",
            "  delete <name>      Delete an entry\n",
            "  amend              Amend an entry (interactive, creates if missing)\n",
            "\n",
            "Account management (requires login):\n",
            "  register           Register current auth key with server\n",
            "  update-auth <key>  Replace auth key (key rotation)\n",
            "  remove-auth        Remove user and all data from server\n",
            "  list-users         List all registered users\n",
        )
        .to_string()
    }
}
