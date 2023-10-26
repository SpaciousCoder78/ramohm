use std::io;
use crate::menu;



//function for calculating resistance in series
pub fn resistanceinseries(_noofres:i32){
    //series resistance
    println!("---------Resistance In Series-------");
    //declaring total resistance = 0
    let mut totalresistanceinseries: i32 = 0;
    //declaring a counter variable
    let mut count:i32 = 1;
    //for loop that uses no of resistors as range
    for _i in 0.._noofres{
        //resistanceof each series variable
        let mut resistanceofeachseries = String::new();
        //printing resistor count no
        println!("Enter {}th Resistance",count);
        //reading input from user
        io::stdin().read_line(&mut resistanceofeachseries).expect("Failed to take input");
        //parsing string to int32
        let resistanceofeachseries:i32 = match resistanceofeachseries.trim().parse(){
            Ok(parse)=>parse,
            Err(_)=>{
                0
            }
        };
        //adding the resistances
        totalresistanceinseries += resistanceofeachseries;
        //incrementing count by 1 every time the loop ends
        count+=1;
        

    }
    //printing total value of resistance in series
    println!("Resistance in Series is {} Ohms",totalresistanceinseries);
    menu();
    
}
//function for resistance in parallel
pub fn resistanceinparallel(_noofres:i32){
    //series resistance
    println!("---------Resistance In Parallel-------");
    //declaring total resistance = 0
    let mut totalresistanceinparallel: i32 = 0;
    //declaring a counter variable
    let mut count:i32 = 1;
    //for loop that uses no of resistors as range
    for _i in 0.._noofres{
        //resistanceof each series variable
        let mut resistanceofeachparallel = String::new();
        //printing resistor count no
        println!("Enter {}th Resistance",count);
        //reading input from user
        io::stdin().read_line(&mut resistanceofeachparallel).expect("Failed to take input");
        //parsing string to int32
        let resistanceofeachparallel:i32 = match resistanceofeachparallel.trim().parse(){
            Ok(parse)=>parse,
            Err(_)=>{
                0
            }
        };
        //adding the resistances
        totalresistanceinparallel += 1/resistanceofeachparallel;
        //incrementing count by 1 every time the loop ends
        count+=1;
        

    }
    //printing total value of resistance in series
    println!("Resistance in Parallel is {} Ohms",totalresistanceinparallel);
    menu();
}