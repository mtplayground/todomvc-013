mod footer;
mod todo_input;
mod todo_item;
mod todo_list;

pub use footer::*;
pub use todo_input::*;
pub use todo_item::*;
pub use todo_list::*;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum Filter {
    #[default]
    All,
    Active,
    Completed,
}

impl Filter {
    pub fn from_hash(hash: &str) -> Self {
        match hash {
            "#/active" => Filter::Active,
            "#/completed" => Filter::Completed,
            _ => Filter::All,
        }
    }
}
