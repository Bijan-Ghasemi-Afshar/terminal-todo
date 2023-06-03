use std::fmt::Display;

#[derive(Debug)]
pub struct ToDo {
    title: String,
    description: String,
}

impl ToDo {
    pub fn new(title: String, description: String) -> Self {
        ToDo { title, description }
    }
}

impl Display for ToDo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Title: {}, Description: {}", self.title.replace('\n', ""), self.description.replace('\n', ""))
    }
}
