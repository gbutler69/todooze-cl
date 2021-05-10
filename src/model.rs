use string_supplements::StringConstrained;

use crate::string_supplements;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct State {
    pub id: u64,
    pub name: StringConstrained<20, false, false>,
    pub description: StringConstrained<60, false, false>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Action {
    pub id: u64,
    pub name: StringConstrained<20, false, false>,
    pub description: String,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct StateAction {}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Resource {}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Project {}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ToDo {}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ToDoResource {}
