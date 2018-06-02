extern crate reqwest;

extern crate serde_derive;
extern crate toml;

use std::fs::File;
use std::io::prelude::*;

use std::collections::HashMap;

#[derive(Deserialize)]
struct TAccount {
    name: String,
    login: String,
    password: String,
}

#[derive(Deserialize)]
struct TAccounts {
    account: Vec<TAccount>,
}

pub struct SmsService {
    info: HashMap<String, TAccount>,
}

impl SmsService {
    pub fn new(filepath_accounts: &str) -> Self {
        let mut accounts_file = match File::open(filepath_accounts) {
            Ok(f) => f,
            Err(e) => panic!(
                "Error occurred opening file: {} - Err: {}",
                filepath_accounts, e
            ),
        };

        let mut accounts_str = String::new();
        match accounts_file.read_to_string(&mut accounts_str) {
            Ok(s) => s,
            Err(e) => panic!("Error Reading file: {}", e),
        };
        println!("Zone File: {}", filepath_accounts);

        let tml: TAccounts = toml::from_str(&accounts_str).unwrap();
        let mut sms_obj = SmsService {
            info: HashMap::new(),
        };
        for x in tml.account {
            sms_obj.info.insert(x.name.to_string(), x);
        }
        sms_obj
    }

    pub fn sms_user(&self, user: &str, msg: &str) -> Result<String, String> {
        let user_info = self.info.get(user);
        match user_info {
            Some(user_info_unwrapped) => SmsService::sms_api(
                &user_info_unwrapped.login,
                &user_info_unwrapped.password,
                msg,
            ),
            None => Err("No information on user".to_string()),
        }
    }

    pub fn sms_api(login: &str, password: &str, msg: &str) -> Result<String, String> {
        let params = [("user", login), ("pass", password), ("msg", msg)];
        let client = reqwest::Client::new();
        let resp = client
            .get("https://smsapi.free-mobile.fr/sendmsg")
            .query(&params)
            .send();

        match resp {
            Ok(resp_unwrapped) => if resp_unwrapped.status().is_success() {
                Ok("Message sent".to_string())
            } else {
                Err("Sms error".to_string())
            },
            Err(_) => Err("Invalid answer".to_string()),
        }
    }
}
