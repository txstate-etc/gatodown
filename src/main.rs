#[macro_use] extern crate lazy_static;
use std::env;
use warp::Filter;
use warp::http::{StatusCode, header};
//use std::net::SocketAddr;

// BASE_URL_PATH is the base path the load balancers use for health checks
// which differs between edit and public gato instances. The default is "/"
// which edit instances use. The public instances use a /mj.../ path.
lazy_static! {
    static ref BASE_URL_PATH: Option<String> = match env::var("BASE_URL_PATH") {
        Ok(base_url_path) => Some(base_url_path.into()),
        Err(_) => None,
    };
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    //let ip = warp::header::<SocketAddr>("x-clientip");
    // All non base /mj.../ traffic gets 502 server failure to force webaches to serve their content
    //let path: PathBuf = "/var/www".into();

    let redirect = warp::any().map(|| Ok(warp::reply::with_header(
            StatusCode::FOUND,
            header::LOCATION,
            "notify")));
    match &*BASE_URL_PATH {
    Some(base) => { // public
        let root = warp::path(base).and(redirect);
        let routes = warp::get().and(root.or(warp::any().map(|| Ok(StatusCode::BAD_GATEWAY))));
        warp::serve(routes.with(warp::log("gatodown")))
            .run(([0, 0, 0, 0], 8080))
            .await;
        },
    None => { // edit
        // notify "xml/html" ico is with static files in "static/iso" directory
        let notify = warp::path("notify").and(warp::fs::file("static/index.html"));
        // favicon.ico "image/x-icon" ico is with static files in "static/" directory
        let favicon = warp::path("favicon.ico").and(warp::fs::file("static/favicon.ico"));
        let files = warp::path("static").and(warp::fs::dir("static"));
        // All requests for notify, favicon and static files get served
        let routes = warp::get().and(
            notify
            .or(favicon)
            .or(files)
            .or(redirect));
        warp::serve(routes.with(warp::log("gatodown")))
            .run(([0, 0, 0, 0], 8080))
            .await;
        },
    }
}
