use crate::error_logger::Logger;
use crate::todo::ToDo;
use std::env::Args;
use std::fmt::Display;
use std::io::{self, stdout, Write};

pub mod database;

const CREATE: &'static str = "create";
const LIST: &'static str = "list";
const EDIT: &'static str = "edit";
const DONE: &'static str = "done";
const UNDONE: &'static str = "undone";

#[derive(PartialEq, Debug)]
pub struct Action {
    pub name: &'static str,
    pub requires_arguments: bool,
    pub arguments: Vec<String>,
    pub operation: fn(Vec<String>) -> (),
}

fn list_operation(_: Vec<String>) {
    println!("Printing all ToDo items");
    let todos: Vec<ToDo> = database::read_items();

    todos.iter().enumerate().for_each(|(index, todo)| {
        println!(
            "===============\n# {}\n{todo}\n===============\n",
            index + 1
        )
    });
}

fn create_operation(_: Vec<String>) {
    println!("Creating a ToDo item");

    print!("Title: ");
    stdout().flush().unwrap();
    let mut title = String::new();
    io::stdin().read_line(&mut title).unwrap();

    print!("Description: ");
    stdout().flush().unwrap();
    let mut description = String::new();
    io::stdin().read_line(&mut description).unwrap();

    let todo: ToDo = ToDo::new(title, description);

    database::store_item(todo);
}

fn edit_operation(args: Vec<String>) {
    let item_index: usize = args
        .get(0)
        .unwrap()
        .parse::<usize>()
        .expect("Given ToDo Item index is wrong")
        - 1;

    println!("Editing #{} ToDo item", item_index + 1);
    let mut todos: Vec<ToDo> = database::read_items();

    let mut edit_todo = todos
        .get_mut(item_index)
        .expect("Given ToDo Item index is wrong");

    println!("{edit_todo}");

    print!("New Title: ");
    stdout().flush().unwrap();
    let mut new_title = String::new();
    io::stdin().read_line(&mut new_title).unwrap();

    print!("New Description: ");
    stdout().flush().unwrap();
    let mut new_description = String::new();
    io::stdin().read_line(&mut new_description).unwrap();

    if new_title.len() > 1 {
       edit_todo.title = new_title;
    }

    if new_description.len() > 1 {
        edit_todo.description = new_description;
    }

    database::store_existing_items(todos);

    println!("Updated the database");
}

fn update_todo_status(args: Vec<String>, status: String) {
    let item_index: usize = args
        .get(0)
        .unwrap()
        .parse::<usize>()
        .expect("Given ToDo Item index is wrong")
        - 1;
    println!("Editing #{} ToDo item", item_index + 1);
    let mut todos: Vec<ToDo> = database::read_items();

    let mut edit_todo = todos
        .get_mut(item_index)
        .expect("Given ToDo Item index is wrong");


    edit_todo.done = status;
    
    database::store_existing_items(todos);
}

fn done_operation(args: Vec<String>) {
    update_todo_status(args, "✅".into())
}

fn undone_operation(args: Vec<String>) {
    update_todo_status(args, "❌".into())
}

pub const ACTIONS: [Action; 5] = [
    Action {
        name: CREATE,
        requires_arguments: false,
        arguments: vec![],
        operation: create_operation,
    },
    Action {
        name: LIST,
        requires_arguments: false,
        arguments: vec![],
        operation: list_operation,
    },
    Action {
        name: EDIT,
        requires_arguments: true,
        arguments: vec![],
        operation: edit_operation,
    },
    Action {
        name: DONE,
        requires_arguments: true,
        arguments: vec![],
        operation: done_operation,
    },
    Action {
        name: UNDONE,
        requires_arguments: true,
        arguments: vec![],
        operation: undone_operation,
    },
];

impl Action {
    pub fn validate_input<L: Logger>(
        mut user_input: Args,
        mut logger: L,
    ) -> Result<Self, &'static str> {
        if user_input.len() < 2 {
            return Err("An action needs to be provided [create, list]");
        }

