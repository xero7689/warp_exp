use warp::Filter;
use warp_exp::question;

#[tokio::main]
async fn main() {
    let hello = warp::get().map(|| format!("Hello, World!"));

    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}
