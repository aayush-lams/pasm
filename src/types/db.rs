use axum::http::StatusCode;
use sled::Db;
use uuid::Uuid;

use crate::types::{entry::RequestData, error::PasmResult};

/// A wrapper around the `sled` embedded database.
///
/// Provides password entry management with user isolation and encrypted storage.
///
/// # Database Structure
/// The database is organized as follows:
///
/// - `users` tree: Maps authentication keys to user IDs
/// - `{user_id}` tree: Contains encrypted password entries for each user
///   - Key: `entry:{entry_name}`
///   - Value: Encrypted entry data
///
/// # Example
/// ```
/// let db = sled::open("path/to/db").unwrap();
/// let pasm_db = PasmDb::new(db);
/// ```
#[derive(Clone)]
pub struct PasmDb {
    db: Db,
}

impl PasmDb {
    /// Creates a new `PasmDb` wrapper around an existing `sled::Db` instance.
    ///
    /// # Arguments
    /// * `db` - An open sled database connection
    ///
    /// # Returns
    /// A new `PasmDb` instance wrapping the provided database
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    /// Opens the `users` tree from the database.
    ///
    /// The users tree stores the mapping between authentication keys and user IDs.
    ///
    /// # Returns
    /// * `Ok(sled::Tree)` - The users tree on success
    ///
    /// # Errors
    /// * `PasmResult::DatabaseError` - If the tree cannot be opened
    pub fn users(&self) -> Result<sled::Tree, PasmResult> {
        self.db
            .open_tree("users")
            .map_err(|e| PasmResult::DatabaseError { err: e })
    }

    /// Retrieves a user ID associated with the given authentication key.
    ///
    /// # Arguments
    /// * `authkey` - The authentication key (Bearer token)
    ///
    /// # Returns
    /// * `Ok(String)` - The user ID associated with the key
    ///
    /// # Errors
    /// * `PasmResult::ServerStatus(NOT_FOUND)` - If the auth key is not found
    /// * `PasmResult::UTF8ConversionError` - If the stored user ID is invalid UTF-8
    pub fn get_user_id_by_authkey(&self, authkey: &str) -> Result<String, PasmResult> {
        let users = self.users()?;
        let user_id = users
            .get(authkey.as_bytes())
            .map_err(|e| PasmResult::ServerStatus(StatusCode::NOT_FOUND, e.to_string()))?;

        let u_id = match user_id {
            Some(ivec) => String::from_utf8(ivec.to_vec()),
            None => {
                return Err(PasmResult::ServerStatus(
                    StatusCode::NOT_FOUND,
                    "user_id not found".to_string(),
                ))
            }
        };

        match u_id {
            Ok(res) => Ok(res),
            Err(e) => Err(PasmResult::UTF8ConversionError { err: e }),
        }
    }

    /// Associates an authentication key with a user ID in the database.
    ///
    /// # Arguments
    /// * `authkey` - The authentication key (Bearer token)
    /// * `user_id` - The user identifier to store
    ///
    /// # Returns
    /// * `Ok(())` - On successful insertion
    ///
    /// # Errors
    /// * `PasmResult::DatabaseError` - If the insertion fails
    pub fn set_user(&self, authkey: &str, user_id: &str) -> Result<(), PasmResult> {
        let tree = self.users()?;
        tree.insert(authkey.as_bytes(), user_id.as_bytes())
            .map_err(|e| PasmResult::DatabaseError { err: e })?;
        Ok(())
    }

    /// Opens the tree for a specific user, identified by their user ID.
    ///
    /// Each user has their own isolated tree for storing encrypted password entries.
    ///
    /// # Arguments
    /// * `user_id` - The unique identifier for the user
    ///
    /// # Returns
    /// * `Ok(sled::Tree)` - The user's entry tree
    ///
    /// # Errors
    /// * `PasmResult::DatabaseError` - If the tree cannot be opened
    pub fn user_tree(&self, user_id: &str) -> Result<sled::Tree, PasmResult> {
        self.db
            .open_tree(user_id)
            .map_err(|e| PasmResult::DatabaseError { err: e })
    }

    /// Adds a new encrypted password entry for a user.
    ///
    /// Uses compare-and-swap to ensure no existing entry with the same name is overwritten.
    ///
    /// # Arguments
    /// * `user_id` - The user to add the entry for
    /// * `entry_name` - A unique identifier for this entry (e.g., "github", "gmail")
    /// * `encrypted_data` - The AES-256 encrypted entry data
    ///
    /// # Returns
    /// * `Ok(())` - On successful creation
    ///
    /// # Errors
    /// * `PasmResult::ServerStatus(CONFLICT)` - If an entry with this name already exists
    /// * `PasmResult::DatabaseError` - If the database operation fails
    pub fn add_entry(
        &self,
        user_id: &str,
        entry_name: &str,
        encrypted_data: String,
    ) -> Result<(), PasmResult> {
        let tree = self.user_tree(user_id)?;
        let key = format!("entry:{}", entry_name);
        match tree.compare_and_swap(&key, None as Option<&[u8]>, Some(encrypted_data.as_bytes())) {
            Ok(Ok(_)) => Ok(()),
            Ok(Err(_)) => Err(PasmResult::ServerStatus(
                StatusCode::CONFLICT,
                "entry already exists".to_string(),
            )),
            Err(e) => Err(PasmResult::DatabaseError { err: e }),
        }
    }

