mod todo;

fn main() {
    let mut task1 = todo::Task::new_task(String::from("learn rust"));
    println!("{} is {}", task1.name(), task1.status());
    task1.check();
    println!("{} is {}", task1.name(), task1.status());
}
