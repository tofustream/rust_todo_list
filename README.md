
# Rust TODO List Application

This is a simple command-line TODO list application written in Rust. The application allows users to manage tasks by adding, listing, removing, and toggling the completion status of tasks.

## Features

- **Add Task**: Add a new task with a description to the TODO list.
- **List Tasks**: Display all tasks in the TODO list.
- **Remove Task**: Remove a task from the TODO list by specifying its index.
- **Toggle Task Completion**: Toggle the completion status of a task by specifying its index.
- **Exit**: Exit the application.

## Prerequisites

- **Rust**: Make sure you have Rust installed. You can install it using the following command:
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

## Usage

1. **Clone the repository:**

    ```bash
    git clone git@github.com:tofustream/rust_todo_list.git
    cd rust_todo_list
    ```

2. **Build the project:**

    ```bash
    cargo build --release
    ```

3. **Run the application:**

    ```bash
    cargo run
    ```

4. **Using the application:**

    - When you run the application, you will be presented with a menu that allows you to choose an action.
    - Enter the corresponding number to perform an action:
      - `1`: Add a task.
      - `2`: List all tasks.
      - `3`: Remove a task.
      - `4`: Toggle task completion.
      - `5`: Exit the application.

5. **Adding a Task:**

    - Select the "Add task" option by entering `1`.
    - Enter the task description and press Enter.
    - The task will be added to the TODO list.

6. **Listing Tasks:**

    - Select the "List tasks" option by entering `2`.
    - The application will display all the tasks currently in the TODO list.

7. **Removing a Task:**

    - Select the "Remove task" option by entering `3`.
    - Enter the index of the task you want to remove and press Enter.
    - The task will be removed from the TODO list.

8. **Toggling Task Completion:**

    - Select the "Toggle task completion" option by entering `4`.
    - Enter the index of the task you want to toggle and press Enter.
    - The completion status of the task will be toggled.

9. **Exiting the Application:**

    - Select the "Exit" option by entering `5`.
    - The application will close.

## Example

Here is an example of how the application might look when running:

```
TODO List Application
1. Add task
2. List tasks
3. Remove task
4. Toggle task completion
5. Exit
Enter your choice: 1
Enter task description: Buy groceries
Task added successfully!

TODO List Application
1. Add task
2. List tasks
3. Remove task
4. Toggle task completion
5. Exit
Enter your choice: 2
Tasks:
1. Buy groceries

TODO List Application
1. Add task
2. List tasks
3. Remove task
4. Toggle task completion
5. Exit
Enter your choice: 4
Enter task index to toggle: 1
Task completion status toggled successfully!
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by the need for a simple, command-line-based TODO list application in Rust.
