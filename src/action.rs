use crate::{log_wrapper::Logger, todo::ToDo};
use std::fmt::{self, Display};
use std::io::{self, stdout, Write};

use self::database::Database;

pub mod database;

#[derive(PartialEq, Debug)]
pub enum ActionType {
    Create(bool),
    List(bool),
    Edit(bool),
    Done(bool),
    Undone(bool),
    Delete(bool),
}

impl ActionType {
    pub fn new(action_name: &str) -> Result<Self, &'static str> {
        match action_name {
            "create" => Ok(ActionType::Create(false)),
            "list" => Ok(ActionType::List(false)),
            "edit" => Ok(ActionType::Edit(true)),
            "done" => Ok(ActionType::Done(true)),
            "undone" => Ok(ActionType::Undone(true)),
            "delete" => Ok(ActionType::Delete(true)),
            _ => Err("Action is not valid"),
        }
    }

    pub fn requires_arguments(&self) -> bool {
        match self {
            ActionType::Create(req_args) => *req_args,
            ActionType::List(req_args) => *req_args,
            ActionType::Edit(req_args) => *req_args,
            ActionType::Done(req_args) => *req_args,
            ActionType::Undone(req_args) => *req_args,
            ActionType::Delete(req_args) => *req_args,
        }
    }
}

pub struct Action<'a> {
    pub action_type: ActionType,
    pub requires_arguments: bool,
    pub arguments: Vec<String>,
    pub logger: Option<&'a mut dyn Logger>,
    pub database: Option<&'a dyn Database>,
}

impl<'a> fmt::Debug for Action<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Action")
            .field("logger", &"Logging object")
            .finish()
    }
}

impl<'a> PartialEq for Action<'a> {
    fn eq(&self, other: &Self) -> bool {
        let self_type = self.logger.as_ref().unwrap().as_any().type_id();
        let other_type = other.logger.as_ref().unwrap().as_any().type_id();

        self.action_type == other.action_type
            && self.requires_arguments == other.requires_arguments
            && self.arguments == other.arguments
            && self_type == other_type
    }
}

impl<'a> Action<'a> {
    pub fn new(
        action_type: &str,
        logger: &'a mut dyn Logger,
        database: &'a dyn Database,
    ) -> Result<Self, &'static str> {
        let act_type = ActionType::new(action_type)?;
        let req_args = act_type.requires_arguments();

        Ok(Action {
            action_type: act_type,
            requires_arguments: req_args,
            arguments: vec![],
            logger: Some(logger),
            database: Some(database),
        })
    }

    pub fn execute_action(&mut self) -> Result<(), &'static str> {
        match self.action_type {
            ActionType::Create(_) => self.create(),
            ActionType::List(_) => self.list(),
            ActionType::Edit(_) => self.edit(),
            ActionType::Done(_) => self.done(),
            ActionType::Undone(_) => self.undone(),
            ActionType::Delete(_) => self.delete(),
        }
    }

    fn list(&mut self) -> Result<(), &'static str> {
        self.logger
            .as_mut()
            .unwrap()
            .log_stdln(&"Printing all ToDo items.")
            .unwrap();

        let todos: Vec<ToDo> = self
            .database
            .expect("Database could not be found")
            .read_items()?;

        todos.iter().enumerate().for_each(|(index, todo)| {
            self.logger
                .as_mut()
                .unwrap()
                .log_stdln(&format!(
                    "===============\n# {}\n{todo}\n===============\n",
                    index + 1
                ))
                .unwrap();
        });

        Ok(())
    }

    fn create(&mut self) -> Result<(), &'static str> {
        self.logger
            .as_mut()
            .unwrap()
            .log_stdln(&"Creating a ToDo item")
            .unwrap();

        self.logger.as_mut().unwrap().log_std(&"Title: ").unwrap();
        stdout().flush().unwrap();
        let mut title = String::new();
        io::stdin().read_line(&mut title).unwrap();

        self.logger
            .as_mut()
            .unwrap()
            .log_std(&"Description: ")
            .unwrap();
        stdout().flush().unwrap();
        let mut description = String::new();
        io::stdin().read_line(&mut description).unwrap();

        let todo: ToDo = ToDo::new(title, description);

        self.database
            .expect("Database could not be found")
            .store_item(todo)?;

        Ok(())
    }

    fn edit(&mut self) -> Result<(), &'static str> {
        let item_index: usize = self.get_item_index_arg()?;

        let mut todos: Vec<ToDo> = self
            .database
            .expect("Database could not be found")
            .read_items()?;

        let mut edit_todo = match todos.get_mut(item_index) {
            Some(todo) => todo,
            None => return Err("Given ToDo Item index is wrong"),
        };

        self.logger
            .as_mut()
            .unwrap()
            .log_stdln(&format!("Editing #{} ToDo item", item_index + 1))
            .unwrap();

        self.logger
            .as_mut()
            .unwrap()
            .log_stdln(&format!("{}", edit_todo))
            .unwrap();

        self.logger
            .as_mut()
            .unwrap()
            .log_std(&"New Title: ")
            .unwrap();

        stdout().flush().unwrap();
        let mut new_title = String::new();
        io::stdin().read_line(&mut new_title).unwrap();

        self.logger
            .as_mut()
            .unwrap()
            .log_std(&"New Description: ")
            .unwrap();
        stdout().flush().unwrap();
        let mut new_description = String::new();
        io::stdin().read_line(&mut new_description).unwrap();

        if new_title.len() > 1 {
            edit_todo.title = new_title;
        }

        if new_description.len() > 1 {
            edit_todo.description = new_description;
        }

        self.database
            .expect("Database could not be found")
            .store_existing_items(todos)?;

        Ok(())
    }

    fn update_todo_status(&mut self, status: String) -> Result<(), &'static str> {
        let item_index: usize = self.get_item_index_arg()?;

        self.logger
            .as_mut()
            .unwrap()
            .log_stdln(&format!("Editing #{} ToDo item", item_index + 1))
            .unwrap();

        let mut todos: Vec<ToDo> = self
            .database
            .expect("Database could not be found")
            .read_items()?;

        let mut edit_todo = match todos.get_mut(item_index) {
            Some(todo) => todo,
            None => return Err("Given ToDo Item index is wrong"),
        };

        edit_todo.done = status;

        self.database
            .expect("Database could not be found")
            .store_existing_items(todos)?;

        Ok(())
    }

    fn done(&mut self) -> Result<(), &'static str> {
        self.update_todo_status("✅".into())?;
        Ok(())
    }

    fn undone(&mut self) -> Result<(), &'static str> {
        self.update_todo_status("❌".into())?;
        Ok(())
    }

    fn delete(&mut self) -> Result<(), &'static str> {
        let item_index: usize = self.get_item_index_arg()?;

        let mut todos: Vec<ToDo> = self
            .database
            .expect("Database could not be found")
            .read_items()?;

        if item_index >= todos.len() {
            return Err("Given item index is wrong");
        }

        self.logger
            .as_mut()
            .unwrap()
            .log_stdln(&format!("Deleting #{} ToDo item", item_index + 1))
            .unwrap();

        todos.remove(item_index);

        self.database
            .expect("Database could not be found")
            .store_existing_items(todos)?;

        Ok(())
    }

    fn get_item_index_arg(&self) -> Result<usize, &'static str> {
        match self.arguments.get(0) {
            Some(arg) => match arg.parse::<usize>() {
                Ok(index) => {
                    if index == 0 {
                        return Err("Given argument should be a positive number");
                    }
                    return Ok(index - 1);
                }
                Err(_) => return Err("Given argument should be a positive number"),
            },
            None => return Err("Could not get passed argument"),
        };
    }
}

