use std::{
    env,
    io::{self, Stderr, Stdout},
    process,
};
use terminal_todo::{
    action::{database::DatabaseAgent, Action},
    log_wrapper::LogWrapper,
    validator::Validator,
};

fn main() {
    let mut logger: LogWrapper<Stderr, Stdout> = LogWrapper::new(io::stderr(), io::stdout());
    let database = DatabaseAgent {};

    let mut valid_action: Action = Validator::validate_input(env::args(), &mut logger, &database)
        .unwrap_or_else(|err| {
            eprintln!("{err}");
            process::exit(1);
        });

    valid_action.execute_action().unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });
}
