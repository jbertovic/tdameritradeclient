use std::env;
use tdameritradeclient::{TDAClient, OptionChain};

fn main() {

    let c = TDAClient::new(env::var("TDAUTHTOKEN").unwrap());

    titleprint("Option Chain:");
    prettyprint(&c.getoptionchain(
    &[
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