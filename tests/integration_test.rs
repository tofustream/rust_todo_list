#[cfg(test)]
mod tests {
    use rust_todo_list::TodoList;

    #[test]
    fn it_adds_a_task() {
        let todo_list = TodoList::new();
        let updated_todo_list = todo_list.add_task("Learn Rust".to_string());

        assert_eq!(updated_todo_list.tasks.len(), 1);
        assert_eq!(updated_todo_list.tasks[0].description, "Learn Rust");
        assert!(!updated_todo_list.tasks[0].completed);
    }

    #[test]
    fn it_removes_a_task() {
        let todo_list = TodoList::new();
        let todo_list = todo_list.add_task("Learn Rust".to_string());
        let todo_list = todo_list.add_task("Write code".to_string());

        let updated_todo_list = todo_list.remove_task(0);
        assert_eq!(updated_todo_list.tasks.len(), 1);
        assert_eq!(updated_todo_list.tasks[0].description, "Write code");
    }

    #[test]
    fn it_toggles_task_completion() {
        let todo_list = TodoList::new();
        let todo_list = todo_list.add_task("Learn Rust".to_string());

        let updated_todo_list = todo_list.toggle_task(0);
        assert!(updated_todo_list.tasks[0].completed);

        let updated_todo_list = updated_todo_list.toggle_task(0);
        assert!(!updated_todo_list.tasks[0].completed);
    }
}
