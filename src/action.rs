use crate::{log_wrapper::Logger, todo::ToDo};
use std::fmt::Display;
use std::io::{self, stdout, Write};

pub mod database;

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

/* #[derive(PartialEq, Debug)] */
pub struct Action<'a> {
    pub action_type: ActionType,
    pub requires_arguments: bool,
    pub arguments: Vec<String>,
    pub logger: Option<&'a mut dyn Logger>,
}

impl<'a> Action<'a> {
    pub fn new(action_type: &str, logger: &'a mut dyn Logger) -> Result<Self, &'static str> {
        let act_type = ActionType::new(action_type)?;
        let req_args = act_type.requires_arguments();

        Ok(Action {
            action_type: act_type,
            requires_arguments: req_args,
            arguments: vec![],
            logger: Some(logger),
        })
    }

    pub fn execute_action(self) -> Result<(), &'static str> {
        match self.action_type {
            ActionType::Create(_) => self.create(),
            ActionType::List(_) => self.list(),
            ActionType::Edit(_) => self.edit(),
            ActionType::Done(_) => self.done(),
            ActionType::Undone(_) => self.undone(),
            ActionType::Delete(_) => self.delete(),
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
            "Action:\nRequired Arguments: {:?}\nArguments{:?}",
            self.requires_arguments, self.arguments
        )
    }
}
