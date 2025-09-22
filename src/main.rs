// mod task;
use std::io;

fn main() {
    let mut input = String::new();

    println!("-------------------------------------");
    println!("1- Add Task");
    println!("2- List Task");
    println!("3- Mark as done");
    println!("4- Delete Task");
    println!("-------------------------------------");
    println!("Select one Option: {}", input.trim());
    io::stdin().read_line(&mut input).expect("Failed to read line");

}
