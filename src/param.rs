/// Query parameters must implement Pair trait to allow us to use Url crate in
/// underlying http client to create the proper query_string
pub trait Pair<'a> {
    fn pair(self) -> (&'a str, String);
}
/// function to convert a collection of query parameters in to a vector of pairs
pub fn convert_to_pairs<'a, T>(queryparams: T) -> Vec<(&'a str, String)>
where
    T: IntoIterator,
    T::Item: Pair<'a>,
{
    let mut params: Vec<(&str, String)> = vec![];
    for qenum in queryparams.into_iter() {
        params.push(qenum.pair())
    }
    params
}

///
/// Query Parameters for /v1/accounts/
///
/// Balances displayed by default, additional fields can be added here by adding
/// positions or orders
///
#[derive(Debug)]
pub enum Account {
    Positions,
    Orders,
    PositionsAndOrders,
}

impl<'a> Pair<'a> for &Account {
    fn pair(self) -> (&'a str, String) {
        match self {
            Account::Positions => ("fields", String::from("positions")),
            Account::Orders => ("fields", String::from("orders")),
            Account::PositionsAndOrders => ("fields", String::from("positions,orders")),
        }
    }
}

///
/// Query Parameters for /v1/orders/
///
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

impl<'a> Pair<'a> for &Order<'a> {
    fn pair(self) -> (&'a str, String) {
        match self {
            Order::MaxResults(i) => ("maxResults", (*i).to_string()),
            Order::FromEnteredTime(s) => ("fromEnteredTime", (*s).to_string()),
            Order::ToEnteredTime(s) => ("toEnteredTime", (*s).to_string()),
            Order::Status(s) => ("status", (*s).to_string()),
        }
    }
}

