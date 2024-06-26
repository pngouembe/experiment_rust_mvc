use crate::TodoItem;

#[derive(Default)]
pub struct TodoListView;

impl TodoListView {
    pub fn update_view(&self, tasks: &[&TodoItem]) {
        for task in tasks {
            println!("{}", task);
        }
    }
}
