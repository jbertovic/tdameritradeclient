use std::env;
use tdameritradeclient::{TDAClient, Endpoint, param};
use tdameritradeclient::model::userprincipals::UserPrincipals;
use tdameritradeclient::model::account::AccountRoot;

fn main() {
    env_logger::init();

    // grab authorization token from an environmental variable
    let c = TDAClient::new(env::var("TDAUTHTOKEN").unwrap());

    // get userprincipals endpoint and parse into userprincipals type
    title_print("UserPrincipals Model:");

    let userprincipals: UserPrincipals = serde_json::from_value(c.get(&Endpoint::UserPrincipals, &[param::Empty])).unwrap();
    println!("{:?}\n",&userprincipals);

    // pull out primary account id
    let accountid = &userprincipals.primary_account_id;

    // get account details on positions
    title_print("Account Position:");
    let account_root: AccountRoot = serde_json::from_value(c.get(&Endpoint::Account(accountid), &[param::Account::Positions])).unwrap();

    // pull out positions
    let positions = account_root.securities_account.positions;

    // iterate through positions
    for p in positions {
        println!("symbol: {}, quantity: {}, value: {}", p.instrument.symbol, p.long_quantity-p.short_quantity, p.market_value);
    }

}

fn title_print(heading: &str) {
    println!("{}", heading.to_uppercase());
    println!("{}", "-".repeat(heading.len()));
}