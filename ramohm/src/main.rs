//importing libraries
use std::io;

pub mod formulae;

//main function
fn main() {
    //calling menu function
    menu();
}
//menu function
pub fn menu(){
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
    //parsing the inputted string into int32
    let numchoice:i32 = match choice.trim().parse(){
        //assign the integer type if the test is ok
        Ok(parsed) => parsed,
        //if theres an error, do this
        Err(_) => {
        //returning 0 in case of error
        0
   }
    };
    //match statement to guide the user to the selected function
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

//resistance calculator function
pub fn calcresistance(){
    println!("----------------------Resistance------------------");
    println!("Enter the number of resistors: ");
    //creating variable string
    let mut _noofres = String::new();
    //input
    io::stdin().read_line(&mut _noofres).expect("Failed to read input");
    //parsing
    let _noofres:i32 = match _noofres.trim().parse(){
        Ok(parse)=>parse,
        Err(_)=>{
            0
        }
    };
    //asking if the resistors are in series or parallel
    println!("Choose if the resistors are in series or parallel: ");
    println!("1.Series");
    println!("2.Parallel");
    println!("Enter your choice: ");
    //creating variable string
    let mut reschoice = String::new();
    //input
    io::stdin().read_line(&mut reschoice).expect("Failed to read input");
    //parsing to int32
    let reschoice:i32 = match reschoice.trim().parse(){
        Ok(parse)=>parse,
        Err(_)=>{
            0
        }
    };
    //taking the user to the specified function based on their choice
    match reschoice{
        1=>formulae::resistanceinseries(_noofres),
        2=>formulae::resistanceinparallel(_noofres),
        _=>println!("invalid choice"),
    }


}
fn calccurrent(){

}
fn calcvoltage(){

}
fn calcpower(){

}
fn calcenergy(){

}