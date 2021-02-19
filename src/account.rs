//Module for working with the account endpoint.
//Contains structs for balances, positions, and orders. Also includes the functions for getting and making them
use super::*;
use serde::{Serialize, Deserialize};
//use tdameritradeclient::{TDAClient, Account};

//Functions for getting data and serlializing/deserializing it to custom structs:
//
//Gets JSON of balances and positions
fn get_positions(client: &TDAClient, accountid: &str) -> std::io::Result<serde_json::Value> {
    Ok(client.get_account(accountid, &[Account::Positions]))
}
//
//Gets JSON of balances and orders
fn get_orders(client: &TDAClient, accountid: &str) -> std::io::Result<serde_json::Value> {
    Ok(client.get_account(accountid, &[Account::Orders]))
}
//
//Gets the account's balances and positions and deserializes the response into a custom struct
pub fn get_positions_struct(client: &TDAClient, accountid: &str) -> std::io::Result<SecuritiesAccount>{
    let value = account::get_positions(client, accountid)?; //Get JSON
    let pos_struct = serde_json::from_value(value["securitiesAccount"].clone())?; //Convert JSON to rust struct
    Ok(pos_struct)//Return struct or error
}
//
//Gets the account's balances and positions and deserializes the response into a custom struct containing only the orders
pub fn get_orders_struct(client: &TDAClient, accountid: &str) -> std::io::Result<SecuritiesAccount>{
    //One line way of doing the same things as get_positions_struct
    Ok(serde_json::from_value(account::get_orders(client, accountid)?["securitiesAccount"].clone())?)}
//
//Structs for holding balances, positions and orders 
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
struct OrderLegCollection {
    instruction: String,
    instrument: Instrument,
    leg_id: u8,
    order_leg_type: String,
    position_effect: String,
    quantity: f64
}
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
struct OrderStrategies {
    account_id: f64,
    cancel_time: String,
    cancelable: bool,
    complex_order_strategy_type: String,
    destination_link_name: String,
    duration: String,
    editable: bool,
    entered_time: String,
    filled_quantity: f64,
    order_id: f64,
    order_leg_collection: Vec<OrderLegCollection>,
    order_strategy_type: String,
    order_type: String,
    price: f64,
    quantity: f64,
    remaining_quantity: f64,
    requested_destination: String,
    session: String,
    status: String,
    tag: String
}
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
struct ProjectedBalances {
    available_funds: f64,
    available_funds_non_marginable_trade: f64,
    buying_power: f64,
    day_trading_buying_power: f64,
    day_trading_buying_power_call: f64,
    is_in_call: bool,
    maintenance_call: f64,
    reg_t_call: f64,
    stock_buying_power: f64
}
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
struct Positions {
    average_price: f64,
    current_day_profit_loss: f64,
    current_day_profit_loss_percentage: f64,
    instrument: Instrument,
    long_quantity: f64,
    maintenance_requirement: f64,
    market_value: f64,
    settled_long_quantity: f64,
    settled_short_quantity: f64,
    short_quantity: f64
}
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
struct Instrument {
    asset_type: String,
    cusip: String,
    symbol: String
}
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
struct InitialBalances {
    account_value: f64,
    accrued_interest: f64,
    available_funds_non_marginable_trade: f64,
    bond_value: f64,
    buying_power: f64,
    cash_available_for_trading: f64,
    cash_balance: f64,
    cash_receipts: f64,
    day_trading_buying_power: f64,
    day_trading_buying_power_call: f64,
    day_trading_equity_call: f64,
    equity: f64,
    equity_percentage: f64,
    is_in_call: bool,
    liquidation_value: f64,
    long_margin_value: f64,
    long_option_market_value: f64,
    long_stock_value: f64,
    maintenance_call: f64,
    maintenance_requirement: f64,
    margin: f64,
    margin_balance: f64,
    margin_equity: f64,
    money_market_fund: f64,
    mutual_fund_value: f64,
    pending_deposits: f64,
    reg_t_call: f64,
    short_balance: f64,
    short_margin_value: f64,
    short_option_market_value: f64,
    short_stock_value: f64,
    total_cash: f64
}
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
struct CurrentBalances {
    accrued_interest: f64,
    available_funds: f64,
    available_funds_non_marginable_trade: f64,
    bond_value: f64,
    buying_power: f64,
    buying_power_non_marginable_trade: f64,
    cash_balance: f64,
    cash_receipts: f64,
    day_trading_buying_power: f64,
    equity: f64,
    equity_percentage: f64,
    liquidation_value: f64,
    long_margin_value: f64,
    long_market_value: f64,
    long_option_market_value: f64,
    maintenance_call: f64,
    maintenance_requirement: f64,
    margin_balance: f64,
    money_market_fund: f64,
    mutual_fund_value: f64,
    pending_deposits: f64,
    reg_t_call: f64,
    savings: f64,
    short_balance: f64,
    short_margin_value: f64,
    short_market_value: f64,
    short_option_market_value: f64,
    sma: f64
}
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct SecuritiesAccount {
    pub account_id: String,
    current_balances: CurrentBalances,
    initial_balances: InitialBalances,
    is_closing_only_restricted: bool,
    is_day_trader: bool,
    #[serde(default)]
    order_strategies: Vec<OrderStrategies>,
    #[serde(default)]
    positions: Vec<Positions>,
    projected_balances: ProjectedBalances,
    round_trips: u8,
    r#type: String
}
//Methods and helper functions
impl SecuritiesAccount {
    //
    //Retrieves a Vector of references for all positions for a given symbol
    fn get_position(&self, symbol: &str) -> std::io::Result<Vec<&account::Positions>>{
        let matchingpositions: Vec<&account::Positions> = self.positions
            .iter()
            .filter(|x| x.instrument.symbol == symbol)
            .collect();
        Ok(matchingpositions)
    }
    //
    //Totals all positions for a symbol 
    fn total_position(position: Vec<&account::Positions>) -> std::io::Result<f64>{
        let mut total = 0.0;
        position.iter().for_each(|x| total += x.market_value);
        Ok(total)
    }
}

