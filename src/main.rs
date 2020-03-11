use warp::Filter;
use warp::http::{StatusCode, header};
//use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    //let ip = warp::header::<SocketAddr>("x-clientip");
    let notify = warp::path("notify")
            .and(warp::fs::file("static/index.html"));
    let favicon = warp::path("favicon.ico")
            .and(warp::fs::file("static/favicon.ico"));
    let files = warp::path("static")
            .and(warp::fs::dir("static"));
    let redirect = warp::any().map(|| Ok(warp::reply::with_header(
                StatusCode::FOUND,
                header::LOCATION,
                "notify")));
    let routes = warp::get().and(notify.or(favicon).or(files).or(redirect)).with(warp::log("gatodown"));

    warp::serve(routes)
        .run(([0, 0, 0, 0], 8082))
        .await;
}
