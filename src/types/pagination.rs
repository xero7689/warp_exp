#![warn(clippy::all)]

use std::collections::HashMap;

use handle_errors::Error;

/// Pagniation struct that is getting extracted
/// from query params
#[derive(Debug)]
pub struct Pagniation {
    pub start: usize,
    pub end: usize,
}

/// Extract query paramters from the `/questions` route
/// # Example Query
/// Get requests to this route can have a pagniation attached so we just
/// return the question we need
/// `/questions?start=1&send=10`
pub fn extract_pagniation(params: HashMap<String, String>) -> Result<Pagniation, Error> {
    if params.contains_key("start") && params.contains_key("end") {
        return Ok(Pagniation {
            start: params
                .get("start")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseError)?,
            end: params
                .get("end")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseError)?,
        });
    }

    Err(Error::MissingParameters)
}
