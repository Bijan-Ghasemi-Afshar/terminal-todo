use std::fmt::Display;

#[derive(Debug, PartialEq)]
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

    pub fn deserialise(todo_str: &str) -> Result<Self, &'static str> {
        let props: Vec<&str> = todo_str.split(',').collect();

        if props.len() != 3 {
            return Err("Database is corrupted, could not read data");
        }

        Ok(ToDo {
            title: props.get(0).unwrap().to_string(),
            description: props.get(1).unwrap().to_string(),
            done: props.get(2).unwrap().to_string(),
        })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_make_a_new_todo_properly() {
        let expected: ToDo = ToDo {
            title: "title".into(),
            description: "description".into(),
            done: "❌".into(),
        };
        let todo: ToDo = ToDo::new("title".into(), "description".into());
        assert_eq!(todo, expected);
    }

    #[test]
    fn should_serialise_properly() {
        let todo: ToDo = ToDo::new("title".into(), "description".into());

        let serialised_todo: String = todo.serialise();
        let expected: String = "title,description,❌\n".into();
        assert_eq!(serialised_todo, expected);
    }

    #[test]
    fn should_deserialise_properly() {
        let serialised_todo: &str = "title,description,✅";
        let deserialised_todo = ToDo::deserialise(serialised_todo).unwrap();
        let expected_todo: ToDo = ToDo {
            title: "title".into(),
            description: "description".into(),
            done: "✅".into(),
        };
        assert_eq!(expected_todo, deserialised_todo,);
    }

    #[test]
    fn should_handle_error_peroperly_when_deserialising() {
        let corrupted_serialised_todo: &str = "fsjl,dsj";
        let deserialised_todo = ToDo::deserialise(corrupted_serialised_todo);
        if let Err(err) = deserialised_todo {
            let expected_todo = "Database is corrupted, could not read data";
            assert_eq!(expected_todo, err);
        } else {
            assert!(false);
        }
    }
}
