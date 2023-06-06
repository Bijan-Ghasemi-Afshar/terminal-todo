use std::fmt::Display;

#[derive(Debug)]
pub struct ToDo {
    pub title: String,
    pub description: String,
    pub done: String,
}

impl ToDo {
    pub fn new(title: String, description: String) -> Self {
        ToDo {
            title,
            description,
            done: "❌".into(),
        }
    }

    pub fn deserialise(todo_str: &str) -> Self {
        let props: Vec<&str> = todo_str.split(',').collect();

        ToDo {
            title: props.get(0).unwrap().to_string(),
            description: props.get(1).unwrap().to_string(),
            done: props.get(2).unwrap().to_string(),
        }
    }

    pub fn serialise(&self) -> String {
        format!(
            "{},{},{}\n",
            self.title.replace('\n', ""),
            self.description.replace('\n', ""),
            self.done
        )
    }
}

impl Display for ToDo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Title: {}\nDescription: {}\nDone: {}",
            self.title.replace('\n', ""),
            self.description.replace('\n', ""),
            self.done,
        )
    }
}
