// TASK: todo list :D
//
// CLI application, something where you can define multiple tasks,
// add, remove, edit tasks
// mark tasks as complete

use std::io;

struct Task {
    name : String,
    description: String,
}

impl Task {
    fn print(&self) {
        println!("name: {}, description: {}", self.name, self.description);
    }
}

fn main() {

    // initialize empty task-list
    let mut todoList : Vec<Task> = Vec::new();
    
    println!("welcome to to-do-list.");
    
    loop {
        println!("what's the name of your task?");
        let mut task = String::new();
        // user-input
        io::stdin()
            .read_line(&mut task)
            .expect("Failed to read line");
        // used to remove new_line for printing
        task = task.trim().to_string();
        
        println!("what's the description?");
        let mut desc = String::new();

        io::stdin()
            .read_line(&mut desc)
            .expect("Failed to read line");
        desc = desc.trim().to_string();

        println!("task {} has been pushed", &task);

        let mut currTask = Task{
            name: task,
            description: desc,
        };

        todoList.push(currTask);

        println!("want to make more tasks? y/n");
        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");

        if answer.remove(0) != 'y' {
             break;
        }
    }

    for x in todoList {
        x.print();
    }

}
