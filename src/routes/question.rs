use std::collections::HashMap;
use tracing::{info, instrument};
use warp::http::StatusCode;

use crate::store::Store;
use crate::types::pagination::extract_pagniation;
use crate::types::question::{Question, QuestionId};
use handle_errors::Error;

#[instrument]
pub async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("Start querying questions");
    if let Some(n) = params.get("start") {
        println!("{:?}", n.parse::<usize>().expect("Couldn't parse start"));
    }

    if !params.is_empty() {
        let pagination = extract_pagniation(params)?;
        info!("Pagination set {:?}", &pagination);

        let res: Vec<Question> = store.questions.read().await.values().cloned().collect();

        // Check pagination size with length of vector
        if pagination.end > res.len() {
            return Err(Error::RangeError.into());
        }

        let res = &res[pagination.start..pagination.end];
        Ok(warp::reply::json(&res))
    } else {
        info!("Not pagination used!");
        let res: Vec<Question> = store.questions.read().await.values().cloned().collect();
        Ok(warp::reply::json(&res))
    }
}

pub async fn add_question(
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    store
        .questions
        .write()
        .await
        .insert(question.id.clone(), question);

    Ok(warp::reply::with_status("Question Added", StatusCode::OK))
}

pub async fn update_question(
    id: i32,
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.questions.write().await.get_mut(&QuestionId(id)) {
        Some(q) => *q = question,
        None => return Err(warp::reject::custom(Error::QuestionNotFound)),
    }

    Ok(warp::reply::with_status(
        "Question Updated!",
        StatusCode::OK,
    ))
}

pub async fn delete_question(
    id: i32,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.questions.write().await.remove(&QuestionId(id)) {
        Some(_) => Ok(warp::reply::with_status("Question Deleted", StatusCode::OK)),
        None => Err(warp::reject::custom(Error::QuestionNotFound)),
    }
}
