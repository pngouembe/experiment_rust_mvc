use std::fmt::{Display, Error, Formatter};

use crate::{TodoItem, TodoListView};

impl Display for TodoItem {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let completion_str = match self.completed {
            true => "- [X]",
            false => "- [ ]",
        };
        write!(f, "{} {}", completion_str, self.title)
    }
}

#[derive(Default)]
pub struct ConsoleTodoListView;

impl TodoListView for ConsoleTodoListView {
    fn update_view(&self, tasks: &[&TodoItem]) {
        for task in tasks {
            println!("{}", task);
        }
    }
}
