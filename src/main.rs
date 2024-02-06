mod day;
use axum::{extract::Path, http::StatusCode, routing::get, Router};

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
        .route("/1/*params", get(day::one::compute));

    Ok(router.into())
}
