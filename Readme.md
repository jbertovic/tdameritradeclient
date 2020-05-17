## Description

A client that uses the TD Ameritrade API as described on [developer.tdameritrade.com].  The client does not currently handle authorization and assumes you have a valid auth token which is set through an environmental variable (see below). See tests as examples on how to use.

I have matching projects in my repository;
- tdacli - which acts as a command line interface to this library.  You can also look at it for examples.
- tokenstatemanager - which uses node.js to maintain a small server and a local mysql db to always have a valid token on hand.

## Example

```
use std::env;
use tdameritradeclient::{TDAClient, Execute}

fn main {
    //set key and token from environment variables
    let consumerkey = env::var("TDCONSUMERKEY").unwrap();
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
|TDCONSUMERKEY|Used to create new `TDAClient`| as registered on [developer.tdameritrade.com]
|TDAUTHTOKEN|Used to create new `TDAClient`| you will need to manually create as per [developer.tdameritrade.com] 

## Current TODO

- [ ] grab account data (endpoint below /accounts)
- [ ] able to view saved and current orders with filter (endpoint below /saveorders)
- [ ] able to create and delete saved order  (endpoint below with PUT and DELETE /saveorders)
- [ ] a method to filter out response into the data required (need to think about how to keep this simple)


## Future IDEAS
- [ ] use `refresh` token instead of actual token if maintaining a client
- [ ] add documentation
- [ ] add better error checking


## Endpoints added
see [https://developer.tdameritrade.com/user-principal/apis]

- [X] GET /marketdata/quotes
- [X] GET /userprincipals
- [X] GET /marketdata/{}/pricehistory
- [X] GET /marketdata/chains

## Endpoints working on
- [ ] GET /accounts/{}
- [ ] GET /accounts/{}/savedorders
- [ ] POST /accounts/{}/savedorders
- [ ] DELETE /accounts/{}/savedorders
