use serde::{Deserialize, Serialize};
use crate::json_builder::{LeftBranch, RightBranch};

#[derive(Serialize, Deserialize)]
pub struct CompleteGroup{
    pub key: char,
    pub bytes: String

}

#[derive(Serialize, Deserialize)]
pub struct Group{
    pub group: Vec<CompleteGroup>
}