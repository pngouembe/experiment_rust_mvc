use std::collections::HashMap;

use crate::TodoItem;

type Id = String;

#[derive(Default)]
pub struct TodoListModel(HashMap<Id, TodoItem>);

impl TodoListModel {
    pub fn get_tasks(&self) -> Vec<&TodoItem> {
        self.0.values().collect()
    }

    pub fn add_task(&mut self, title: String) {
        let id = format!("{}", self.0.len());
        let item = TodoItem {
            title,
            completed: false,
        };
        self.0.insert(id, item);
    }
}
