// mod task;
use std::io;

fn main() {
    let mut input = String::new();

    println!("-------------------------------------");
    println!("1- Add Task");
    println!("2- List Task");
    println!("3- Mark as done");
    println!("4- Delete Task");
    println!("5- Exit");
    println!("-------------------------------------");
    println!("Select one Option: {}", input);
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let slice = input.trim().parse::<i32>().unwrap();

    match slice {
        1 => println!("Add task"),
        2 => println!("List Task"),
        3 => println!("Mark as done"),
        4 => println!("Delete Task"),
        5 => println!("Exit"),
        _ => println!("Nothing"),

    }

}
