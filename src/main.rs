use std::{
    env,
    io::{self, Stderr, Stdout},
    process,
};
use terminal_todo::{action::Action, log_wrapper::LogWrapper, validator::Validator};

fn main() {
    let mut logger: LogWrapper<Stderr, Stdout> = LogWrapper::new(io::stderr(), io::stdout());

    let valid_action: Action =
        Validator::validate_input(env::args(), &mut logger).unwrap_or_else(|err| {
            eprintln!("{err}");
            process::exit(1);
        });

    (valid_action.execute)(valid_action.arguments).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });
}
