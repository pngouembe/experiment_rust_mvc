use std::fmt::{Display, Error, Formatter};

use model::TodoListModel;
use presenter::TodoListPresenter;
use view::TodoListView;

mod model;
mod presenter;
mod view;

struct TodoItem {
    title: String,
    completed: bool,
}

impl Display for TodoItem {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let completion_str = match self.completed {
            true => "- [X]",
            false => "- [ ]",
        };
        write!(f, "{} {}", completion_str, self.title)
    }
}

fn main() {
    let view = TodoListView::default();
    let model = TodoListModel::default();

    let mut presenter = TodoListPresenter::new(view, model);

    presenter.run();
}
