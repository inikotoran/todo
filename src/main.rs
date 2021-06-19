use std::env;

mod todo;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut task1 = todo::Task::new_task(&args[1]);
    println!("created a task: {}", task1.name());
    task1.check();
    println!("{} is {}", task1.name(), task1.status());
}
