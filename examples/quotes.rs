use std::env;
use tdameritradeclient::TDAClient;

fn main() {
    env_logger::init();

    let c = TDAClient::new(env::var("TDAUTHTOKEN").unwrap());
    title_print("Quotes:");
    pretty_print(&c.get_quotes("F,SPY,INTC,IWM"));
}

fn pretty_print(toprint: &serde_json::Value) {
    println!("{}\n", serde_json::to_string_pretty(toprint).unwrap());
}

fn title_print(heading: &str) {
    println!("{}", heading.to_uppercase());
    println!("{}", "-".repeat(heading.len()));
}
