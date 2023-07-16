#[derive(Clone)]
pub struct Task {
    id: usize,
    title: String,
    description: String,
    completed: bool,
}

pub struct Tasks {
    list: Vec<Task>,
}

impl Tasks {
    pub fn new() -> Self {
        Tasks { list: Vec::new() }
    }

    pub fn add_task(&mut self, title: String, description: String){
        let new_task = Task {
            id: self.list.len() + 1,
            title: title.replace('\n', "").replace('\r', ""),
            description: description.replace('\n', "").replace('\r', ""),
            completed: false,
        };
        self.list.push(new_task)
        
    }

    pub fn get_tasks(&self) -> Vec<Task> {
        self.list.clone()
    }

    pub fn edit_task(&mut self, id: usize, new_task: Task) {
        if let Some(task) = self.list.iter_mut().find(|task| task.id == id){
            *task = new_task;
        }
    }

    pub fn delete_task(&mut self, id: usize) {
        let task = self
            .list
            .iter()
            .position(|task| task.id == id)
            .map(|i| self.list.swap_remove(i));

        match task {
            Some(task) => println!("Task \"{}\" is remvoed", task.title),
            None => println!("Task not found"),
        }
    }

    pub fn print_all(&self) {
        for task in self.list.iter() {
            pretty_print(task);
        }
    }

    pub fn make_task_done(&mut self, id: usize) {
        let index = self.list.iter().position(|task| task.id == id);
        
        match index {
            Some(i) => self.list[i].completed = true,
            None => println!("Task not found"),
        }

    }
}

fn pretty_print(task: &Task) {
    println!("id: {}", task.id);
    println!("{} : {}", task.title, task.description);
    println!("completed: {}", task.completed);
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_is_added() {
        let mut tasks = Tasks::new();

        tasks.add_task("testititteli".to_string(), "testidesrkiptioon".to_string());

        let task_vec = tasks.get_tasks();

        assert_eq!(task_vec.len(), 1);
    }
}
