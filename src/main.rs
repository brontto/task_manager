pub mod tasks;

use std::io::stdin;
use tasks::Tasks;

fn main() {
    let mut tasks = Tasks::new();
    tasks.add_task("title".to_string(), "description".to_string());
    let mut command = String::new();
    println!("Welcome to Task Manager");
    
    loop {
        command.clear();

        println!("1. Print all tasks");
        println!("2. Add task");
        println!("3. Make task done");
        println!("4. Remove task");
        println!("Write \"exit\" to exit");
        stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        let command_str = command.trim();

        match command_str {
            "1" => tasks.print_all(),
            "2" => {
                println!("Give title");
                let mut title = String::new();
                stdin().read_line(&mut title).expect("Failed to read line");

                println!("Give description");
                let mut description = String::new();
                stdin()
                    .read_line(&mut description)
                    .expect("Failed to read line");

                tasks.add_task(title, description);
                println!();
            }
            "3" => {
                let mut id = String::new();
                print!("Give task id: ");
                stdin().read_line(&mut id).expect("Failed to read line");

                let id: usize = match id.trim().parse::<usize>() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Input is not a number.");
                        continue;
                    }
                };
                tasks.make_task_done(id);
                println!();
            }
            "4" => {
                let mut id = String::new();
                print!("Give task id: ");   
                stdin().read_line(&mut id).expect("Failed to read line");

                let id: usize = match id.trim().parse::<usize>() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Input is not a number.");
                        continue;
                    }
                };
                tasks.delete_task(id);
                println!();
            }
            "exit" => break,
            _ => println!("Command not found"),
        }
    }
}
