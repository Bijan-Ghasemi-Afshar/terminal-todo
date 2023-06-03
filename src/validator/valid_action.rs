const CREATE: &'static str = "create";
const LIST: &'static str = "list";

#[derive(PartialEq, Debug)]
pub struct ValidAction {
    pub name: &'static str,
    pub requires_arguments: bool,
    pub arguments: Vec<String>,
    pub operation: fn() -> (),
}

fn list_operation() {
    println!("Printing all ToDo items");
}

fn create_operation() {
    println!("Creating a ToDo item");
}

pub const VALID_ACTIONS: [ValidAction; 2] = [
    ValidAction {
        name: CREATE,
        requires_arguments: false,
        arguments: vec![],
        operation: create_operation,
    },
    ValidAction {
        name: LIST,
        requires_arguments: true,
        arguments: vec![],
        operation: list_operation,
    },
];