impl Display for Action<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Action:\nRequired Arguments: {:?}\nArguments{:?}",
            self.requires_arguments, self.arguments
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::log_wrapper::{LogWrapper, Logger};
    use std::{
        error::Error,
        io::{ErrorKind, Stderr, Stdout},
    };

    #[test]
    fn can_create_new_action_type() {
        let action_type: ActionType = ActionType::new(&"create").unwrap();
        assert_eq!(action_type, ActionType::Create(false));
    }

    #[test]
    fn should_return_error_if_action_type_is_wrong() {
        let action_type: Result<ActionType, &'static str> = ActionType::new(&"wrong");
        assert_eq!(action_type, Err("Action is not valid"));
    }

    #[test]
    fn should_return_correct_value_for_required_args() {
        let action_type = ActionType::new(&"create").unwrap();
        assert_eq!(action_type.requires_arguments(), false);

        let action_type = ActionType::new(&"list").unwrap();
        assert_eq!(action_type.requires_arguments(), false);

        let action_type = ActionType::new(&"edit").unwrap();
        assert_eq!(action_type.requires_arguments(), true);

        let action_type = ActionType::new(&"done").unwrap();
        assert_eq!(action_type.requires_arguments(), true);

        let action_type = ActionType::new(&"undone").unwrap();
        assert_eq!(action_type.requires_arguments(), true);

        let action_type = ActionType::new(&"delete").unwrap();
        assert_eq!(action_type.requires_arguments(), true);
    }

    #[derive(PartialEq, Clone)]
    struct MockErrorLogger {
        was_called: bool,
    }

    impl Logger for MockErrorLogger {
        fn log_errln<'a>(&mut self, msg: &'a str) -> Result<(), Box<dyn std::error::Error>> {
            if msg == "" {
                return Err(Box::new(io::Error::new(ErrorKind::Other, "oh no!")));
            }
            self.was_called = true;
            Ok(())
        }
        fn log_stdln<'a>(&mut self, _msg: &'a str) -> Result<(), Box<dyn Error>> {
            Ok(())
        }
        fn log_err<'a>(&mut self, _msg: &'a str) -> Result<(), Box<dyn Error>> {
            Ok(())
        }
        fn log_std<'a>(&mut self, _msg: &'a str) -> Result<(), Box<dyn Error>> {
            Ok(())
        }

        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    struct MockDatabase {}

    impl Database for MockDatabase {
        fn read_items(&self) -> Result<Vec<ToDo>, &'static str> {
            let todo = ToDo::new("title".into(), "description".into());
            Ok(vec![todo])
        }

        fn store_existing_items(&self, _: Vec<ToDo>) -> Result<(), &'static str> {
            Ok(())
        }

        fn store_item(&self, _: ToDo) -> Result<(), &'static str> {
            todo!()
        }
    }

    #[test]
    fn should_make_action_properly() {
        let mut mock_logger = MockErrorLogger { was_called: false };
        let mut other_mock_logger = MockErrorLogger { was_called: false };
        let mut log_wrapper: LogWrapper<Stderr, Stdout> =
            LogWrapper::new(io::stderr(), io::stdout());
        let database = MockDatabase {};

        let action = Action::new(&"create", &mut mock_logger, &database);

        let expected = Action {
            action_type: ActionType::Create(false),
            requires_arguments: false,
            arguments: vec![],
            logger: Some(&mut other_mock_logger),
            database: None,
        };
        assert_eq!(action, Ok(expected));

        let expected = Action {
            action_type: ActionType::Create(false),
            requires_arguments: false,
            arguments: vec![],
            logger: Some(&mut log_wrapper),
            database: None,
        };
        assert_ne!(action, Ok(expected));
    }

    #[test]
    fn should_list_todo_properly() {
        let mock_logger_err: Vec<u8> = Vec::<u8>::new();
        let mock_logger_std: Vec<u8> = Vec::new();
        let mut logger = LogWrapper::new(mock_logger_err, mock_logger_std);
        let database = MockDatabase {};
        let mut list_action = Action::new("list".into(), &mut logger, &database).unwrap();
        let res = list_action.execute_action();
        assert_eq!(res, Ok(()));
        let logs = "Printing all ToDo items.
===============
# 1
Title: title
Description: description
Done: ❌
===============\n\n";

        assert_eq!(logger.std_writer, logs.as_bytes());
    }

    #[test]
    fn should_mark_todo_as_done() {
        let mock_logger_err: Vec<u8> = Vec::<u8>::new();
        let mock_logger_std: Vec<u8> = Vec::new();
        let mut logger = LogWrapper::new(mock_logger_err, mock_logger_std);
        let database = MockDatabase {};
        let mut list_action = Action::new("done".into(), &mut logger, &database).unwrap();
        list_action.arguments = vec!["1".into()];
        let res = list_action.execute_action();
        assert_eq!(res, Ok(()));
        let logs = "Editing #1 ToDo item\n";
        assert_eq!(logger.std_writer, logs.as_bytes());
    }

    #[test]
    fn should_mark_todo_as_undone() {
        let mock_logger_err: Vec<u8> = Vec::<u8>::new();
        let mock_logger_std: Vec<u8> = Vec::new();
        let mut logger = LogWrapper::new(mock_logger_err, mock_logger_std);
        let database = MockDatabase {};
        let mut list_action = Action::new("undone".into(), &mut logger, &database).unwrap();
        list_action.arguments = vec!["1".into()];
        let res = list_action.execute_action();
        assert_eq!(res, Ok(()));
        let logs = "Editing #1 ToDo item\n";
        assert_eq!(logger.std_writer, logs.as_bytes());
    }

    #[test]
    fn should_delete_todo_item_successfully() {
        let mock_logger_err: Vec<u8> = Vec::<u8>::new();
        let mock_logger_std: Vec<u8> = Vec::new();
        let mut logger = LogWrapper::new(mock_logger_err, mock_logger_std);
        let database = MockDatabase {};
        let mut list_action = Action::new("delete".into(), &mut logger, &database).unwrap();
        list_action.arguments = vec!["1".into()];
        let res = list_action.execute_action();
        assert_eq!(res, Ok(()));
        let logs = "Deleting #1 ToDo item\n";
        assert_eq!(logger.std_writer, logs.as_bytes());
    }

    #[test]
    fn should_throw_error_if_negative_index_for_delete_is_given() {
        let mock_logger_err: Vec<u8> = Vec::<u8>::new();
        let mock_logger_std: Vec<u8> = Vec::new();
        let mut logger = LogWrapper::new(mock_logger_err, mock_logger_std);
        let database = MockDatabase {};
        let mut list_action = Action::new("delete".into(), &mut logger, &database).unwrap();
        list_action.arguments = vec!["-1".into()];
        let res = list_action.execute_action();
        assert_eq!(res, Err("Given argument should be a positive number"));
    }

    #[test]
    fn should_throw_error_if_non_existing_index_for_delete_is_given() {
        let mock_logger_err: Vec<u8> = Vec::<u8>::new();
        let mock_logger_std: Vec<u8> = Vec::new();
        let mut logger = LogWrapper::new(mock_logger_err, mock_logger_std);
        let database = MockDatabase {};
        let mut list_action = Action::new("delete".into(), &mut logger, &database).unwrap();
        list_action.arguments = vec!["100".into()];
        let res = list_action.execute_action();
        assert_eq!(res, Err("Given item index is wrong"));
    }
}
