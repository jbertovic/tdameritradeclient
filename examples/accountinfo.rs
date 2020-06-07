use std::env;
use tdameritradeclient::{TDAClient, Account};

fn main() {

    let c = TDAClient::new(env::var("TDAUTHTOKEN").unwrap());

    titleprint("user/account info:");
    let resptxt: serde_json::Value = c.getuserprincipals();
    prettyprint(&resptxt);

    let accountid = resptxt["primaryAccountId"].as_str().unwrap();

    titleprint("position info:");
    prettyprint(&c.getaccount(accountid, &[Account::Positions]));

    titleprint("orders:");
    prettyprint(&c.getorders(accountid, &[]));
}

fn prettyprint(toprint: &serde_json::Value) {
    println!("{}\n", serde_json::to_string_pretty(toprint).unwrap());
}

fn titleprint(heading: &str) {
    println!("{}", heading.to_uppercase());
    println!("{}", "-".repeat(heading.len()));
}