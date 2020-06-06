use std::env;
use tdameritradeclient::{TDAClient};

fn main() {

    let c = TDAClient::new(env::var("TDAUTHTOKEN").unwrap());

    let resptxt: serde_json::Value = c.getuserprincipals();
    let accountid = resptxt["primaryAccountId"].as_str().unwrap();

    let order_def = r#"
    {
        "orderType": "LIMIT",
        "session": "NORMAL",
        "price": "30.01",
        "duration": "DAY",
        "orderStrategyType": "SINGLE",
        "orderLegCollection": [
          {
            "instruction": "Buy",
            "quantity": 100,
            "instrument": {
              "symbol": "TRP",
              "assetType": "EQUITY"
            }
          }
        ]
      }
    "#;

    let resptxt = c.createorder(&accountid, order_def); 
    println!("Order Created: {}", resptxt); // list working orders

    let resptxt: serde_json::Value = c.getorders(&accountid); // get working orders and find the order above

    prettyprint(&resptxt); // need to find the right ID so i can try replacing the order
    //get input from keyboard before proceeding

    
    println!("\nReplacing Order");
    //get input from keyboard before proceeding




    //find the new order number again and then delete order.
    //get input from keyboard before proceeding

    println!("\nDeleting Order");



}

fn prettyprint(toprint: &serde_json::Value) {
    println!("{}\n", serde_json::to_string_pretty(toprint).unwrap());
}