        // Skipping the first arg since it the program name
        user_input.next();

        // Get the operation
        let valid_action: Action = match user_input.next() {
            Some(op) => Action::validate_operation(op)?,
            None => return Err("Error parsing action"),
        };

        // Get the operation arguments
        let valid_action = Action::validate_arguments(user_input, valid_action, &mut logger)?;

        Ok(valid_action)
    }

    fn validate_operation(operation: String) -> Result<Action, &'static str> {
        for action in ACTIONS {
            if action.name == operation {
                return Ok(action);
            }
        }
        Err("Operation is not valid.\n[create, list]")
    }

    fn validate_arguments<'a, T, L: Logger>(
        mut user_args: T,
        valid_action: Action,
        err_logger: &mut L,
    ) -> Result<Action, &'static str>
    where
        T: Iterator<Item = String>,
    {
        let mut arguments: Vec<String> = vec![];

        while let Some(arg) = user_args.next() {
            arguments.push(arg);
        }

        match valid_action.requires_arguments {
            x if x && arguments.len() <= 0 => return Err("Operation requires arguments"),
            x if !x && arguments.len() > 0 => {
                err_logger
                    .log(&"Operation does not take arguments")
                    .unwrap();
                Ok(Action {
                    arguments,
                    ..valid_action
                })
            }
            _ => Ok(Action {
                arguments,
                ..valid_action
            }),
        }
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Operation: {}\nRequired Arguments: {:?}\nArguments{:?}",
            self.name, self.requires_arguments, self.arguments
        )
    }
}

#[cfg(test)]
mod tests {
    use std::io::{self, ErrorKind};

    use super::*;
    use crate::error_logger::ErrorLogger;

    fn empty_func(_: Vec<String>) {}

    #[test]
    fn validates_operation_correctly() {
        let valid_action = Action::validate_operation("create".into());
        assert!(valid_action.is_ok());
    }

    #[test]
    fn returns_error_if_operation_in_invalid() {
        assert_eq!(
            Action::validate_operation("invalid".into()),
            Err("Operation is not valid.\n[create, list]")
        );
    }

    #[test]
    fn validates_arguments_correctly() {
        let action_with_args: Action = Action {
            name: "list",
            requires_arguments: true,
            arguments: vec![],
            operation: empty_func,
        };

        let mut err_logger = ErrorLogger::new(Box::new(vec![]));

        let args = vec![String::from("test"), String::from("test")];

        let valid_action =
            Action::validate_arguments(args.into_iter(), action_with_args, &mut err_logger);

        assert!(valid_action.is_ok());
    }

    #[test]
    fn validates_arguments_if_not_passed_required_arguments() {
        let action_with_no_required_args: Action = Action {
            name: "list",
            requires_arguments: true,
            arguments: vec![],
            operation: empty_func,
        };

        let mut err_logger = ErrorLogger::new(Box::new(vec![]));

        assert_eq!(
            Action::validate_arguments(
                [].into_iter(),
                action_with_no_required_args,
                &mut err_logger
            ),
            Err("Operation requires arguments"),
        );
    }

    struct MockErrorLogger {
        was_called: bool,
    }

    impl Logger for MockErrorLogger {
        fn log<'a>(&mut self, msg: &'a str) -> Result<(), Box<dyn std::error::Error>> {
            if msg == "" {
                return Err(Box::new(io::Error::new(ErrorKind::Other, "oh no!")));
            }
            self.was_called = true;
            Ok(())
        }
    }

    #[test]
    fn prints_warning_if_arguments_are_not_required() {
        let action_with_no_required_args: Action = Action {
            name: "create",
            requires_arguments: false,
            arguments: vec![],
            operation: empty_func,
        };
        let args = vec![String::from("test"), String::from("test")];

        let mut mock_logger = MockErrorLogger { was_called: false };

        let valid_action = Action::validate_arguments(
            args.into_iter(),
            action_with_no_required_args,
            &mut mock_logger,
        );

        assert!(valid_action.is_ok());

        assert!(mock_logger.was_called);
    }
}
