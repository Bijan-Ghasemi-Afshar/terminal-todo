use crate::{log_wrapper::Logger, todo::ToDo};
use std::fmt::Display;
use std::io::{self, stdout, Write};

pub mod database;

const CREATE: &'static str = "create";
const LIST: &'static str = "list";
const EDIT: &'static str = "edit";
const DONE: &'static str = "done";
const UNDONE: &'static str = "undone";
const DELETE: &'static str = "delete";

pub enum ActionType {
    Create,
    List,
    Edit,
    Done,
    Undone,
    Delete,
}

impl ActionType {
    pub fn new(action_name: &str) -> Result<Self, &'static str> {
        match action_name {
            "create" => Ok(ActionType::Create),
            "list" => Ok(ActionType::List),
            "edit" => Ok(ActionType::Edit),
            "done" => Ok(ActionType::Done),
            "undone" => Ok(ActionType::Undone),
            "delete" => Ok(ActionType::Delete),
            _ => Err("Action is not valid"),
        }
    }
}

/* #[derive(PartialEq, Debug)] */
pub struct Action<'a> {
    pub action_type: ActionType,
    pub name: &'static str,
    pub requires_arguments: bool,
    pub arguments: Vec<String>,
    pub error_logger: Option<&'a dyn Logger>,
}

impl Action<'_> {
    pub fn execute_action(self) -> Result<(), &'static str> {
        match self.action_type {
            ActionType::Create => self.create(),
            ActionType::List => self.list(),
            ActionType::Edit => self.edit(),
            ActionType::Done => self.done(),
            ActionType::Undone => self.undone(),
            ActionType::Delete => Ok(()),
        }
    }

    fn list(self) -> Result<(), &'static str> {
        println!("Printing all ToDo items");
        let todos: Vec<ToDo> = database::read_items()?;

        todos.iter().enumerate().for_each(|(index, todo)| {
            println!(
                "===============\n# {}\n{todo}\n===============\n",
                index + 1
            )
        });

        Ok(())
    }

    fn create(self) -> Result<(), &'static str> {
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

        database::store_item(todo)?;

        Ok(())
    }

    fn edit(self) -> Result<(), &'static str> {
        let item_index: usize = self.get_item_index_arg()?;

        println!("Editing #{} ToDo item", item_index + 1);
        let mut todos: Vec<ToDo> = database::read_items()?;

        let mut edit_todo = match todos.get_mut(item_index) {
            Some(todo) => todo,
            None => return Err("Given ToDo Item index is wrong"),
        };

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

        database::store_existing_items(todos)?;

        Ok(())
    }

    fn update_todo_status(self, status: String) -> Result<(), &'static str> {
        let item_index: usize = self.get_item_index_arg()?;

        println!("Editing #{} ToDo item", item_index + 1);
        let mut todos: Vec<ToDo> = database::read_items()?;

        let mut edit_todo = match todos.get_mut(item_index) {
            Some(todo) => todo,
            None => return Err("Given ToDo Item index is wrong"),
        };

        edit_todo.done = status;

        database::store_existing_items(todos)?;

        Ok(())
    }

    fn done(self) -> Result<(), &'static str> {
        self.update_todo_status("✅".into())?;
        Ok(())
    }

    fn undone(self) -> Result<(), &'static str> {
        self.update_todo_status("❌".into())?;
        Ok(())
    }

    fn delete(self) -> Result<(), &'static str> {
        let item_index: usize = self.get_item_index_arg()?;

        let mut todos: Vec<ToDo> = database::read_items()?;

        if item_index >= todos.len() || item_index <= 0 {
            return Err("Given item index is wrong");
        }

        println!("Deleting #{} ToDo item", item_index + 1);

        todos.remove(item_index);

        database::store_existing_items(todos)?;

        Ok(())
    }

    fn get_item_index_arg(self) -> Result<usize, &'static str> {
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
            "Action: {}\nRequired Arguments: {:?}\nArguments{:?}",
            self.name, self.requires_arguments, self.arguments
        )
    }
}

pub const ACTIONS: [Action; 6] = [
    Action {
        action_type: ActionType::Create,
        name: CREATE,
        requires_arguments: false,
        arguments: vec![],
        error_logger: None,
    },
    Action {
        action_type: ActionType::List,
        name: LIST,
        requires_arguments: false,
        arguments: vec![],
        error_logger: None,
    },
    Action {
        action_type: ActionType::Edit,
        name: EDIT,
        requires_arguments: true,
        arguments: vec![],
        error_logger: None,
    },
    Action {
        action_type: ActionType::Done,
        name: DONE,
        requires_arguments: true,
        arguments: vec![],
        error_logger: None,
    },
    Action {
        action_type: ActionType::Undone,
        name: UNDONE,
        requires_arguments: true,
        arguments: vec![],
        error_logger: None,
    },
    Action {
        action_type: ActionType::Delete,
        name: DELETE,
        requires_arguments: true,
        arguments: vec![],
        error_logger: None,
    },
];
