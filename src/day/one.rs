use axum::extract::Path;

pub async fn compute(
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