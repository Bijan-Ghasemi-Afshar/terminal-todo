use crate::{action::Action, log_wrapper::Logger};
use std::env::Args;

pub struct Validator {}

impl<'a> Validator {
    pub fn validate_input<L: Logger + 'a>(
        mut user_input: Args,
        logger: &'a mut L,
    ) -> Result<Action<'a>, &'static str> {
        if user_input.len() < 2 {
            return Err("An action needs to be provided\ncreate\nlist\nedit [index]\ndone [index]\nundone [index]\ndelete [index]");
        }

        // Skipping the first arg since it the program name
        user_input.next();

        // Get the action
        let mut valid_action: Action = match user_input.next() {
            Some(op) => Action::new(&op, logger)?,
            None => return Err("Error parsing action"),
        };

        // Get the action arguments
        Validator::validate_arguments::<Args, L>(user_input, &mut valid_action)?;

        Ok(valid_action)
    }

    fn validate_arguments<'b, T, L: Logger>(
        mut user_args: T,
        valid_action: &mut Action<'a>,
    ) -> Result<(), &'static str>
    where
        T: Iterator<Item = String>,
    {
        let mut arguments: Vec<String> = vec![];

        while let Some(arg) = user_args.next() {
            arguments.push(arg);
        }

        match valid_action.requires_arguments {
            x if x && arguments.len() <= 0 => return Err("Action requires arguments"),
            x if !x && arguments.len() > 0 => {
                valid_action
                    .logger
                    .as_mut()
                    .unwrap()
                    .log_errln(&"Action does not take arguments")
                    .unwrap();
                valid_action.arguments = arguments;
                Ok(())
            }
            _ => {
                valid_action.arguments = arguments;
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        error::Error,
        io::{self, ErrorKind},
    };

    use super::*;
    use crate::log_wrapper::Logger;

    struct MockErrorLogger {
        was_called: bool,
    }

    impl Logger for MockErrorLogger {
        fn log_errln<'a>(&mut self, msg: &'a str) -> Result<(), Box<dyn std::error::Error>> {
            if msg == "" {
                return Err(Box::new(io::Error::new(ErrorKind::Other, "oh no!")));
            }
            self.was_called = true;
            Ok(())
        }
        fn log_stdln<'a>(&mut self, _msg: &'a str) -> Result<(), Box<dyn Error>> {
            Ok(())
        }
        fn log_err<'a>(&mut self, _msg: &'a str) -> Result<(), Box<dyn Error>> {
            Ok(())
        }
        fn log_std<'a>(&mut self, _msg: &'a str) -> Result<(), Box<dyn Error>> {
            Ok(())
        }

        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    #[test]
    fn validates_arguments_correctly() {
        let mut mock_logger = MockErrorLogger { was_called: false };

        let mut action_with_args: Action = Action::new(&"create", &mut mock_logger).unwrap();

        let args = vec![String::from("test"), String::from("test")];

        let valid_action = Validator::validate_arguments::<
            std::vec::IntoIter<String>,
            MockErrorLogger,
        >(args.into_iter(), &mut action_with_args);

        assert!(valid_action.is_ok());
    }

    #[test]
    fn validates_arguments_if_not_passed_required_arguments() {
        let mut mock_logger = MockErrorLogger { was_called: false };

        let mut action_with_no_required_args: Action =
            Action::new(&"edit", &mut mock_logger).unwrap();

        assert_eq!(
            Validator::validate_arguments::<std::vec::IntoIter<String>, MockErrorLogger>(
                Vec::<String>::new().into_iter(),
                &mut action_with_no_required_args
            ),
            Err("Action requires arguments"),
        );
    }

    #[test]
    fn prints_warning_if_arguments_are_not_required() {
        let mut mock_logger = MockErrorLogger { was_called: false };

        let mut action_with_no_required_args: Action =
            Action::new(&"create", &mut mock_logger).unwrap();

        let args = vec![String::from("test"), String::from("test")];

        let valid_action = Validator::validate_arguments::<
            std::vec::IntoIter<String>,
            MockErrorLogger,
        >(args.into_iter(), &mut action_with_no_required_args);

        assert!(valid_action.is_ok());

        assert!(mock_logger.was_called);
    }
}
