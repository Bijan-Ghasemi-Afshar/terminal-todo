use std::env::Args;
use std::fmt::Display;

// Valid Actions
const CREATE_ACTION: &str = "create";

pub struct ToDoOperation {
    action: String,
    arguments: Vec<String>,
}

impl ToDoOperation {
    pub fn new(mut user_input: Args) -> Result<Self, &'static str> {
        let mut action: String = "".into();
        let mut arguments: Vec<String> = vec![];

        if user_input.len() < 2 {
            return Err("An action needs to be provided [create, list]")
        } 

        // Skipping the first arg since it the program name
        user_input.next();

        action = match user_input.next() {
            Some(act) => act,
            None => return Err("Error parsing action"),
        };

        while let Some(arg) = user_input.next() {
            arguments.push(arg);
        }

        Ok(ToDoOperation { action, arguments })
    }
}

impl Display for ToDoOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Action: {}\nArguments: {:?}", self.action, self.arguments)
    }
}
