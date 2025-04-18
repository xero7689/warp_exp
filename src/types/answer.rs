use crate::types::question::QuestionId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnswerId(pub i32);

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Answer {
    pub id: AnswerId,
    pub question_id: QuestionId,
    pub content: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewAnswer {
    pub content: String,
    pub question_id: QuestionId,
}
