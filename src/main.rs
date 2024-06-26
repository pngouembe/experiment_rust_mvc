use model::HashMapTodoListModel;
use presenter::TodoListPresenter;
use view::ConsoleTodoListView;

mod model;
mod presenter;
mod view;

struct TodoItem {
    title: String,
    completed: bool,
}

trait TodoListView {
    fn update_view(&self, tasks: &[&TodoItem]);
}

trait TodoListModel {
    fn get_tasks(&self) -> Vec<&TodoItem>;
    fn add_task(&mut self, title: String);
}

fn main() {
    let view = ConsoleTodoListView::default();
    let model = HashMapTodoListModel::default();

    let mut presenter = TodoListPresenter::new(Box::new(view), Box::new(model));

    presenter.run();
}
