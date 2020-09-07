use std::env;
use tdameritradeclient::{OptionChain, TDAClient};

fn main() {
    env_logger::init();

    let c = TDAClient::new(env::var("TDAUTHTOKEN").unwrap());
    // TODO: add symbol in params
    titleprint("Option Chain:");
    prettyprint(&c.getoptionchain(&[
        OptionChain::Symbol("SPY"),
        OptionChain::StrikeCount(3),
        OptionChain::ContractType("CALL"),
    ]));
}

fn prettyprint(toprint: &serde_json::Value) {
    println!("{}\n", serde_json::to_string_pretty(toprint).unwrap());
}

fn titleprint(heading: &str) {
    println!("{}", heading.to_uppercase());
    println!("{}", "-".repeat(heading.len()));
}
