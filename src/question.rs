use std::fmt::{Display, Formatter};
use std::io::{Error, ErrorKind};
use std::str::FromStr;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct QuestionId(pub String);

#[derive(Debug, Serialize)]
pub struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

impl Question {
    pub fn new(
        id: QuestionId,
        title: String,
        content: String,
        tags: Option<Vec<String>>,
    ) -> Question {
        Question {
            id,
            title,
            content,
            tags,
        }
    }

    fn update_title(&mut self, new_title: String) -> Self {
        Question::new(
            self.id.clone(),
            new_title,
            self.content.clone(),
            self.tags.clone(),
        )
    }
}

impl Display for Question {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "[{:?}]\ttitle: {}, content: {}, tags: {:?}",
            self.id, self.title, self.content, self.tags
        )
    }
}

impl FromStr for QuestionId {
    type Err = std::io::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(Error::new(ErrorKind::InvalidInput, "No id provided")),
        }
    }
}
