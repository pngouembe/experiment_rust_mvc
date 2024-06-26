use std::io;

use crate::{model::TodoListModel, view::TodoListView};

pub struct TodoListPresenter {
    view: TodoListView,
    model: TodoListModel,
}

impl TodoListPresenter {
    pub fn new(view: TodoListView, model: TodoListModel) -> Self {
        Self { view, model }
    }

    pub fn run(&mut self) {
        loop {
            let task_name = self.get_user_input();
            self.model.add_task(task_name);
            self.view.update_view(&self.model.get_tasks());
        }
    }

    fn get_user_input(&self) -> String {
        let mut buffer = String::new();
        let stdin = io::stdin(); // We get `Stdin` here.
        stdin.read_line(&mut buffer).unwrap();
        buffer.trim().to_string()
    }
}
