use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct CompleteGroup{
    pub key: char,
    pub bites: String
}

#[derive(Serialize, Deserialize)]
pub struct Group{
    pub group: Vec<CompleteGroup>
}