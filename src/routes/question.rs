use std::collections::HashMap;
use warp::http::StatusCode;

use crate::store::Store;
use crate::types::pagination::extract_pagniation;
use crate::types::question::{Question, QuestionId};
use handle_errors::Error;

pub async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
    id: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("{} Start querying questions", id);
    if let Some(n) = params.get("start") {
        println!("{:?}", n.parse::<usize>().expect("Couldn't parse start"));
    }

    if !params.is_empty() {
        let pagination = extract_pagniation(params)?;
        log::info!("{} Pagination set {:?}", id, &pagination);

        let res: Vec<Question> = store.questions.read().await.values().cloned().collect();

        // Check pagination size with length of vector
        if pagination.end > res.len() {
            return Err(Error::RangeError.into());
        }

        let res = &res[pagination.start..pagination.end];
        Ok(warp::reply::json(&res))
    } else {
        log::info!("{} Not pagination used!", id);
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
    id: String,
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
    id: String,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.questions.write().await.remove(&QuestionId(id)) {
        Some(_) => Ok(warp::reply::with_status("Question Deleted", StatusCode::OK)),
        None => Err(warp::reject::custom(Error::QuestionNotFound)),
    }
}
