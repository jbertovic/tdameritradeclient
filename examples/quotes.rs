use std::env;
use tdameritradeclient::{param, Endpoint, TDAClient};

fn main() {
    env_logger::init();

    // grab authorization token from environmental variable
    let c = TDAClient::new(env::var("TDAUTHTOKEN").unwrap());

    // get quotes endpoint with a variety of symbols supplied as parameters
    title_print("Quotes:");
    pretty_print(&c.get(
        &Endpoint::Quotes,
        &[param::Quotes::Symbol("F,SPY,INTC,IWM")],
    ));
}

fn pretty_print(toprint: &serde_json::Value) {
    println!("{}\n", serde_json::to_string_pretty(toprint).unwrap());
}

fn title_print(heading: &str) {
    println!("{}", heading.to_uppercase());
    println!("{}", "-".repeat(heading.len()));
}
