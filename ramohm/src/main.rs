//importing libraries
use std::io;

//main function
fn main() {
    //calling menu function
    menu();
}
//menu function
fn menu(){
    //menu function menu
    println!("------------------------RamOhm--------------------------");
    println!("------------------------v1.0-----------------------------");
    println!("------------------------Menu-----------------------------");
    println!("1. Calculate Resistance");
    println!("2. Calculate Current");
    println!("3. Calculate Voltage");
    println!("4. Calculate Power");
    println!("5. Calculate Energy");
    println!("6. Exit");
    println!("Enter your choice: ");
    //declaring choice variable 
    let mut choice = String::new();
    //reading input from user
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let numchoice:i32 = choice.trim().parse();

    match numchoice{
        1 => calcresistance(),
        2 => calccurrent(),
        3 => calcvoltage(),
        4 => calcpower(),
        5 => calcenergy(),
        6 => println!("Exiting..."),
        _ => println!("Invalid choice"),
    }

}
fn calcresistance(){
    println!("test");
}
fn calccurrent(){

}
fn calcvoltage(){

}
fn calcpower(){

}
fn calcenergy(){

}