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

fn main() {

    // initialize empty task-list
    let mut todoList : Vec<Task> = Vec::new();
    
    // loop starts here
        
    // user-input
    println!("welcome to to-do-list. what's the name of your first task?");

    let mut task = String::new();

    io::stdin()
        .read_line(&mut task)
        .expect("Failed to read line");
  
    task.pop();

    println!("what's the description?");
    let mut desc = String::new();

    io::stdin()
        .read_line(&mut desc)
        .expect("Failed to read line");

    println!("task {} has been pushed", &task);
    
    let mut currTask = Task{
        name: task,
        description: desc,
    };
    
    // println!("task {} has been pushed"), &currTask.name;
    todoList.push(currTask);

}