/* //TODO: Tests currently broken because I used functions from a unincluded crate.
#[cfg(test)]
mod tests{
    use super::*;
    //
    //Create a SecuritesAccount struct with known fields for testing
    fn new_teststruct() -> std::io::Result<SecuritiesAccount>{
        //Assemble test struct for testing
        let mut teststruct: SecuritiesAccount = Default::default();
        let testinstrument = Instrument {
                asset_type:  "EQUITY".to_string(),
                cusip: "G3682E192".to_string(),
                symbol: "FRO".to_string()
        };
        let testpositions = Positions {
            average_price: 6.47,
            current_day_profit_loss: 0.0,
            current_day_profit_loss_percentage: 0.0,
            instrument: testinstrument,
            long_quantity: 1.0,
            maintenance_requirement: 1.95,
            market_value: 6.49,
            settled_long_quantity: 0.0,
            settled_short_quantity: 0.0,
            short_quantity: 0.0
        };
        teststruct.positions.push(testpositions);
        Ok(teststruct)
    }
    #[test]
    fn test_total_postion() -> std::io::Result<()>{
        let teststruct = new_teststruct().unwrap();
        let total = account::SecuritiesAccount::total_position(teststruct.get_position("FRO").unwrap())?;
        assert_eq!(total, 6.49); // Check total is equal to the known value of the position we created for testing
        Ok(())
    }
    #[test]
    fn test_retreive_position() -> std::io::Result<()> {
        //Assemble test struct for testing
        let teststruct: SecuritiesAccount = account::tests::new_teststruct()?;
        
        //Test retreive positions
        let testpostions = teststruct.get_position("FRO")?;
        assert!(testpostions.len() > 0);
        Ok(())
    }
    #[test]
    fn test_get_positions() -> std::io::Result<()>{
        tokens::renew_access()?;
        let client = TDAClient::new(tokens::read_access().unwrap());
        let userprincipals: serde_json::Value = tokens::read_principals()?;
        let accountid = tokens::get_accountid(&userprincipals)?;
        let _testvalue: serde_json::Value = get_positions(&client, accountid)?;
        Ok(())
    }
    #[test]
    fn test_get_positions_struct() -> std::io::Result<()>{
        tokens::renew_access()?;
        let client = TDAClient::new(tokens::read_access().unwrap());
        let userprincipals: serde_json::Value = tokens::read_principals()?;
        let accountid = tokens::get_accountid(&userprincipals)?;
        let _teststruct: SecuritiesAccount = get_positions_struct(&client, accountid)?;
        Ok(())
    }
    #[test]
    fn test_get_orders() -> std::io::Result<()>{
        tokens::renew_access()?;
        let client = TDAClient::new(tokens::read_access().unwrap());
        let userprincipals: serde_json::Value = tokens::read_principals()?;
        let accountid = tokens::get_accountid(&userprincipals)?;
        let _testvalue: serde_json::Value = get_orders(&client, accountid)?;
        Ok(())
    }
    #[test]
    fn test_get_orders_struct() -> std::io::Result<()>{
        tokens::renew_access()?;
        let client = TDAClient::new(tokens::read_access().unwrap());
        let userprincipals: serde_json::Value = tokens::read_principals()?;
        let accountid = tokens::get_accountid(&userprincipals)?;
        let _teststruct: SecuritiesAccount = get_orders_struct(&client, accountid)?;
        Ok(())
    }
}
*/