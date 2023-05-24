
const CREATE: &'static str = "create";
const LIST: &'static str = "list";

pub struct ValidAction {
    pub name: &'static str,
    pub uses_arguments: bool,
}

pub const VALID_ACTIONS: [ValidAction; 2] = [
    ValidAction { name: CREATE, uses_arguments: false },
    ValidAction { name: LIST, uses_arguments: true },
];
