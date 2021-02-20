use std::env;
use tdameritradeclient::TDAClient;

fn main() {
    env_logger::init();

    let c = TDAClient::new(env::var("TDAUTHTOKEN").unwrap());

    let resptxt: serde_json::Value = c.get_user_principals();
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
              "symbol": "INTC",
              "assetType": "EQUITY"
            }
          }
        ]
      }
    "#;

    let resptxt = c.create_order(&accountid, order_def);
    println!("Order Created: {}", resptxt); // list working orders

    let resptxt: serde_json::Value = c.get_orders(&accountid, &[]); // get working orders and find the order above
    pretty_print(&resptxt);
}

fn pretty_print(toprint: &serde_json::Value) {
    println!("{}\n", serde_json::to_string_pretty(toprint).unwrap());
}
