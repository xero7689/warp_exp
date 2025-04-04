use std::str::FromStr;
use warp::Filter; // Bring the Filter trait to scope for using `map`
use warp_exp::question;

async fn get_questions() -> Result<impl warp::Reply, warp::Rejection> {
    let question = warp_exp::question::Question::new(
        warp_exp::question::QuestionId::from_str("1").expect("No id provided"),
        "1st Question".to_string(),
        "Hello question!".to_string(),
        Some(vec!["faq".to_string()]),
    );

    Ok(warp::reply::json(&question))
}

#[tokio::main]
async fn main() {
    let hello = warp::path("hello").map(|| format!("Hello, World!"));

    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}