///
/// Query Parameters for /v1/marketdata/{symbol}/pricehistory
///
#[derive(Debug)]
pub enum History<'a> {
    /// The type of period to show. Valid values are day, month, year, or
    /// ytd (year to date). Default is day.
    PeriodType(&'a str),
    /// Number of periods to show. Valid by `PeriodType` (* is default)
    /// day: 1, 2, 3, 4, 5, 10*
    /// month: 1*, 2, 3, 6
    /// year: 1*, 2, 3, 5, 10, 15, 20
    /// ytd: 1*
    Period(u8),
    /// the type of frequency with which a new candle is formed.
    /// Valid frequencyTypes by periodType (defaults marked with an asterisk):
    /// day: minute*
    /// month: daily, weekly*
    /// year: daily, weekly, monthly*
    /// ytd: daily, weekly*
    FrequencyType(&'a str),
    /// the number of the frequencyType to be included in each candle.
    /// Valid frequencies by frequencyType (defaults marked with an asterisk):
    /// minute: 1*, 5, 10, 15, 30
    /// daily: 1*
    /// weekly: 1*
    /// monthly: 1*
    Frequency(u8),
    /// Start date as milliseconds since epoch. If startDate and endDate
    /// are provided, period should not be provided.
    StartDate(u64),
    /// End date as milliseconds since epoch. If startDate and endDate are
    /// provided, period should not be provided. Default is previous trading day.
    EndDate(u64),
    /// true to return extended hours data, false for regular market hours only.
    /// Default is true
    NeedExendedHoursData(bool),
}

impl<'a> Pair<'a> for &History<'a> {
    fn pair(self) -> (&'a str, String) {
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

///
/// Query Parameters for /v1/marketdata/chains
///
#[derive(Debug)]
pub enum OptionChain<'a> {
    /// Underlying symbol <Required>
    Symbol(&'a str),
    /// Type of contracts to return in the chain. Can be CALL, PUT, or ALL. Default is ALL.
    ContractType(&'a str),
    /// The number of strikes to return above and below the at-the-money price.
    StrikeCount(u8),
    ///Passing a value returns a Strategy Chain. Possible values are SINGLE, ANALYTICAL (allows use of the
    /// volatility, underlyingPrice, interestRate, and daysToExpiration params to calculate theoretical values),
    /// COVERED, VERTICAL, CALENDAR, STRANGLE, STRADDLE, BUTTERFLY, CONDOR, DIAGONAL, COLLAR, or ROLL. Default is SINGLE.
    Strategy(&'a str),
    /// Strike interval for spread strategy chains (see strategy param).
    Interval(f64),
    /// Provide a strike price to return options only at that strike price.
    Strike(f64),
    /// Include quotes for options in the option chain. Can be TRUE or FALSE. Default is FALSE.
    IncludeQuotes(bool),
    /// Returns options for the given range. Possible values are:
    ///  ITM: In-the-money
    ///  NTM: Near-the-money
    ///  OTM: Out-of-the-money
    ///  SAK: Strikes Above Market
    ///  SBK: Strikes Below Market
    ///  SNK: Strikes Near Market
    ///  ALL: All Strikes
    ///  Default is ALL.
    Range(&'a str),
    /// Only return expirations after this date. For strategies, expiration refers to the nearest term expiration
    ///  in the strategy. Valid ISO-8601 formats are: yyyy-MM-dd and yyyy-MM-dd'T'HH:mm:ssz.
    FromDate(&'a str),
    /// Only return expirations before this date. For strategies, expiration refers to the nearest term expiration
    ///  in the strategy. Valid ISO-8601 formats are: yyyy-MM-dd and yyyy-MM-dd'T'HH:mm:ssz.
    ToDate(&'a str),
    /// Volatility to use in calculations. Applies only to ANALYTICAL strategy chains (see strategy param).
    Volatility(f64),
    /// Underlying price to use in calculations. Applies only to ANALYTICAL strategy chains (see strategy param)
    UnderlyingPrice(f64),
    /// Interest rate to use in calculations. Applies only to ANALYTICAL strategy chains (see strategy param)
    InterestRate(f64),
    /// Days to expiration to use in calculations. Applies only to ANALYTICAL strategy chains (see strategy param).
    DaysToExpiration(f64),
    /// 'Return only options expiring in the specified month. Month is given in the three character format.
    ///   Example: JAN, Default is ALL.
    ExpireMonth(&'a str),
    /// Type of contracts to return. Possible values are:
    ///  S: Standard contracts, NS: Non-standard contracts, ALL: All contracts. Default is ALL.
    OptionType(&'a str),
}

impl<'a> Pair<'a> for &OptionChain<'a> {
    fn pair(self) -> (&'a str, String) {
        match self {
            OptionChain::Symbol(s) => ("symbol", (*s).to_string()),
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
            OptionChain::ExpireMonth(s) => ("expMonth", (*s).to_string()),
            OptionChain::OptionType(s) => ("optionType", (*s).to_string()),
        }
    }
}
///
/// Query Parameters for /account/transactions
///
#[derive(Debug)]
pub enum Transactions<'a> {
    ///
    /// type = ALL, TRADE, BUY_ONLY, SELL_ONLY, CASH_IN_OR_CASH_OUT, CHECKING, DIVIDEND, INTEREST
    ///        OTHER, ADVISOR_FEES
    /// default = ALL
    TransactionType(&'a str),
    /// Specify symbol, otherwise all symbols
    Symbol(&'a str),
    /// Start Date in "yyyy-mm-dd"
    /// Maximum date range is one year
    StartDate(&'a str),
    /// End Date in "yyyy-mm-dd"
    /// Maximum data range is one year
    EndDate(&'a str),
}

impl<'a> Pair<'a> for &Transactions<'a> {
    fn pair(self) -> (&'a str, String) {
        match self {
            Transactions::TransactionType(s) => ("type", (*s).to_string()),
            Transactions::Symbol(s) => ("symbol", (*s).to_string()),
            Transactions::StartDate(s) => ("startDate", (*s).to_string()),
            Transactions::EndDate(s) => ("endDate", (*s).to_string()),
        }
    }
}

///
/// Query Parameters for /v1/instruments
///
#[derive(Debug)]
pub enum Instruments<'a> {
    /// Specify symbol or search parameter
    Symbol(&'a str),
    ///
    /// Type of Request
    /// symbol-search: Retrieve instrument data of a specific symbol or cusip
    /// symbol-regex: Retrieve instrument data for all symbols matching regex.
    ///      Example: symbol=XYZ.* will return all symbols beginning with XYZ
    /// desc-search: Retrieve instrument data for instruments whose description
    ///      contains the word supplied. Example: symbol=FakeCompany will return
    ///      all instruments with FakeCompany in the description.
    /// desc-regex: Search description with full regex support.
    ///      Example: symbol=XYZ.[A-C] returns all instruments whose descriptions
    ///      contain a word beginning with XYZ followed by a character A through C.
    /// fundamental: Returns fundamental data for a single instrument specified by exact symbol.
    SearchType(&'a str),
}

impl<'a> Pair<'a> for &Instruments<'a> {
    fn pair(self) -> (&'a str, String) {
        match self {
            Instruments::Symbol(s) => ("symbol", (*s).to_string()),
            Instruments::SearchType(s) => ("projection", (*s).to_string()),
        }
    }
}
