use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Entry{
    pub key: String,
    pub value: String,
}

#[derive(Serialize, Deserialize)]
pub struct Query{
    pub key: String,
}