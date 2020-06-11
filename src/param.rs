#[derive(Debug)]
pub enum Account {
    Positions,
    Orders,
    PositionsAndOrders,
}

impl Into<(&'static str, String)> for &Account {
    fn into(self) -> (&'static str, String) {
        match self {
            Account::Positions => ("fields", String::from("positions")),
            Account::Orders => ("fields", String::from("orders")),
            Account::PositionsAndOrders => ("fields", String::from("positions,orders")),
        }
    }
}

#[derive(Debug)]
pub enum Order<'a> {
    /// Max number of orders to retrieve
    MaxResults(u8),
    /// Specifies that no orders entered before this time should be returned.
    /// Must be 60 days from today's date
    /// format yyyy-mm-dd
    FromEnteredTime(&'a str),
    /// Specifies that no orders entered after this time should be returned.
    /// format yyyy-mm-dd
    ToEnteredTime(&'a str),
    /// specifies type of orders to be returned: WORKING, FILLED, EXPIRED, etc...
    Status(&'a str),
}

impl<'a> Into<(&'static str, String)> for &Order<'a> {
    fn into(self) -> (&'static str, String) {
        match self {
            Order::MaxResults(i) => ("maxResults", (*i).to_string()),
            Order::FromEnteredTime(s) => ("fromEnteredTime", (*s).to_string()),
            Order::ToEnteredTime(s) => ("toEnteredTime", (*s).to_string()),
            Order::Status(s) => ("status", (*s).to_string()),
        }
    }
}


#[derive(Debug)]
pub enum History<'a> {
    PeriodType(&'a str),
    Period(u8),
    FrequencyType(&'a str),
    Frequency(u8),
    StartDate(u64),
    EndDate(u64),
    NeedExendedHoursData(bool),
}

impl<'a> Into<(&'static str, String)> for &History<'a> {
    fn into(self) -> (&'static str, String) {
        match self {
            History::PeriodType(s) => ("periodType", (*s).to_string()),
            History::Period(i) => ("period", (*i).to_string()),
            History::FrequencyType(s) => ("frequencyType", (*s).to_string()),
            History::Frequency(i) => ("frequency", (*i).to_string()),
            History::StartDate(i) => ("startDate", (*i).to_string()),
            History::EndDate(i) => ("endDate", (*i).to_string()),
            History::NeedExendedHoursData(b) => ("needExtendedHoursData", (*b).to_string()),
        }
    }
}

#[derive(Debug)]
pub enum OptionChain<'a> {
    ContractType(&'a str),
    StrikeCount(u8),
    Strategy(&'a str),
    Interval(u8),
    Strike(f64),
    IncludeQuotes(bool),
    Range(&'a str),
    FromDate(&'a str),
    ToDate(&'a str),
    Volatility(f64),
    UnderlyingPrice(f64),
    InterestRate(f64),
    DaysToExpiration(f64),
    ExpireMonth(&'a str),
    OptionType(&'a str),
}

impl<'a> Into<(&'static str, String)> for &OptionChain<'a> {
    fn into(self) -> (&'static str, String) {
        match self {
            OptionChain::ContractType(i) => ("contractType", (*i).to_string()),
            OptionChain::Strategy(s) => ("strategy", (*s).to_string()),
            OptionChain::StrikeCount(i) => ("strikeCount", (*i).to_string()),
            OptionChain::Interval(i) => ("interval", (*i).to_string()),
            OptionChain::Strike(i) => ("strike", (*i).to_string()),
            OptionChain::IncludeQuotes(b) => ("includeQuotes", b.to_string()),
            OptionChain::Range(s) => ("range", (*s).to_string()),
            OptionChain::FromDate(s) => ("fromDate", (*s).to_string()),
            OptionChain::ToDate(s) => ("toDate", (*s).to_string()),
            OptionChain::Volatility(i) => ("volatility", (*i).to_string()),
            OptionChain::UnderlyingPrice(i) => ("underlyingPrice", (*i).to_string()),
            OptionChain::InterestRate(i) => ("interestRate", (*i).to_string()),
            OptionChain::DaysToExpiration(i) => ("daysToExpiration", (*i).to_string()),
            OptionChain::ExpireMonth(s) => ("expireMonth", (*s).to_string()),
            OptionChain::OptionType(s) => ("optionType", (*s).to_string()),
        }
    }
}

