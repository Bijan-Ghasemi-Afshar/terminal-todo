
const CREATE: &'static str = "create";
const LIST: &'static str = "list";

#[derive(PartialEq, Debug)]
pub struct ValidAction {
    pub name: &'static str,
    pub requires_arguments: bool,
}

pub const VALID_ACTIONS: [ValidAction; 2] = [
    ValidAction { name: CREATE, requires_arguments: false },
    ValidAction { name: LIST, requires_arguments: true },
];
