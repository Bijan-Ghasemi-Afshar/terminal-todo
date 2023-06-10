use std::{
    env,
    io::{self, Stderr},
    process,
};
use terminal_todo::{action::Action, error_logger::ErrorLogger, validator::Validator};

fn main() {
    let mut error_logger: ErrorLogger<Stderr> = ErrorLogger::new(io::stderr());

    let valid_action: Action =
        Validator::validate_input(env::args(), &mut error_logger).unwrap_or_else(|err| {
            eprintln!("{err}");
            process::exit(1);
        });

    (valid_action.execute)(valid_action.arguments).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });
}
