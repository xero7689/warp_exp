use std::collections::HashMap;
use tracing::{instrument, Level};
use warp::http::StatusCode;

use crate::store::Store;
use crate::types::account::Session;
use crate::types::pagination::{extract_pagniation, Pagination};
use crate::types::question::{NewQuestion, Question};

#[instrument]
pub async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    tracing::event!(target: "warp_exp", Level::INFO, "Querying Question");
    let mut pagination = Pagination::default();

    if !params.is_empty() {
        tracing::event!(Level::INFO, pagination = true);
        pagination = extract_pagniation(params)?;
    }

    tracing::info!(pagination = ?pagination);
    let res: Vec<Question> = match store
        .get_questions(pagination.limit, pagination.offset)
        .await
    {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };
    Ok(warp::reply::json(&res))
}

pub async fn add_question(
    store: Store,
    session: Session,
    new_question: NewQuestion,
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_id = session.account_id;

    match store.add_question(new_question, account_id).await {
        Ok(question) => Ok(warp::reply::json(&question)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn update_question(
    id: i32,
    session: Session,
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_id = session.account_id;

    if store.is_question_owner(id, &account_id).await? {
        let res = match store.update_question(question, id).await {
            Ok(res) => res,
            Err(e) => return Err(warp::reject::custom(e)),
        };
        Ok(warp::reply::json(&res))
    } else {
        Err(warp::reject::custom(handle_errors::Error::Unauthorized))
    }
}

pub async fn delete_question(
    id: i32,
    session: Session,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_id = session.account_id;
    if store.is_question_owner(id, &account_id).await? {
        if let Err(e) = store.delete_question(id).await {
            return Err(warp::reject::custom(e));
        };

        Ok(warp::reply::with_status(
            format!("Question {} Deleted", id),
            StatusCode::OK,
        ))
    } else {
        Err(warp::reject::custom(handle_errors::Error::Unauthorized))
    }
}
