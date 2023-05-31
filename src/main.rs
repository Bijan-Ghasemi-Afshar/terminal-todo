use std::{
    env,
    io::{self},
    process,
    rc::Rc, cell::RefCell,
};
use terminal_todo::{error_logger::{ErrorLogger, Logger}, validator::ToDoOperation};

// const DATABASE: &str = "todo-list.txt";

fn main() {
    // let error_logger = Rc::new(RefCell::new(Box::new(ErrorLogger::new(Box::new(io::stderr()))) as dyn Logger));

    // let todo_operations: ToDoOperation = ToDoOperation::new(env::args(), Rc::clone(&error_logger))
    //     .unwrap_or_else(|err| {
    //         eprintln!("{err}");
    //         process::exit(1);
    //     });

    // println!("{todo_operations}");

    // match action.as_str() {
    //     CREATE_ACTION => {
    //         if action_args.len() <=0 {
    //             eprintln!("Arguments need to be passed to the action");
    //             panic!();
    //         }

    //         let todo_title = action_args.get(0).unwrap().clone();
    //         let todo = ToDo::new(todo_title);
    //         println!("{:?}", todo);

    //         let mut database: File = File::options()
    //             .append(true)
    //             .create(true)
    //             .open(DATABASE)
    //             .expect("Issue finding the file");

    //         database
    //             .write(&format!("{todo}\n")
    //             .as_bytes())
    //             .expect("Issue writing to the database");
    //     },
    //     _ => println!("No valid action was provided"),
    // }
}
