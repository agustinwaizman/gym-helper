use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResult {
    pub token: String,
    pub refresh: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RefreshResult{
    pub token: String,
}