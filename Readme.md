**Disclaimer:** I'm not endorsing and am not affiliated with TD Ameritrade. Be careful using the API and understand that actual orders can be created through this library.  Read and understand the TD Ameritrade's api terms of service and documentation before using.

## Note Version 0.4.4 Changes
- Added error checking on TDauth module. Used when retrieving tokens.
- Gave TDAClientAuth acces to TDauth struct
- Managed TDAClientAuth has the ability to deal with no token returned due to some error in web retrieval or authorization
- Integrated TDauth module better with TDAClientAuth
- Added model::pricehistory and tests

## Note Version 0.4 Changes

I changed my approach to this wrapper around TD Ameritrade's API.  Previous versions used multiple functions on the client to specify each endpoint.  This version has a major change in that it specifies the endpoints in the request module as an Endpoint enum.  So now both the parameters for query parameters and specifying the endpoints are kept in enums.  I have updated the examples to show how to use them.  This change will break backward compatibility to previous versions.  Feedback is welcomed.   

I added a model module to contain response types that can be parsed from the json output.  I tried using TD Ameritrade Schema's that were located on their developer site, however, they don't always match or there is additional data available.  Therefore, I'm creating these response type's manually - experimenting a bit.  Would be much more useful if there was a way to code generate the types that match each response.  Its your option if you want to use them, or stick with json or define your own response types.

# tdameritradeclient

