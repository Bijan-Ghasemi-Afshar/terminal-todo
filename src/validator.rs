use crate::error_logger::Logger;
use crate::validator::valid_action::{ValidAction, VALID_ACTIONS};
use std::env::Args;
use std::fmt::Display;

pub mod valid_action;

impl ValidAction {
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
        let valid_action: ValidAction = match user_input.next() {
            Some(op) => ValidAction::validate_operation(op)?,
            None => return Err("Error parsing action"),
        };

        // Get the operation arguments
        let valid_action = ValidAction::validate_arguments(user_input, valid_action, &mut logger)?;

        Ok(valid_action)
    }

    fn validate_operation(operation: String) -> Result<ValidAction, &'static str> {
        for action in VALID_ACTIONS {
            if action.name == operation {
                return Ok(action);
            }
        }
        Err("Operation is not valid.\n[create, list]")
    }

    fn validate_arguments<'a, T, L: Logger>(
        mut user_args: T,
        valid_action: ValidAction,
        err_logger: &mut L,
    ) -> Result<ValidAction, &'static str>
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
                Ok(ValidAction {
                    arguments,
                    ..valid_action
                })
            }
            _ => Ok(ValidAction {
                arguments,
                ..valid_action
            }),
        }
    }
}

impl Display for ValidAction {
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

    fn empty_func() {}

    #[test]
    fn validates_operation_correctly() {
        let valid_action = ValidAction::validate_operation("create".into());
        assert!(valid_action.is_ok());
    }

    #[test]
    fn returns_error_if_operation_in_invalid() {
        assert_eq!(
            ValidAction::validate_operation("invalid".into()),
            Err("Operation is not valid.\n[create, list]")
        );
    }

    #[test]
    fn validates_arguments_correctly() {
        let action_with_args: ValidAction = ValidAction {
            name: "list",
            requires_arguments: true,
            arguments: vec![],
            operation: empty_func,
        };

        let mut err_logger = ErrorLogger::new(Box::new(vec![]));

        let args = vec![String::from("test"), String::from("test")];

        let valid_action =
            ValidAction::validate_arguments(args.into_iter(), action_with_args, &mut err_logger);

        assert!(valid_action.is_ok());
    }

    #[test]
    fn validates_arguments_if_not_passed_required_arguments() {
        let action_with_no_required_args: ValidAction = ValidAction {
            name: "list",
            requires_arguments: true,
            arguments: vec![],
            operation: empty_func,
        };

        let mut err_logger = ErrorLogger::new(Box::new(vec![]));

        assert_eq!(
            ValidAction::validate_arguments(
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
        let action_with_no_required_args: ValidAction = ValidAction {
            name: "create",
            requires_arguments: false,
            arguments: vec![],
            operation: empty_func,
        };
        let args = vec![String::from("test"), String::from("test")];

        let mut mock_logger = MockErrorLogger { was_called: false };

        let valid_action = ValidAction::validate_arguments(
            args.into_iter(),
            action_with_no_required_args,
            &mut mock_logger,
        );

        assert!(valid_action.is_ok());

        assert!(mock_logger.was_called);
    }
}
