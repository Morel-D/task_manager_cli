// Different Task activities
use std::io;

struct  task {
    title: String,
    done: bool,
}


pub fn add_task(){
    println!("Add task activated");

    let mut input = String::new();


    println!("-------------ADD TASK ---------------");
    println!("Enter a task: {}", input);
    io::stdin().read_line(&mut input).expect("Failed to insert");
}

pub fn list_task(){
    println!("Print task activated");
}

pub fn done_task(){
    println!("Mark as done activated");
}

pub fn delete_task(){
    println!("Task Deleted activated");
}