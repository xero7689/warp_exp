use std::fmt::{Display, Formatter};

#[derive(Debug)]
struct QuestionId(String);

#[derive(Debug)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

impl Question {
    fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Question {
        Question {
            id,
            title,
            content,
            tags,
        }
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

fn main() {
    println!("Hello, world!");
}
