use crate::error_logger::Logger;
use crate::validator::valid_action::{ValidAction, VALID_ACTIONS};
use std::cell::RefCell;
use std::env::Args;
use std::fmt::Display;
use std::rc::Rc;

pub mod valid_action;

pub struct ToDoOperation {
    operation: ValidAction,
    arguments: Vec<String>,
    err_lgr: Rc<RefCell<dyn Logger>>,
}

impl ToDoOperation {
    pub fn new(
        mut user_input: Args,
        logger: Rc<RefCell<dyn Logger>>,
    ) -> Result<Self, &'static str> {
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
        let arguments = ToDoOperation::validate_arguments(user_input, &operation, logger.clone())?;

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
        err_logger: Rc<RefCell<dyn Logger>>,
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
                    .borrow_mut()
                    .log(&"Operation does not take arguments")
                    .unwrap();
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
    use std::io::{self, ErrorKind};

    use super::*;
    use crate::error_logger::ErrorLogger;

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

        let err_logger = Rc::new(RefCell::new(ErrorLogger::new(Box::new(vec![]))));

        let args = vec![String::from("test"), String::from("test")];
        assert_eq!(
            ToDoOperation::validate_arguments(args.into_iter(), &action_with_args, err_logger),
            Ok(vec![String::from("test"), String::from("test")])
        );
    }

    #[test]
    fn validates_arguments_if_not_passed_required_arguments() {
        let action_with_no_required_args: ValidAction = ValidAction {
            name: "list",
            requires_arguments: true,
        };

        let err_logger = Rc::new(RefCell::new(ErrorLogger::new(Box::new(vec![]))));

        assert_eq!(
            ToDoOperation::validate_arguments(
                [].into_iter(),
                &action_with_no_required_args,
                err_logger
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

        let mock_logger = MockErrorLogger { was_called: false };

        let err_logger = Rc::new(RefCell::new(mock_logger));

        let ref_err = Rc::clone(&err_logger) as Rc<RefCell<dyn Logger>>;

        assert_eq!(
            ToDoOperation::validate_arguments(
                args.into_iter(),
                &action_with_no_required_args,
                Rc::clone(&ref_err)
            ),
            Ok(vec![String::from("test"), String::from("test")])
        );

        assert!(err_logger.borrow().was_called);
    }
}
