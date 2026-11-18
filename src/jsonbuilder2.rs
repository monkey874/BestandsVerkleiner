use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct CompleteGroup{
    pub key: char,
    pub bytes: String

}

#[derive(Serialize, Deserialize)]
pub struct Group{
    pub group: Vec<CompleteGroup>
}