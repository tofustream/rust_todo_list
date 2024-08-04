use rust_todo_list::TodoList;
use std::io::{self, Write};

fn main() {
    let mut todo_list = TodoList::new();

    loop {
        println!("\nTODO List Application");
        println!("1. Add task");
        println!("2. List tasks");
        println!("3. Remove task");
        println!("4. Toggle task completion");
        println!("5. Exit");

        print!("Enter your choice: ");
        if io::stdout().flush().is_err() {
            println!("Failed to flush stdout");
            continue;
        }

        let mut choice = String::new();
        if io::stdin().read_line(&mut choice).is_err() {
            println!("Failed to read line");
            continue;
        }

        let choice = match choice.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, please enter a number");
                continue;
            }
        };

        match choice {
            1 => {
                print!("Enter task description: ");
                if io::stdout().flush().is_err() {
                    println!("Failed to flush stdout");
                    continue;
                }

                let mut description = String::new();
                if io::stdin().read_line(&mut description).is_err() {
                    println!("Failed to read line");
                    continue;
                }

                let description = description.trim().to_string();
                todo_list = todo_list.add_task(description);
                println!("Task added successfully!");
            }
            2 => {
                let tasks = todo_list.list_tasks();
                if tasks.is_empty() {
                    println!("No tasks available.");
                } else {
                    println!("Tasks:");
                    for task in tasks {
                        println!("{}", task);
                    }
                }
            }
            3 => {
                print!("Enter task index to remove: ");
                if io::stdout().flush().is_err() {
                    println!("Failed to flush stdout");
                    continue;
                }

                let mut index = String::new();
                if io::stdin().read_line(&mut index).is_err() {
                    println!("Failed to read line");
                    continue;
                }

                let index = match index.trim().parse::<usize>() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input, please enter a number");
                        continue;
                    }
                };

                todo_list = todo_list.remove_task(index - 1);
                println!("Task removed successfully!");
            }
            4 => {
                print!("Enter task index to toggle: ");
                if io::stdout().flush().is_err() {
                    println!("Failed to flush stdout");
                    continue;
                }

                let mut index = String::new();
                if io::stdin().read_line(&mut index).is_err() {
                    println!("Failed to read line");
                    continue;
                }

                let index = match index.trim().parse::<usize>() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input, please enter a number");
                        continue;
                    }
                };

                todo_list = todo_list.toggle_task(index - 1);
                println!("Task completion status toggled successfully!");
            }
            5 => {
                println!("Exiting the application.");
                break;
            }
            _ => println!("Invalid choice, please try again"),
        }
    }
}
