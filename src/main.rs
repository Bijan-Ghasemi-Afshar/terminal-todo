use std::{
    env,
    io::{self, Stderr},
    process,
};
use terminal_todo::{error_logger::ErrorLogger, validator::ValidAction};

// const DATABASE: &str = "todo-list.txt";

fn main() {
    let error_logger: ErrorLogger<Stderr> = ErrorLogger::new(io::stderr());

    let valid_action: ValidAction = ValidAction::validate_input(env::args(), error_logger)
        .unwrap_or_else(|err| {
            eprintln!("{err}");
            process::exit(1);
        });

    (valid_action.operation)(valid_action.arguments);
}
