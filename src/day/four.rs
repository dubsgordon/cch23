pub mod strength {
    use axum::extract;
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct Deer {
        name: String,
        strength: i64,
    }

    pub async fn compute(extract::Json(payload): extract::Json<Vec<Deer>>) -> String {
        let mut total_strength = 0;
        for d in payload {
            total_strength += d.strength;
        }
        
        total_strength.to_string()
    }
}
