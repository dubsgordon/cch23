mod day;
use axum::{http::StatusCode, routing::{get, post}, Router};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn ise() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(ise))
        .route("/1/*params", get(day::one::compute))
        .route("/4/strength", post(day::four::strength::compute))
        .route("/5", post(day::five::compute));

    Ok(router.into())
}
