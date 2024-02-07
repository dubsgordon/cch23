use std::{fmt, str::FromStr};

use axum::{extract::{self, Query}, Json};
use serde::{de, Deserialize, Deserializer};


#[derive(Deserialize)]
pub struct Params {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    offset: Option<usize>,
    limit: Option<usize>,
    split: Option<usize>, // todo
}

/// Serde deserialization to map empty Strings to None,
fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}


pub async fn compute(
    params: Query<Params>,
    extract::Json(names): extract::Json<Vec<String>>
) -> Json<Vec<String>> 
{
    let start = params.offset.unwrap_or(0);
    let end = params.limit.unwrap_or(names.len() - start);
    Json(names[start..(start + end)].to_vec())
}