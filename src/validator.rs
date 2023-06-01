use crate::error_logger::Logger;
use crate::validator::valid_action::{ValidAction, VALID_ACTIONS};
use std::env::Args;
use std::fmt::Display;

pub mod valid_action;

pub struct ToDoOperation<L>
where
    L: Logger,
{
    operation: ValidAction,
    arguments: Vec<String>,
    err_lgr: L,
}

impl<L> ToDoOperation<L>
where
    L: Logger,
{
    pub fn new(mut user_input: Args, mut logger: L) -> Result<Self, &'static str> {
        if user_input.len() < 2 {
            return Err("An action needs to be provided [create, list]");
        }

        // Skipping the first arg since it the program name
        user_input.next();

        // Get the operation
        let operation = match user_input.next() {
            Some(op) => ToDoOperation::<L>::validate_operation(op)?,
            None => return Err("Error parsing action"),
        };

        // Get the operation arguments
        let arguments = ToDoOperation::validate_arguments(user_input, &operation, &mut logger)?;

        let todo_op = ToDoOperation {
            operation,
            arguments,
            err_lgr: logger,
        };

        Ok(todo_op)
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
        err_logger: &mut L,
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
                err_logger
                    .log(&"Operation does not take arguments")
                    .unwrap();
                Ok(arguments)
            }
            _ => Ok(arguments),
        }
    }
}

impl<L> Display for ToDoOperation<L>
where
    L: Logger,
{
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
    use std::io::{self, ErrorKind, Stderr};

    use super::*;
    use crate::error_logger::ErrorLogger;

    #[test]
    fn validates_operation_correctly() {
        let valid_action = ValidAction {
            name: "create",
            requires_arguments: false,
        };
        assert_eq!(
            ToDoOperation::<ErrorLogger<Stderr>>::validate_operation("create".into()),
            Ok(valid_action)
        );
    }

    #[test]
    fn returns_error_if_operation_in_invalid() {
        assert_eq!(
            ToDoOperation::<ErrorLogger<Stderr>>::validate_operation("invalid".into()),
            Err("Operation is not valid.\n[create, list]")
        );
    }

    #[test]
    fn validates_arguments_correctly() {
        let action_with_args: ValidAction = ValidAction {
            name: "list",
            requires_arguments: true,
        };

        let mut err_logger = ErrorLogger::new(Box::new(vec![]));

        let args = vec![String::from("test"), String::from("test")];
        assert_eq!(
            ToDoOperation::validate_arguments(args.into_iter(), &action_with_args, &mut err_logger),
            Ok(vec![String::from("test"), String::from("test")])
        );
    }

    #[test]
    fn validates_arguments_if_not_passed_required_arguments() {
        let action_with_no_required_args: ValidAction = ValidAction {
            name: "list",
            requires_arguments: true,
        };

        let mut err_logger = ErrorLogger::new(Box::new(vec![]));

        assert_eq!(
            ToDoOperation::validate_arguments(
                [].into_iter(),
                &action_with_no_required_args,
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
        let action_with_no_required_args: ValidAction = ValidAction {
            name: "create",
            requires_arguments: false,
        };
        let args = vec![String::from("test"), String::from("test")];

        let mut mock_logger = MockErrorLogger { was_called: false };

        assert_eq!(
            ToDoOperation::validate_arguments(
                args.into_iter(),
                &action_with_no_required_args,
                &mut mock_logger
            ),
            Ok(vec![String::from("test"), String::from("test")])
        );

        assert!(mock_logger.was_called);
    }
}
