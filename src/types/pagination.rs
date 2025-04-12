use std::collections::HashMap;

use handle_errors::Error;

#[derive(Debug)]
pub struct Pagniation {
    pub start: usize,
    pub end: usize,
}

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
