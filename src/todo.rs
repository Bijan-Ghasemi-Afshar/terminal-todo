use std::fmt::Display;

#[derive(Debug)]
pub struct ToDo {
    title: String,
}

impl ToDo {
    pub fn new(title: String) -> Self {
        ToDo { title }
    }
}

impl Display for ToDo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Title: {}", self.title)
    }
}
