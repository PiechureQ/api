use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Place {
    pub id: i32,
    pub name: String,
}
