use std::env;

const CREATE_ACTION: &str = "create";

fn main() {
    let mut args = env::args();

    if args.len() < 2 {
        eprintln!("Action and arguments need to be passed to the program");
    }

    // Skipping the first argument which is the name of the program
    args.next().unwrap();

    let action = args.next().unwrap();
    println!("Action: {action}");

    let action_args: Vec<String> = args.collect();
    println!("Action arguments: {:?}", action_args);

    match action.as_str() {
        CREATE_ACTION => {
            let todo_title = action_args.get(0).unwrap().clone();
            let first_todo = ToDo::new(todo_title);
            println!("{:?}", first_todo);
        },
        _ => println!("No valid action was provided"),
    }

}


#[derive(Debug)]
struct ToDo {
    title: String,
}

impl ToDo {
    fn new(title: String) -> Self {
        ToDo {
            title,
        }
    }
}
