## Description

A client that uses the TD Ameritrade API as described on [developer.tdameritrade.com].  The client does not currently handle authorization and assumes you have a valid auth token which is set through an environmental variable (see below). See tests as examples on how to use.

This client uses a blocking web request client library [attohttps](https://github.com/sbstp/attohttpc).

I have matching projects in my repository;
- [tdacli](https://github.com/jbertovic/tdacli) - which acts as a command line interface to this library.  You can also look at it for examples.
- [tokenstatemanager](https://github.com/jbertovic/tokenstatemanager) - which uses node.js to maintain a small server and a local mysql db to always have a valid token on hand.

## Example

see unit tests in `lib.rs` for ideas
```
use std::env;
use tdameritradeclient::{TDAClient, Execute}

fn main() {

    //set key and token from environment variables
    let token = env::var("TDAUTHTOKEN").unwrap();

    // initiate client
    let c = TDAClient::new(consumerkey, token);

    // get quotes for 3 symbols and execute
    let resptxt: String = c.getquotes("F,INTC,TRP").execute();

    // output will be text string in json format
    println!("{:?}", resptxt);
}
```


## Setup

Environment Variables required
|Property|Location|Description
|---|---|---
|TDAUTHTOKEN|Used to create new `TDAClient`| you will need to manually create as per [developer.tdameritrade.com] 

## Current TODO
- [X] grab account data (endpoint below /accounts)
- [X] How do I use the enum better to correspond to the proper endpoint?
- [X] able to view saved and current orders with filter (endpoint below /saveorders)
- [ ] able to create and delete saved order  (endpoint below with PUT and DELETE /saveorders)
- [ ] create example to pull history, optionchain, and quote
- [ ] create example to create saved order
- [ ] create feature options from serde_json and chrono

## Future IDEAS
- [ ] use `refresh` token instead of actual token if maintaining a client
- [ ] add documentation
- [ ] add better error checking
- [ ] grouping param pairs on function call instead of only chaining


## Endpoints added
see [https://developer.tdameritrade.com/apis]

- [X] GET /userprincipals
- [X] GET /accounts/{}
- [X] GET /marketdata/quotes
- [X] GET /marketdata/{}/pricehistory
- [X] GET /marketdata/chains
- [X] GET /accounts/{}?fields=positions
- [X] GET /accounts/{}?fields=orders
- [X] GET /accounts/{}?fields=positions,orders
- [X] GET /marketdata/{}/pricehistory?parameters*  
- [X] GET /marketdata/chains?parameters* 
- [X] GET /accounts/{}/savedorders

## Endpoints working on
- [ ] POST /accounts/{}/savedorders 
- [ ] DELETE /accounts/{}/savedorders 
- [ ] POST /accounts/{}/orders 
- [ ] DELETE /accounts/{}/orders 
