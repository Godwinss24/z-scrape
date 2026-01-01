#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize,Serialize)]
pub struct ZResponse {
    pub response: Vec<Category>,
}
#[derive(Debug, Deserialize,Serialize)]
pub struct Category {
    pub id: String,
    pub title: String,
    pub totalQuestCount: u64,
    pub claimedQuestCount: u64,
    pub quests: Vec<Quest>,
}
#[derive(Debug, Deserialize,Serialize)]
pub struct Quest {
    pub id: String,
    pub tasks: Option<Vec<Task>>,
    pub canRetry: bool,
    pub claimed: bool,
    pub completed: bool,
    pub inReview: bool,
    pub locked: bool,
    pub opened: bool,
}

#[derive(Debug, Deserialize,Serialize)]
pub struct Task {
    pub r#type: String,
}

