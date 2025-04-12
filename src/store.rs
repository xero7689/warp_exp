use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::types::question::{Question, QuestionId};

#[derive(Clone)]
pub struct Store {
    pub questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
}

impl Store {
    fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("can't read questions.json")
    }

    pub fn new() -> Self {
        Store {
            questions: Arc::new(RwLock::new(Self::init())),
            //questions: Self::init(),
        }
    }
}
