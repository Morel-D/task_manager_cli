mod task;
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
            1 => task::add_task(),
            2 => task::list_task(),
            3 => task::done_task(),
            4 => task::delete_task(),
            5 => println!("Exit"),
            _ => println!("Nothing"),
        }

}
