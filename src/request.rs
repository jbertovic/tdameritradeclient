pub enum _RequestType {
    Get,
}

pub enum Endpoint<'a> {
    ///
    /// /userprincipals
    /// get user information including all linked accounts
    /// 
    UserPrincipals,
    ///
    /// /accounts
    /// get all account information like ids, access level, market approval, etc.
    /// 
    Accounts,
    /// 
    /// /accounts/{ACCOUNTID}
    /// get specific account info
    /// optional `param::Account` to include positions and orders
    /// 
    Account(&'a str),
    ///
    /// /accounts/{ACCOUNTID}/orders
    /// get order details for specified account
    /// 
    Orders(&'a str),
    ///
    /// /marketdata/quotes
    /// get quotes for a list of symbols. example: "SPY,IWM,QQQ"
    /// 
    Quotes,
    ///
    /// /marketdata/{MARKET}/hours
    /// get todays market hours for given market
    /// MARKET can be EQUITY, OPTION, FUTURE, BOND, or FOREX
    /// optional `param::MarketHours` to specify market hours for a specific date
    ///
    MarketHours(&'a str),
    ///
    /// Search Instruments
    /// use `param::Instruments` for supplying Search parameters
    /// 
    Instruments,
    ///
    /// Get Instrument by CUSIP
    /// 
    Instrument(&'a str),
    ///
    /// /marketdata/{SYMBOL}/pricehistory
    /// additional query parameters need to be added from `param::History` Enum
    /// 
    History(&'a str),
    ///
    /// /marketdata/chains
    /// additional query parameters need to be added from `param::OptionChain` Enum
    /// 
    OptionChain,
    ///
    /// /accounts/{ACCOUNTID}/transactions
    /// retrieve transactions in a specified Account
    /// additional query parameters can be added from `param::Transactions` Enum
    /// 
    Transactions(&'a str),
    ///
    /// /accounts/{account}/transactions/{transactionId}
    /// retrieve in a specified Account a specified transaction by Id 
    /// 
    Transaction((&'a str, &'a str)),

}

impl<'a> Endpoint<'a> {
    pub fn url_endpoint(&self) -> String {
        match self {
            Endpoint::UserPrincipals => format!("{}userprincipals", crate::APIWWW),
            Endpoint::Accounts => format!("{}accounts", crate::APIWWW),
            Endpoint::Account(account) => format!("{}accounts/{}", crate::APIWWW, account),
            Endpoint::Orders(account) => format!("{}accounts/{}/orders", crate::APIWWW, account),
            Endpoint::Quotes => format!("{}marketdata/quotes", crate::APIWWW),
            Endpoint::MarketHours(market) => format!("{}marketdata/{}/hours", crate::APIWWW, market),
            Endpoint::Instruments => format!("{}instruments", crate::APIWWW),
            Endpoint::Instrument(cusip) => format!("{}instruments/{}", crate::APIWWW, cusip),
            Endpoint::History(symbol) => format!("{}marketdata/{}/pricehistory", crate::APIWWW, symbol),
            Endpoint::OptionChain => format!("{}marketdata/chains", crate::APIWWW),
            Endpoint::Transactions(account) => format!("{}accounts/{}/transactions", crate::APIWWW, account),
            Endpoint::Transaction((account, transaction)) => format!("{}accounts/{}/transactions/{}", crate::APIWWW, account, transaction),
        }
    }
}