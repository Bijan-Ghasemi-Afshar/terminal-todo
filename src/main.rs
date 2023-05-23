use std::env;
use std::fs::File;
use std::io::{Write};
use terminal_todo::todo::ToDo;

const CREATE_ACTION: &str = "create";
const DATABASE: &str = "todo-list.txt";

fn main() {
    let mut args = env::args();

    if args.len() < 2 {
        panic!("Action and arguments need to be passed to the program")
    }

    // Skipping the first argument which is the name of the program
    args.next().unwrap();

    let action = args.next().unwrap();
    println!("Action: {action}");

    let action_args: Vec<String> = args.collect();
    println!("Action arguments: {:?}", action_args);

    match action.as_str() {
        CREATE_ACTION => {
            if action_args.len() <=0 { 
                eprintln!("Arguments need to be passed to the action");
                panic!();
            }
            
            let todo_title = action_args.get(0).unwrap().clone();
            let todo = ToDo::new(todo_title);
            println!("{:?}", todo);

            let mut database: File = File::options()
                .append(true)
                .create(true)
                .open(DATABASE)
                .expect("Issue finding the file");

            database
                .write(&format!("{todo}\n")
                .as_bytes())
                .expect("Issue writing to the database");
        },
        _ => println!("No valid action was provided"),
    }

}
