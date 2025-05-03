use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateClientRequest {
    pub name: String,
    pub last_name: String,
    pub age: i32,
    pub phone: String,
}