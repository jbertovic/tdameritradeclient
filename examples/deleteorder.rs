use std::env;
use tdameritradeclient::{error::TDAClientError, param, Endpoint, TDAClient};

fn main() -> Result<(), TDAClientError> {
    // Delete order by ORDERID

    env_logger::init();

    let c = TDAClient::new(env::var("TDAUTHTOKEN").unwrap());
    let orderid: String = env::args().nth(1).unwrap();

    println!("orderid to delete: {}", orderid);

    let resptxt: serde_json::Value = c.get(&Endpoint::UserPrincipals, &[param::Empty])?;
    let accountid = resptxt["primaryAccountId"].as_str().unwrap();

    let _: String = c.delete(&Endpoint::Order((&accountid, &orderid)))?;

    let resptxt: serde_json::Value = c.get(&Endpoint::Orders(accountid), &[param::Empty])?; // get working orders
    println!("orders remaining: ");
    prettyprint(&resptxt);

    Ok(())
}

fn prettyprint(toprint: &serde_json::Value) {
    println!("{}\n", serde_json::to_string_pretty(toprint).unwrap());
}
