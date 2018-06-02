extern crate sms_free_api;

use sms_free_api::sms_service::SmsService;

fn main() {
    let sms = SmsService::new("Accounts.toml");
    let _ = sms.sms_user("cf", "Hello\nWorld!");
}
