pub mod ui;

use std::process::Command;

use crate::types::entry::RequestData;

pub struct ApiClient {
    api_key: String,
    pub result: String,
}

impl ApiClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            result: String::new(),
        }
    }

    pub fn create(&mut self, key: &str, value: &str) {
        let payload = serde_json::json!({ "key": key, "value": value }).to_string();
        self.result = run_curl(&[
            "-s",
            "-X",
            "POST",
            "http://localhost:3000/entry",
            "-H",
            &format!("Authorization: Bearer {}", self.api_key),
            "-H",
            "Content-Type: application/json",
            "-d",
            &payload,
        ]);
    }

    pub fn find(&mut self, name: &str) {
        self.result = run_curl(&[
            "-s",
            "-X",
            "GET",
            &format!("http://localhost:3000/entry/{}", name),
            "-H",
            &format!("Authorization: Bearer {}", self.api_key),
        ]);
    }

    pub fn list(&mut self) {
        let res = run_curl(&[
            "-s",
            "-X",
            "GET",
            "http://localhost:3000/entries",
            "-H",
            &format!("Authorization: Bearer {}", self.api_key),
        ]);

        self.result = res;
        // let items: Vec<RequestData> = serde_json::from_str(&res).unwrap();

        // let formatted = items
        //     .into_iter()
        //     .map(|i| format!("{}: {}", i.key, i.value))
        //     .collect::<Vec<String>>()
        //     .join("\n");
        // self.result = formatted;
    }

    pub fn delete(&mut self, name: &str) {
        self.result = run_curl(&[
            "-s",
            "-X",
            "DELETE",
            &format!("http://localhost:3000/entry/{}", name),
            "-H",
            &format!("Authorization: Bearer {}", self.api_key),
        ]);
    }

    pub fn amend(&mut self, key: &str, value: &str) {
        let payload = serde_json::json!({ "key": key, "value": value }).to_string();
        self.result = run_curl(&[
            "-s",
            "-X",
            "POST",
            "http://localhost:3000/entry/amend",
            "-H",
            &format!("Authorization: Bearer {}", self.api_key),
            "-H",
            "Content-Type: application/json",
            "-d",
            &payload,
        ]);
    }

    pub fn update_auth(&mut self) {
        self.result = format!("updating auth : {}", self.api_key);
    }

    pub fn remove_auth(&mut self) {
        self.result = format!("removing auth : {}", self.api_key);
    }

    pub fn register_auth(&mut self) {
        self.result = run_curl(&[
            "-s",
            "-X",
            "POST",
            "http://localhost:3000/auth",
            "-H",
            &format!("Authorization: Bearer {}", self.api_key),
        ]);
    }

    pub fn list_users(&mut self) {
        let res = run_curl(&[
            "-s",
            "-X",
            "GET",
            "http://localhost:3000/auth/list",
            "-H",
            &format!("Authorization: Bearer {}", self.api_key),
        ]);
        let items: Vec<String> = serde_json::from_str(&res).unwrap();

        let formatted = items.join("\n");
        self.result = formatted;
    }
}

fn run_curl(args: &[&str]) -> String {
    let output = Command::new("curl").args(args).output();
    match output {
        Ok(o) => {
            String::from_utf8_lossy(&o.stdout).to_string()
        }
        Err(e) => format!("Error: {}", e),
    }
}
