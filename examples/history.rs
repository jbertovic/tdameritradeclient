use std::env;
use tdameritradeclient::{History, TDAClient};

fn main() {
    env_logger::init();
    let c = TDAClient::new(env::var("TDAUTHTOKEN").unwrap());
    title_print("History:");
    pretty_print(&c.get_history(
        "SPY",
        &[
            History::Period(1),
            History::PeriodType("month"),
            History::Frequency(1),
            History::FrequencyType("daily"),
        ],
    ));
}

fn pretty_print(toprint: &serde_json::Value) {
    println!("{}\n", serde_json::to_string_pretty(toprint).unwrap());
}

fn title_print(heading: &str) {
    println!("{}", heading.to_uppercase());
    println!("{}", "-".repeat(heading.len()));
}
