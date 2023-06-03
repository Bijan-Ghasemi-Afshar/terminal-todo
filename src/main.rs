use std::{
    env,
    io::{self, Stderr},
    process,
};
use terminal_todo::{action::Action, error_logger::ErrorLogger};

fn main() {
    let error_logger: ErrorLogger<Stderr> = ErrorLogger::new(io::stderr());

    let valid_action: Action =
        Action::validate_input(env::args(), error_logger).unwrap_or_else(|err| {
            eprintln!("{err}");
            process::exit(1);
        });

    (valid_action.operation)(valid_action.arguments);
}
