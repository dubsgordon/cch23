use axum::{extract::Path, http::StatusCode, routing::get, Router};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn ise() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

async fn one(
    Path(params): Path<String>
) -> String {
    let mut value = 0;
    let num_strings = params.split('/');
    for num_string in num_strings.into_iter() {
        let num: Result<i64, _> = num_string.parse();
        if let Ok(num) = num {
            value ^= num;
        }
    }
    value.pow(3).to_string()
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(ise))
        .route("/1/*params", get(one));

    Ok(router.into())
}
