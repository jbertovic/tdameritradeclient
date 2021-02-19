use std::env;
use tdameritradeclient::{Account, TDAClient};

fn main() {
    env_logger::init();

    let c = TDAClient::new(env::var("TDAUTHTOKEN").unwrap());

    title_print("user/account info:");
    let resptxt: serde_json::Value = c.get_user_principals();
    pretty_print(&resptxt);

    let accountid = resptxt["primaryAccountId"].as_str().unwrap();

    title_print("position info:");
    pretty_print(&c.get_account(accountid, &[Account::Positions]));

    title_print("orders:");
    pretty_print(&c.get_orders(accountid, &[]));
}

fn pretty_print(toprint: &serde_json::Value) {
    println!("{}\n", serde_json::to_string_pretty(toprint).unwrap());
}

fn title_print(heading: &str) {
    println!("{}", heading.to_uppercase());
    println!("{}", "-".repeat(heading.len()));
}
