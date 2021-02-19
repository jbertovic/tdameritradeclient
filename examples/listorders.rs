use std::env;
use tdameritradeclient::TDAClient;

fn main() {
    env_logger::init();

    let c = TDAClient::new(env::var("TDAUTHTOKEN").unwrap());

    let resptxt: serde_json::Value = c.get_user_principals();
    let accountid = resptxt["primaryAccountId"].as_str().unwrap();

    let resptxt: serde_json::Value = c.get_orders(&accountid, &[]); // get working orders
    prettyprint(&resptxt);
}

fn prettyprint(toprint: &serde_json::Value) {
    println!("{}\n", serde_json::to_string_pretty(toprint).unwrap());
}
