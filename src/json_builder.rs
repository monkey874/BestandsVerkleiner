use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct LeftBranch{
    pub binary: i32,
    pub value: i32,
    pub key: String,
    pub has_relation: bool
}
#[derive(Serialize, Deserialize)]
pub struct RightBranch{
    pub binary: i32,
    pub value: i32,
    pub key: String,
    pub has_relation: bool

}
#[derive(Serialize, Deserialize)]
pub struct CompleteBranch{
    pub value: i32,
    pub right_branch: Vec<RightBranch>,
    pub left_branch: Vec<LeftBranch>
}