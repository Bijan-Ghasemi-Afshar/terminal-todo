use crate::{
    action::{Action, ACTIONS},
    error_logger::Logger,
};
use std::env::Args;

pub struct Validator {}

impl Validator {
    pub fn validate_input<L: Logger>(
        mut user_input: Args,
        mut logger: L,
    ) -> Result<Action, &'static str> {
        if user_input.len() < 2 {
            return Err("An action needs to be provided\ncreate\nlist\nedit [index]\ndone [index]\nundone [index]\ndelete [index]");
        }

        // Skipping the first arg since it the program name
        user_input.next();

        // Get the action
        let valid_action: Action = match user_input.next() {
            Some(op) => Validator::validate_action(op)?,
            None => return Err("Error parsing action"),
        };

        // Get the action arguments
        let valid_action = Validator::validate_arguments(user_input, valid_action, &mut logger)?;

        Ok(valid_action)
    }

    fn validate_action(action_name: String) -> Result<Action, &'static str> {
        for action in ACTIONS {
            if action.name == action_name {
                return Ok(action);
            }
        }
        Err("An action needs to be provided\ncreate\nlist\nedit [index]\ndone [index]\nundone [index]\ndelete [index]")
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
            x if x && arguments.len() <= 0 => return Err("action requires arguments"),
            x if !x && arguments.len() > 0 => {
                err_logger
                    .log(&"action does not take arguments")
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

#[cfg(test)]
mod tests {
    use std::io::{self, ErrorKind};

    use super::*;
    use crate::error_logger::ErrorLogger;

    fn empty_func(_: Vec<String>) -> Result<(), &'static str> {
        Ok(())
    }

    #[test]
    fn validates_action_correctly() {
        let valid_action = Validator::validate_action("create".into());
        assert!(valid_action.is_ok());
    }

    #[test]
    fn returns_error_if_action_in_invalid() {
        assert_eq!(
            Validator::validate_action("invalid".into()),
            Err("An action needs to be provided\ncreate\nlist\nedit [index]\ndone [index]\nundone [index]\ndelete [index]")
        );
    }

    #[test]
    fn validates_arguments_correctly() {
        let action_with_args: Action = Action {
            name: "list",
            requires_arguments: true,
            arguments: vec![],
            execute: empty_func,
        };

        let mut err_logger = ErrorLogger::new(Box::new(vec![]));

        let args = vec![String::from("test"), String::from("test")];

        let valid_action =
            Validator::validate_arguments(args.into_iter(), action_with_args, &mut err_logger);

        assert!(valid_action.is_ok());
    }

    #[test]
    fn validates_arguments_if_not_passed_required_arguments() {
        let action_with_no_required_args: Action = Action {
            name: "list",
            requires_arguments: true,
            arguments: vec![],
            execute: empty_func,
        };

        let mut err_logger = ErrorLogger::new(Box::new(vec![]));

        assert_eq!(
            Validator::validate_arguments(
                [].into_iter(),
                action_with_no_required_args,
                &mut err_logger
            ),
            Err("action requires arguments"),
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
            execute: empty_func,
        };
        let args = vec![String::from("test"), String::from("test")];

        let mut mock_logger = MockErrorLogger { was_called: false };

        let valid_action = Validator::validate_arguments(
            args.into_iter(),
            action_with_no_required_args,
            &mut mock_logger,
        );

        assert!(valid_action.is_ok());

        assert!(mock_logger.was_called);
    }
}
