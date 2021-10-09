/// specifies Endpoints for TD Ameritrade's API
///
#[derive(Debug)]
pub enum Endpoint<'a> {
    ///
    /// /userprincipals
    ///
    /// get user information including all linked accounts
    ///
    UserPrincipals,
    ///
    /// /accounts
    ///
    /// get all account information like ids, access level, market approval, etc.
    ///
    Accounts,
    ///
    /// /accounts/{ACCOUNTID}
    ///
    /// get specific account info
    ///
    /// optional `param::Account` to include positions and orders
    ///
    Account(&'a str),
    ///
    /// /accounts/{ACCOUNTID}/orders/{ORDERID}
    ///
    /// get order details for specified account id for specific order id
    ///
    /// delete to cancel order
    ///
    /// put to replace order; need to include body that will replace original order
    ///
    /// First parameter is ACCOUNTID and second parameter is ORDERID
    ///
    Order((&'a str, &'a str)),
    ///
    /// /accounts/{ACCOUNTID}/orders
    ///
    /// get order details for specified account
    ///
    /// post body with order details to submit order
    ///
    Orders(&'a str),
    ///
    /// /accounts/{ACCOUNTID}/savedorders/{SAVEDORDERID}
    ///
    /// get order details for specified account id for specific order id
    ///
    /// delete to cancel order
    ///
    /// put to replace order; need to include body that will replace original order
    ///
    /// First parameter is ACCOUNTID and second parameter is SAVEDORDERID
    ///
    SavedOrder((&'a str, &'a str)),
    ///
    /// /accounts/{ACCOUNTID}/savedorders
    ///
    /// get order details for specified account
    ///
    /// post body with order details to submit order
    ///
    SavedOrders(&'a str),
    ///
    /// /marketdata/quotes
    ///
    /// get quotes for a list of symbols. example: "SPY,IWM,QQQ"
    ///
    Quotes,
    ///
    /// /marketdata/{MARKET}/hours
    ///
    /// get todays market hours for given market
    ///
    /// MARKET can be EQUITY, OPTION, FUTURE, BOND, or FOREX
    ///
    /// optional `param::MarketHours` to specify market hours for a specific date
    ///
    MarketHours(&'a str),
    ///
    /// Search Instruments
    ///
    /// use `param::Instruments` for supplying Search parameters
    ///
    Instruments,
    ///
    /// Get Instrument by CUSIP
    ///
    Instrument(&'a str),
    ///
    /// /marketdata/{SYMBOL}/pricehistory
    ///
    /// search for instruments
    ///
    /// additional query parameters need to be added from `param::History` Enum
    ///
    History(&'a str),
    ///
    /// /marketdata/chains
    ///
    /// additional query parameters need to be added from `param::OptionChain` Enum
    ///
    OptionChain,
    ///
    /// /accounts/{ACCOUNTID}/transactions
    ///
    /// get transactions in a specified Account
    ///
    /// additional query parameters can be added from `param::Transactions` Enum
    ///
    Transactions(&'a str),
    ///
    /// /accounts/{ACCOUNTID}/transactions/{TRANSACTIONID}
    ///
    /// get in a specified Account a specified transaction by Id
    ///
    /// First parameter is ACCOUNTID and second parameter is TRANSACTIONID
    ///
    Transaction((&'a str, &'a str)),
    ///
    /// /accounts/{ACCOUNTID}/watchlists
    ///
    /// get all watchlists for specified account
    ///
    /// post to create watchlist
    ///
    Watchlists(&'a str),
    ///
    /// /accounts/{ACCOUNTID}/watchlists/{WATCHLISTID}
    ///
    /// get a specific watchlid for a specified account
    ///
    /// put to replace watchlist
    ///
    /// patch to update watchlist
    ///
    /// delete to remove watchlist
    ///
    Watchlist((&'a str, &'a str)),
    ///
    /// **Has not been tested**
    ///
    /// /marketdata/{INDEX}/movers
    ///
    /// get mover information by index symbol, direction type and change
    ///
    /// INDEX: $SPX.X, $COMPX, $DJI
    ///
    Movers(&'a str),
}

impl<'a> Endpoint<'a> {
    /// defines the URL for the specified Endpoint
    pub fn url_endpoint(&self) -> String {
        match self {
            Endpoint::UserPrincipals => format!("{}userprincipals", crate::APIWWW),
            Endpoint::Accounts => format!("{}accounts", crate::APIWWW),
            Endpoint::Account(account) => format!("{}accounts/{}", crate::APIWWW, account),
            Endpoint::Order((account, order)) => {
                format!("{}accounts/{}/orders/{}", crate::APIWWW, account, order)
            }
            Endpoint::Orders(account) => format!("{}accounts/{}/orders", crate::APIWWW, account),
            Endpoint::SavedOrder((account, savedorder)) => format!(
                "{}accounts/{}/savedorders/{}",
                crate::APIWWW,
                account,
                savedorder
            ),
            Endpoint::SavedOrders(account) => {
                format!("{}accounts/{}/savedorders", crate::APIWWW, account)
            }
            Endpoint::Quotes => format!("{}marketdata/quotes", crate::APIWWW),
            Endpoint::MarketHours(market) => {
                format!("{}marketdata/{}/hours", crate::APIWWW, market)
            }
            Endpoint::Instruments => format!("{}instruments", crate::APIWWW),
            Endpoint::Instrument(cusip) => format!("{}instruments/{}", crate::APIWWW, cusip),
            Endpoint::History(symbol) => {
                format!("{}marketdata/{}/pricehistory", crate::APIWWW, symbol)
            }
            Endpoint::OptionChain => format!("{}marketdata/chains", crate::APIWWW),
            Endpoint::Transactions(account) => {
                format!("{}accounts/{}/transactions", crate::APIWWW, account)
            }
            Endpoint::Transaction((account, transaction)) => format!(
                "{}accounts/{}/transactions/{}",
                crate::APIWWW,
                account,
                transaction
            ),
            Endpoint::Watchlists(account) => {
                format!("{}accounts/{}/watchlists", crate::APIWWW, account)
            }
            Endpoint::Watchlist((account, watchlist)) => format!(
                "{}accounts/{}/watchlists/{}",
                crate::APIWWW,
                account,
                watchlist
            ),
            Endpoint::Movers(index) => format!("{}marketdata/{}/movers", crate::APIWWW, index),
        }
    }
}
