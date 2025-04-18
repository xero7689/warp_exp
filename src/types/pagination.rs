#![warn(clippy::all)]

use std::collections::HashMap;

use handle_errors::Error;

/// Pagniation struct that is getting extracted
/// from query params
#[derive(Default, Debug)]
pub struct Pagination {
    pub limit: Option<u32>,
    pub offset: u32,
}

/// Extract query paramters from the `/questions` route
/// # Example Query
/// Get requests to this route can have a pagniation attached so we just
/// return the question we need
/// `/questions?start=1&send=10`
pub fn extract_pagniation(params: HashMap<String, String>) -> Result<Pagination, Error> {
    if params.contains_key("limit") && params.contains_key("offset") {
        return Ok(Pagination {
            limit: Some(
                params
                    .get("limit")
                    .unwrap()
                    .parse::<u32>()
                    .map_err(Error::ParseError)?,
            ),
            offset: params
                .get("end")
                .unwrap()
                .parse::<u32>()
                .map_err(Error::ParseError)?,
        });
    }

    Err(Error::MissingParameters)
}