    /// Removes a password entry for a user.
    ///
    /// # Arguments
    /// * `user_id` - The user whose entry to remove
    /// * `entry_name` - The name of the entry to delete
    ///
    /// # Returns
    /// * `Ok(())` - On successful deletion
    ///
    /// # Errors
    /// * `PasmResult::DatabaseError` - If the database operation fails
    pub fn remove_entry(&self, user_id: &str, entry_name: &str) -> Result<(), PasmResult> {
        let tree = self.user_tree(user_id)?;
        let key = format!("entry:{}", entry_name);
        tree.remove(key.as_bytes())
            .map_err(|e| PasmResult::DatabaseError { err: e })?;
        Ok(())
    }

    /// Retrieves an encrypted password entry for a user.
    ///
    /// # Arguments
    /// * `user_id` - The user whose entry to retrieve
    /// * `entry_name` - The name of the entry to fetch
    ///
    /// # Returns
    /// * `Ok(String)` - The encrypted entry data
    ///
    /// # Errors
    /// * `PasmResult::ServerStatus(NOT_FOUND)` - If the entry does not exist
    /// * `PasmResult::DatabaseError` - If the database operation fails
    /// * `PasmResult::UTF8ConversionError` - If the stored data is invalid UTF-8
    pub fn get_entry(&self, user_id: &str, entry_name: &str) -> Result<String, PasmResult> {
        let tree = self.user_tree(user_id)?;
        let key = format!("entry:{}", entry_name);
        let result = tree
            .get(key.as_bytes())
            .map_err(|e| PasmResult::DatabaseError { err: e })?;
        match result {
            Some(res) => {
                let entry = String::from_utf8(res.to_vec())
                    .map_err(|e| PasmResult::UTF8ConversionError { err: e })?;
                Ok(entry)
            }
            None => Err(PasmResult::ServerStatus(
                StatusCode::NOT_FOUND,
                entry_name.to_string(),
            )),
        }
    }

    /// Checks whether a specific entry exists for a user.
    ///
    /// # Arguments
    /// * `user_id` - The user to check
    /// * `entry_name` - The name of the entry to look for
    ///
    /// # Returns
    /// * `Ok(bool)` - `true` if the entry exists, `false` otherwise
    ///
    /// # Errors
    /// * `PasmResult::DatabaseError` - If the database operation fails
    pub fn has_entry(&self, user_id: &str, entry_name: &str) -> Result<bool, PasmResult> {
        let tree = self.user_tree(user_id)?;
        let key = format!("entry:{}", entry_name);
        tree.contains_key(key.as_bytes())
            .map_err(|e| PasmResult::DatabaseError { err: e })
    }

    /// Lists all password entries for a user.
    ///
    /// Returns a vector of all entries with their names and encrypted data.
    ///
    /// # Arguments
    /// * `user_id` - The user whose entries to list
    ///
    /// # Returns
    /// * `Ok(Vec<RequestData>)` - All entries belonging to the user
    ///
    /// # Errors
    /// * `PasmResult::DatabaseError` - If the database iteration fails
    /// * `PasmResult::UTF8ConversionError` - If entry keys or values are invalid UTF-8
    pub fn list_entries(&self, user_id: &str) -> Result<Vec<RequestData>, PasmResult> {
        let tree = self.user_tree(user_id)?;
        let mut entries: Vec<RequestData> = Vec::new();

        for item in tree.iter() {
            let (key, value) = item.map_err(|e| PasmResult::DatabaseError { err: e })?;
            let entry_name = String::from_utf8(key.to_vec())
                .map_err(|e| PasmResult::UTF8ConversionError { err: e })?;

            let entry = String::from_utf8(value.to_vec())
                .map_err(|err| PasmResult::UTF8ConversionError { err })?;

            let entry = RequestData {
                key: entry_name,
                value: entry,
            };
            entries.push(entry);
        }

        Ok(entries)
    }

    /// Updates or creates an entry for a user.
    ///
    /// Unlike `add_entry`, this method will overwrite existing entries.
    ///
    /// # Arguments
    /// * `user_id` - The user to update the entry for
    /// * `entry_name` - The name of the entry to amend
    /// * `encrypted_data` - The new AES-256 encrypted entry data
    ///
    /// # Returns
    /// * `Ok(())` - If an existing entry was updated
    /// * `Err(CREATED)` - If a new entry was created (returns status code 201)
    ///
    /// # Errors
    /// * `PasmResult::DatabaseError` - If the database operation fails
    pub fn amend_entry(
        &self,
        user_id: &str,
        entry_name: &str,
        encrypted_data: String,
    ) -> Result<(), PasmResult> {
        let tree = self.user_tree(user_id)?;
        match tree.insert(entry_name.as_bytes(), encrypted_data.as_bytes()) {
            Ok(Some(_)) => Err(PasmResult::ServerStatus(
                StatusCode::CREATED,
                "new entry created !".to_string(),
            )),
            Ok(None) => Ok(()),
            Err(err) => Err(PasmResult::DatabaseError { err }),
        }
    }

