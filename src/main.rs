use std::io::{self, BufRead};
use std::fs::File;
use std::io::prelude::*;

mod todo;

fn main() {
    let mut my_todo = todo::Todo::new();
    let stdin = io::stdin();
    let iterator = stdin.lock().lines();
    let mut is_checking = false;
    for line in iterator {
        let text = line.unwrap();
        match text.as_ref() {
            "status" => {
                let tasks_status = my_todo.status();
                for task_status in tasks_status {
                    println!("{}", &task_status);
                }
            },
            "check" => {
                println!("choose index to check: ");
                let tasks_status = my_todo.status();
                for task_status in tasks_status {
                    println!("{}", &task_status);
                }
                is_checking = true;
            }
            "write" => {
                let mut file = File::create("todo.txt").unwrap();
                let tasks_status = my_todo.status();
                for task_status in tasks_status {
                    file.write_all(format!("{}\n", &task_status).as_bytes()).expect("failed to write todo");
                }
            }
            _ => {
                if is_checking {
                    let id_to_check = text.parse::<i32>().unwrap();
                    my_todo.check(id_to_check);
                    is_checking = false;
                } else {
                    let task = todo::Task::new_task(&text);
                    my_todo.add_task(task);
                    println!("{} added", text);
                }
            },
        }
    }
}
