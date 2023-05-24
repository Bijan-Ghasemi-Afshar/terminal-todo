use std::env::Args;
use std::fmt::Display;
use crate::validator::valid_action::VALID_ACTIONS;

pub mod valid_action;

pub struct ToDoOperation {
    operation: String,
    arguments: Vec<String>,
}

impl ToDoOperation {
    pub fn new(mut user_input: Args) -> Result<Self, &'static str> {
        let mut operation: String = "".into();
        let mut arguments: Vec<String> = vec![];

        if user_input.len() < 2 {
            return Err("An action needs to be provided [create, list]")
        } 

        // Skipping the first arg since it the program name
        user_input.next();

        operation = match user_input.next() {
            Some(op) => op,
            None => return Err("Error parsing action"),
        };

        while let Some(arg) = user_input.next() {
            arguments.push(arg);
        }

        Ok(ToDoOperation { operation, arguments })
    }

}

impl Display for ToDoOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Operation: {}\nArguments: {:?}", self.operation, self.arguments)
    }
}
