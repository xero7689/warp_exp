use warp::http::StatusCode;

use crate::store::Store;
use crate::types::answer::{Answer, AnswerId, NewAnswer};
use handle_errors::Error;

pub async fn add_answer(
    store: Store,
    params: NewAnswer,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Err(e) = store.add_answer(params).await {
        return Err(warp::reject::custom(Error::DatabaseQueryError));
    };

    Ok(warp::reply::with_status(
        "Answer added successfully",
        StatusCode::CREATED,
    ))
}
