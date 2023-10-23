use std::io;

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
    
}
//function for resistance in parallel
pub fn resistanceinparallel(_noofres:i32){

}