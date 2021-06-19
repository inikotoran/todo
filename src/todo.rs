pub struct Task {
    name: String,
    done: bool,
}

impl Task {
    pub fn new_task(name: String) -> Task {
        Task{ name, done: false}
    }
    
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn status(&self) -> String {
        if self.done {
            return String::from("done")
        }
        String::from("not done")
    }
    
    pub fn check(&mut self) {
        self.done = true
    }
}