[Documentation](https://jbertovic.github.io/tdameritradeclient/doc/tdameritradeclient/index.html)

## Description

A client that uses the TD Ameritrade API as described on (http://developer.tdameritrade.com).  The client has a utility module `tdameritrade::auth::TDauth` to help with authorization or you can use the client directly with a valid auth token.  I typically use environmental variables to pass tokens and config (see below). I've included examples in the `./examples` folder and in some of the tests.

This client uses a blocking web request client library [attohttps](https://github.com/sbstp/attohttpc).

I have matching CLI tool using this library in my repository: [tdacli](https://github.com/jbertovic/tdacli). It also contains additional examples.

## Example

see unit tests in [./tests/clienttests.rs](https://github.com/jbertovic/tdameritradeclient/blob/master/tests/clienttests.rs) and examples in [./examples/](https://github.com/jbertovic/tdameritradeclient/tree/master/examples) for ideas

For the examples and tests to work you will need to set environmental variables for passing a `token`.  The tests that are indicated with `ignore` should be run one at a time.

```
use std::env;
use tdameritradeclient::{TDAClient, Endpoint, param};

// Will need to set TDAUTHTOKEN as environmental variable containing a valid token

fn main() {

    //set token from environment variables
    let token = env::var("TDAUTHTOKEN").unwrap();

    // initiate client
    let c = TDAClient::new(token);

    // get quotes for 3 symbols and execute
    let resptxt: String = c.get(&Endpoint::Quotes, &[param::Quotes::Symbol("F,SPY,INTC,IWM")]);

    // output will be text string in json format
    println!("{:?}", resptxt);
}
```


## Setup

You will need to determine a way to supply a method of authorization to the client.  The client is simple and will only take a valid token to perform the requests.  The client will not automatically manage the token refresh.  See `auth` module below.

You can implement the client in one of 3 ways:
1) Supply valid ***token*** that will be used in the request as `Bearer <token>`
2) Supply valid ***refresh*** token that will be used to request a valid token for the client as per 1
3) Supply ***authorization code*** that can be used to grab a valid token for the client as per 1

See [developer.tdameritrade.com](http://developer.tdameritrade.com) on how to manually create either of the 3 auth codes listed above or use the below `auth` module to help.

NEW in v0.4.3 - You have the option of using `TDAClientAuth` which would require a refresh_token and client_id.  It will check the validity of the tokens and update if required prior to each request.

## Authorization module

I've included utility tools under `auth` module to help deal with managing tokens using the above 3 options.  See documentation for extra information.

Example using `refresh_token` to renew `token`

```
use std::env;
use tdameritradeclient::auth::TDauth;

// set TDREFRESHTOKEN as environmental variable containing a valid refresh token
// set TDCLIENTKEY as environmental variable containing a valid Consumer Key as registered on developer.tdameritrade.com

fn main() {
        let refresh = env::var("TDREFRESHTOKEN").unwrap();
        let clientid = env::var("TDCLIENTKEY").unwrap();
        let newtdauth = TDauth::new_fromrefresh(&refresh, &clientid);
        println!("{:?}", newtdauth);
}


> cargo run
TDauth { token: "TOKEN_REMOVED_SOME_FOR_EXAMPLE_FOR_SECURITYjoTQVUYGGaevMrDsrJmSEnYbaXCiiuXsITK0jsJcDWlj32ZVOtoSODardYk7U8LQQjD6ImZD7B646G4LdWyMk+rlwrDCjbYVSBVPkA6/
x4fbg1eyJYWTBdrD2S1+xEt2DrrpJY6B9k8BtyDEVnR7sLryYPzzPdLg3ifzW8aTt9Z/0+b6S1uczdEbOyR2aowW8RNQflVUWu8NmylTxiCifubNceIdZOlIrEjj1uXcOygyqrMv/UTgEEtwvwyoiMd0DTYgb3QZbFi9hgyS1g7nQSJpdaS0FqrVyesDn8U2dv7AlQPWZMg/UEDOugkJxgEprITyermJMu94ZgqBOKf/6QmJb//ftgoCdOzQivd2VEQ/zpJxSXwHW39Jz3YOrDkgYnWAqMFvSntugrKpMKkaGcwIR4aMJsD8o8+G5TZEQc4t4pqnB3Vnl2SD3LBwYXhT5Mjqy8JAm5P6IREyG4CLzwg/UvOzOBTaMkc5FalmD74FA7/afzz3T7bgwPJMVTrCi1/+lKAo6100MQuG4LYrgoVi/JHHvlE9ekta0cAGvNXlxJolkc8uTnnzAt3tRIqb5PfSdoPcetFmlMIYOu4MhLEdsnsl8qqrBJUng/yuQlkII7OSrnh6XampiCPQx11zmDoiUy9qEu+KDN5nfgrEzHHPsDyzeMYVY/425DNBT78NjuqZj5HYyXsAfOmMPeVUdbDAW2qj1QdRVh4D/o4CzSTmXz3222zuNAqOmmmJnfoAg7RjBI/DebGWGfcG/sp7z6LOToNWB7AaM0gwFmt4dCncybK/3vKPbpAM/bTeQOD5COvzBhXC35WJbLCbzEQjZvOe46eeh/pSDr+6Hzh82bJaH4vJR/QuPXGIOjA4Vvf7x4ZjrNS4JxgimwUFAg8yPmiHEZdz/dOyfftwplPqVK0FZtjJit1xj6/3MnhzsnmacjGUFjKbxiWXj71yFPbi6w1JC34fuNmwHBr0Pi05+axphdFimGxyIoeeaoHZA+2CxE8br74Pp1susiymGayDvCS0hW1FdEn/Z4i2/5eUTxa+ZChMgKJawq1GqAgtcYfgxjO4SEdOfqp/aaaFJl5l61+RvHFMwAryfAy+V",
refresh: "REFRESH_REMOVED_SOME_FOR_EXAMPLE_FOR_SECURITYkLJXsKiUm8e0hPfV28bF0tm/ZThOH+vgMULJo/Vk/nPfw6zqYcPXtSa8VO9W8jaac/MH+LdOJPjCUB4dkwjifbUhHSZ/bNiE3Hsuwi59HVPJADyiIz04L4As/+snwiHunPDIoSBb0B5B8NpHYZnjGBptAYEwIFcfkWeWBIZCGhOrqfoVJgo0BmcyzOJIKYMusV2xLnYn4FaLU7lAh4lVrzCYNBUkBg9jyt2Yk9CqK+c7YYQtYw0lEAH0tbQUS7ekC7dHxQR5d3FPUXWCAVdgyUS56GzJhngFCdOLizVCRtkbNxDt+Z+JMRmNthn6y0EcNdTuhz/+4xTWz98kLDkXMpzmM1MB/lrr0izn7BcaV6R33e1BycL5rKNoD9jmBA9tI100MQuG4LYrgoVi/JHHvljxDHupriVwrtvZTlK4SFyYvlYfDMd+iDDn5JSoPIan5VgmoY/Xa5RBN+SCzXIRjVyWMbc93akloAPEGMn7TE5gILloHjY779w+TerVLyxBvKYg6FZGSHT42y1AmBA3thWtxq/EwkE3tIBQgApNUgQLqqQtbPwB6G3rtBKN47mIrz6qJWb+OUWvPY5mojYJ79lKtPsBqcfEEJiPznASuF5X3X4UZ5S3hJiApTW99XoFn8lc3IYlOYay/AOE2aNcJa8soJn5+HTR9dXR+cYa3BVyhKPwv4WKe+BqORTR/vVxfaWXsLtERZwOvQ4neriwE2fWZK8ZHNOCPUR+7ue6Pp+wq/SVSwbkJHd2YwgvpwZp7fL2nzZBDULOgCgaMQp4hrAYC2MqP7oVjoyFxFecpnShG75y/KprmnHE", 
clientid: "MYCLIENTIDASREGISTERED@AMER.OAUTHAP", redirecturi: None }
```

## Future Plans
I am investigating transitioning the client to async or at least having the option of async functions.  For this, the 
http request client would need to change.  When downloading lots of history, an async function would allow the requests 
to be run in parallel instead of series, saving considerable time.