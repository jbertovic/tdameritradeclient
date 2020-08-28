use std::env;
use tdameritradeclient::{History, TDAClient};

fn main() {
    env_logger::init();
    let c = TDAClient::new(env::var("TDAUTHTOKEN").unwrap());
    titleprint("History:");
    prettyprint(&c.gethistory(
        "SPY",
        &[
            History::Period(1),
            History::PeriodType("month"),
            History::Frequency(1),
            History::FrequencyType("daily"),
        ],
    ));
}

fn prettyprint(toprint: &serde_json::Value) {
    println!("{}\n", serde_json::to_string_pretty(toprint).unwrap());
}

fn titleprint(heading: &str) {
    println!("{}", heading.to_uppercase());
    println!("{}", "-".repeat(heading.len()));
}
