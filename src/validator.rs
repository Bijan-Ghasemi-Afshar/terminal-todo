use crate::validator::valid_action::{ValidAction, VALID_ACTIONS};
use std::env::Args;
use std::fmt::Display;

pub mod valid_action;

pub struct ToDoOperation {
    operation: ValidAction,
    arguments: Vec<String>,
}

impl ToDoOperation {
    pub fn new(mut user_input: Args) -> Result<Self, &'static str> {
        if user_input.len() < 2 {
            return Err("An action needs to be provided [create, list]");
        }

        // Skipping the first arg since it the program name
        user_input.next();

        // Get the operation
        let operation = match user_input.next() {
            Some(op) => ToDoOperation::validate_operation(op)?,
            None => return Err("Error parsing action"),
        };

        // Get the operation arguments
        let arguments = ToDoOperation::validate_arguments(user_input, &operation)?;

        Ok(ToDoOperation {
            operation,
            arguments,
        })
    }

    fn validate_operation(operation: String) -> Result<ValidAction, &'static str> {
        for action in VALID_ACTIONS {
            if action.name == operation {
                return Ok(action);
            }
        }
        Err("Operation is not valid.\n[create, list]")
    }

    fn validate_arguments<T>(
        mut user_args: T,
        action: &ValidAction,
    ) -> Result<Vec<String>, &'static str>
    where
        T: Iterator<Item = String>,
    {
        let mut arguments: Vec<String> = vec![];

        while let Some(arg) = user_args.next() {
            arguments.push(arg);
        }

        match action.requires_arguments {
            x if x && arguments.len() <= 0 => return Err("Operation requires arguments"),
            x if !x && arguments.len() > 0 => {
                eprintln!("Operation does not take arguments");
                Ok(arguments)
            }
            _ => Ok(arguments),
        }
    }
}

impl Display for ToDoOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Operation: {}\nArguments: {:?}",
            self.operation.name, self.arguments
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn validates_operation_correctly() {
        let valid_action = ValidAction {
            name: "create",
            requires_arguments: false,
        };
        assert_eq!(
            ToDoOperation::validate_operation("create".into()),
            Ok(valid_action)
        );
    }

    #[test]
    fn returns_error_if_operation_in_invalid() {
        assert_eq!(
            ToDoOperation::validate_operation("invalid".into()),
            Err("Operation is not valid.\n[create, list]")
        );
    }

    #[test]
    fn validates_arguments_correctly() {
        let action_with_args: ValidAction = ValidAction {
            name: "list",
            requires_arguments: true,
        };
        let args = vec![String::from("test"), String::from("test")];
        assert_eq!(
            ToDoOperation::validate_arguments(args.into_iter(), &action_with_args),
            Ok(vec![String::from("test"), String::from("test")])
        );
    }
}