    /// Registers a new user with a generated UUID.
    ///
    /// Creates a new authentication key -> user ID mapping in the users tree.
    ///
    /// # Arguments
    /// * `auth_key` - The authentication key (Bearer token) to associate
    ///
    /// # Returns
    /// * `Ok(())` - On successful registration
    ///
    /// # Errors
    /// * `PasmResult::ServerStatus(CONFLICT)` - If the auth key already exists
    /// * `PasmResult::DatabaseError` - If the database operation fails
    pub fn register_auth(&self, auth_key: &str) -> PasmResult {
        let users = match self.users() {
            Ok(tree) => tree,
            Err(err) => return err,
        };

        let user_id = Uuid::new_v4().to_string();

        match users.compare_and_swap(
            auth_key.as_bytes(),
            None as Option<&[u8]>,
            Some(user_id.as_bytes()),
        ) {
            Ok(Ok(_)) => PasmResult::ServerStatus(StatusCode::OK, "".to_string()),
            Ok(Err(_)) => PasmResult::ServerStatus(
                StatusCode::CONFLICT,
                "auth key already exists".to_string(),
            ),
            Err(err) => PasmResult::DatabaseError { err },
        }
    }

    /// Updates an existing authentication key to a new one.
    ///
    /// Atomically replaces the old auth key with the new one while preserving the user ID.
    ///
    /// # Arguments
    /// * `auth_key` - The current authentication key
    /// * `new_auth` - The new authentication key to set
    ///
    /// # Returns
    /// * `Ok(())` - On successful update
    ///
    /// # Errors
    /// * `PasmResult::ServerStatus(NOT_FOUND)` - If the old auth key does not exist
    /// * `PasmResult::ServerStatus(CONFLICT)` - If the new auth key already exists
    /// * `PasmResult::DatabaseError` - If the database operation fails
    pub fn update_auth(&self, auth_key: &str, new_auth: &str) -> PasmResult {
        let users = match self.users() {
            Ok(tree) => tree,
            Err(err) => return err,
        };

        let user_id = match self.get_user_id_by_authkey(auth_key) {
            Ok(id) => id,
            Err(err) => return err,
        };

        match users.compare_and_swap(
            new_auth.as_bytes(),
            None as Option<&[u8]>,
            Some(user_id.as_bytes()),
        ) {
            Ok(Ok(_)) => {}
            Ok(Err(_)) => {
                return PasmResult::ServerStatus(
                    StatusCode::CONFLICT,
                    "new auth key already exists".to_string(),
                )
            }
            Err(err) => return PasmResult::DatabaseError { err },
        }

        match users.compare_and_swap(
            auth_key.as_bytes(),
            Some(user_id.as_bytes()),
            None as Option<&[u8]>,
        ) {
            Ok(Ok(_)) => PasmResult::ServerStatus(StatusCode::OK, "".to_string()),
            Ok(Err(_)) => PasmResult::ServerStatus(
                StatusCode::NOT_FOUND,
                "old auth key does not exist".to_string(),
            ),
            Err(err) => PasmResult::DatabaseError { err },
        }
    }

    /// Removes a user and all their data from the database.
    ///
    /// Deletes the auth key mapping and the user's entire entry tree.
    ///
    /// # Arguments
    /// * `auth_key` - The authentication key of the user to remove
    ///
    /// # Returns
    /// * `Ok(())` - On successful deletion
    ///
    /// # Errors
    /// * `PasmResult::ServerStatus(NOT_FOUND)` - If the auth key does not exist
    /// * `PasmResult::DatabaseError` - If the database operation fails
    pub fn remove_user(&self, auth_key: &str) -> PasmResult {
        let user_id = match self.get_user_id_by_authkey(auth_key) {
            Ok(id) => id,
            Err(err) => return err,
        };

        let users = match self.users() {
            Ok(tree) => tree,
            Err(err) => return err,
        };

        match users.remove(auth_key.as_bytes()) {
            Ok(Some(_)) => {}
            Ok(None) => {
                return PasmResult::ServerStatus(
                    StatusCode::NOT_FOUND,
                    "auth key not found".to_string(),
                )
            }
            Err(err) => return PasmResult::DatabaseError { err },
        }

        match self.db.drop_tree(&user_id) {
            Ok(_) => PasmResult::ServerStatus(StatusCode::OK, "".to_string()),
            Err(err) => PasmResult::DatabaseError { err },
        }
    }
}
