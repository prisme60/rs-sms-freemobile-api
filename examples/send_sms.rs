use sms_freemobile_api::sms_service::SmsService;

fn main() {
    let sms = SmsService::new("Accounts.toml");
    let _ = sms.sms_user("cf", "Hello\nWorld!");
}
