use std::env;
use tdameritradeclient::{TDAClient};

fn main() {

    let c = TDAClient::new(env::var("TDAUTHTOKEN").unwrap());
    titleprint("Quotes:");
    prettyprint(&c.getquotes("F,SPY,INTC,IWM"));
}

fn prettyprint(toprint: &serde_json::Value) {
    println!("{}\n", serde_json::to_string_pretty(toprint).unwrap());
}

fn titleprint(heading: &str) {
    println!("{}", heading.to_uppercase());
    println!("{}", "-".repeat(heading.len()));
}