pub struct Todo {
    todo_tasks: Vec<TodoTask>,
    task_id: i32,
}

pub struct TodoTask {
    id: i32,
    task: Task,
}

impl Todo {
    pub fn new() -> Todo {
        Todo{ todo_tasks: Vec::new(), task_id: 1 }
    }

    pub fn add_task(&mut self, task: Task) {
        self.todo_tasks.push(TodoTask{task, id: self.task_id});
        self.task_id += 1;
    }
    
    pub fn status(&self) -> Vec<String> {
        let mut status: Vec<String> = Vec::new();
        for todo_task in &self.todo_tasks {
            if todo_task.task.done {
                status.push(format!("[x] {}. {}", todo_task.id, todo_task.task.name));
            } else {
                status.push(format!("[ ] {}. {}", todo_task.id, todo_task.task.name));
            }
        }
        status
    }

    pub fn check(&mut self, id_to_check: i32) {
        for todo_task in &mut self.todo_tasks {
            if todo_task.id == id_to_check {
                todo_task.task.done = true;
            }
        }
    }
}

pub struct Task {
    name: String,
    done: bool,
}

impl Task {
    pub fn new_task(name: &String) -> Task {
        Task{ name: name.to_string(), done: false}
    }
}
