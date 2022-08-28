use druid::Data;
use std::sync::Arc;

#[derive(Clone, Data)]
struct TodoList {
    items: Arc<Vec<TodoItem>>,
}

#[derive(Clone, Data)]
struct TodoItem {
    category: Category,
    title: String,
    note: Option<String>,
    completed: bool,
    due_date: Option<Arc<DateTime>>,
    #[data(same_fn = "PartialEq::eq")]
    added_date: DateTime,
    #[data(ignore)]
    debug_timestamp: usize,
}

#[derive(Clone, Data, PartialEq)]
enum Category {
    Work,
    Play,
    Revolution,
}

#[derive(Clone, PartialEq, Data)]
struct DateTime {}

fn main() {
    //
}
