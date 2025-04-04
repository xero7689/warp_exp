use std::str::FromStr;
use warp::{http::StatusCode, reject::Reject, Filter, Rejection, Reply}; // Bring the Filter trait to scope for using `map`
use warp_exp::question;

#[derive(Debug)]
struct InvalidId;
impl Reject for InvalidId {}

async fn get_questions() -> Result<impl warp::Reply, warp::Rejection> {
    let question = warp_exp::question::Question::new(
        warp_exp::question::QuestionId::from_str("1").expect("No id provided"),
        "1st Question".to_string(),
        "Hello question!".to_string(),
        Some(vec!["faq".to_string()]),
    );

    match question.id.0.parse::<i32>() {
        Err(_) => Err(warp::reject::custom(InvalidId)),
        Ok(_) => Ok(warp::reply::json(&question)),
    }
}

async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(_InvalidId) = r.find::<i32>() {
        Ok(warp::reply::with_status(
            "No Valid Id presented",
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found",
            StatusCode::NOT_FOUND,
        ))
    }
}

#[tokio::main]
async fn main() {
    //let hello = warp::path("hello").map(|| format!("Hello, World!"));
    //
    let get_items = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and_then(get_questions)
        .recover(return_error);

    let routes = get_items;

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
