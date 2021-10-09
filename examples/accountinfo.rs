use std::env;
use tdameritradeclient::{param, Endpoint, TDAClient};

fn main() {
    env_logger::init();

    // grab authorization token from an environmental variable
    let c = TDAClient::new(env::var("TDAUTHTOKEN").unwrap());

    // get userprincipals endpoint
    title_print("user/account info:");
    let resptxt: serde_json::Value = c.get(&Endpoint::UserPrincipals, &[param::Empty]);
    pretty_print(&resptxt);

    // pull out primary account id
    let accountid = resptxt["primaryAccountId"].as_str().unwrap();

    // get account details on positions
    title_print("position info:");
    pretty_print(&c.get(&Endpoint::Account(accountid), &[param::Account::Positions]));

    // get account details on any orders
    title_print("orders:");
    pretty_print(&c.get(&Endpoint::Orders(accountid), &[param::Empty]));
}

fn pretty_print(toprint: &serde_json::Value) {
    println!("{}\n", serde_json::to_string_pretty(toprint).unwrap());
}

fn title_print(heading: &str) {
    println!("{}", heading.to_uppercase());
    println!("{}", "-".repeat(heading.len()));
}
