use std::collections::HashMap;
use tracing::{event, info, instrument, Level};
use warp::http::StatusCode;

use crate::store::Store;
use crate::types::pagination::{extract_pagniation, Pagination};
use crate::types::question::{NewQuestion, Question, QuestionId};
use handle_errors::Error;

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
        Err(_) => return Err(warp::reject::custom(Error::DatabaseQueryError)),
    };
    Ok(warp::reply::json(&res))
}

pub async fn add_question(
    store: Store,
    new_question: NewQuestion,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Err(_) = store.add_question(new_question).await {
        return Err(warp::reject::custom(Error::DatabaseQueryError));
    };

    Ok(warp::reply::with_status("Question Added", StatusCode::OK))
}

pub async fn update_question(
    id: i32,
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    let res = match store.update_question(question, id).await {
        Ok(res) => res,
        Err(_) => return Err(warp::reject::custom(Error::DatabaseQueryError)),
    };

    Ok(warp::reply::json(&res))
}

pub async fn delete_question(id: i32, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    if let Err(_) = store.delete_question(id).await {
        return Err(warp::reject::custom(Error::DatabaseQueryError));
    };

    Ok(warp::reply::with_status(
        format!("Question {} Deleted", id),
        StatusCode::OK,
    ))
}
