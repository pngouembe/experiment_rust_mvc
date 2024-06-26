use std::collections::HashMap;

use crate::{TodoItem, TodoListModel};

type Id = String;

#[derive(Default)]
pub struct HashMapTodoListModel(HashMap<Id, TodoItem>);

impl TodoListModel for HashMapTodoListModel {
    fn get_tasks(&self) -> Vec<&TodoItem> {
        self.0.values().collect()
    }

    fn add_task(&mut self, title: String) {
        let id = format!("{}", self.0.len());
        let item = TodoItem {
            title,
            completed: false,
        };
        self.0.insert(id, item);
    }
}
