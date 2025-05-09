use argon2::Error as ArgonError;
use std::fmt::Formatter;
use warp::reject::Reject;
use warp::{filters::body::BodyDeserializeError, http::StatusCode, Rejection, Reply}; // Bring the Filter trait to scope for using `map`

use tracing::{event, Level};

#[derive(Debug)]
pub enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    RangeError,
    QuestionNotFound,
    DatabaseQueryError(sqlx::Error),
    WrongPassword,
    ArgonLibraryError(ArgonError),
    CannotDecrptToken,
    Unauthorized,
    MigrationError(sqlx::migrate::MigrateError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::ParseError(ref err) => {
                write!(f, "Cannot parse parameter: {}", err)
            }
            Error::MissingParameters => write!(f, "Missing parameter"),
            Error::RangeError => write!(f, "Range error"),
            Error::QuestionNotFound => write!(f, "Question Not Found"),
            Error::DatabaseQueryError(_) => {
                write!(f, "Query couldn't be executed")
            }
            Error::WrongPassword => {
                write!(f, "Wrong Password")
            }
            Error::ArgonLibraryError(_) => {
                write!(f, "Can't verify password")
            }
            Error::CannotDecrptToken => {
                write!(f, "Cannot decrypt token")
            }
            Error::Unauthorized => {
                write!(f, "Request is unauthorized")
            }
            Error::MigrationError(_) => {
                write!(f, "Error when doing migration")
            }
        }
    }
}

// You must implement the Reject Trait, to match the Result of Error type which implemented Rejection
impl Reject for Error {}

const DUPLICATE_KEY: u32 = 23505;

pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(crate::Error::DatabaseQueryError(e)) = r.find() {
        event!(Level::ERROR, "Database query error");
        match e {
            sqlx::Error::Database(err) => {
                if err.code().unwrap().parse::<u32>().unwrap() == DUPLICATE_KEY {
                    Ok(warp::reply::with_status(
                        "Account already exists".to_string(),
                        StatusCode::UNPROCESSABLE_ENTITY,
                    ))
                } else {
                    Ok(warp::reply::with_status(
                        "Cannot update data".to_string(),
                        StatusCode::UNPROCESSABLE_ENTITY,
                    ))
                }
            }
            _ => Ok(warp::reply::with_status(
                "Cannot update data".to_string(),
                StatusCode::UNPROCESSABLE_ENTITY,
            )),
        }
    } else if let Some(error) = r.find::<warp::cors::CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else if let Some(error) = r.find::<BodyDeserializeError>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else if let Some(crate::Error::WrongPassword) = r.find() {
        event!(Level::ERROR, "Enter Wrong password");
        Ok(warp::reply::with_status(
            "Wrong E-Mail/Password combination".to_string(),
            StatusCode::UNAUTHORIZED,
        ))
    } else if let Some(crate::Error::Unauthorized) = r.find() {
        event!(Level::ERROR, "Not matching account id");
        Ok(warp::reply::with_status(
            "No permission to change underlying resource".to_string(),
            StatusCode::UNAUTHORIZED,
        ))
    } else {
        println! {"{:?}", r};
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}
