use std::fmt;

#[derive(Debug)]
pub enum TDAClientError {
    /// Deals with connection io issues to the web API
    WebConnectIssue(attohttpc::Error),
    /// Any issues in parsing the response
    ParseError(serde_json::Error),
}

impl std::error::Error for TDAClientError {}

impl fmt::Display for TDAClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TDAClientError::WebConnectIssue(e) => write!(f, "Web Connection Error: {}", e),
            TDAClientError::ParseError(e) => write!(f, "Response Parsing Error: {}", e),
        }
    }
}

impl From<serde_json::Error> for TDAClientError {
    fn from(e: serde_json::Error) -> Self {
        TDAClientError::ParseError(e)
    }
}

impl From<attohttpc::Error> for TDAClientError {
    fn from(e: attohttpc::Error) -> Self {
        TDAClientError::WebConnectIssue(e)
    }
}