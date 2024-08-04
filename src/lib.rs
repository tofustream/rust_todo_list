#[derive(Clone, Debug, PartialEq)]
pub struct Task {
    pub description: String,
    pub completed: bool,
}

impl Task {
    pub fn new(description: String) -> Task {
        Task {
            description,
            completed: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TodoList {
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList { tasks: Vec::new() }
    }

    pub fn add_task(&self, description: String) -> TodoList {
        let mut new_tasks = self.tasks.clone();
        let task = Task::new(description);
        new_tasks.push(task);
        TodoList { tasks: new_tasks }
    }

    pub fn list_tasks(&self) -> Vec<String> {
        self.tasks
            .iter() // タスクの参照を返すイテレータを生成
            .enumerate() // 各タスクにインデックス番号を追加
            .map(|(i, task)| {
                let status = if task.completed { "done" } else { "not done" };
                format!("{}: {} [{}]", i + 1, task.description, status)
            })
            .collect() // イテレータを Vec<String> に変換
    }

    pub fn remove_task(&self, index: usize) -> TodoList {
        let mut new_task = self.tasks.clone();
        if index < new_task.len() {
            new_task.remove(index);
        }
        TodoList { tasks: new_task }
    }

    pub fn toggle_task(&self, index: usize) -> TodoList {
        let mut new_tasks = self.tasks.clone();
        if let Some(task) = new_tasks.get_mut(index) {
            task.completed = !task.completed;
        }
        TodoList { tasks: new_tasks }
    }
}
