use warp::http::StatusCode;

use crate::store::Store;
use crate::types::account::Session;
use crate::types::answer::NewAnswer;

pub async fn add_answer(
    session: Session,
    store: Store,
    params: NewAnswer,
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_id = session.account_id;

    if let Err(e) = store.add_answer(params, account_id).await {
        return Err(warp::reject::custom(e));
    };

    Ok(warp::reply::with_status(
        "Answer added successfully",
        StatusCode::CREATED,
    ))
}
