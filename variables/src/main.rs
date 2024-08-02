// use std::io;
use num::pow::pow;


fn main() {
    // const MAX_POINTS: u32 = 20_000;
    let x: u32 = 5;

    println!("the value of x is {}", x);

    //? differences between shoadowing previous variable vs creating a
    //? mutable varibale

    let y = 78;

    println!("the value of y is {}", y);

    let y: u64 = pow(y,8);

    println!("the value of shadowing y is {}",y);

    // we are creating a new variable with the same name of y which changes 
    // the orignal value of y, this is known as shadowing a variable

    //? now we create a single mutable instance of a variable instead making 
    //? multiple let statements

    let mut g = 34;

    println!("the value of g is {}", g);

    g = g * 45;

    println!("the value of g is {}", g);

    //? the differeneces includes
    
    //todo      let allows you change all the aspects of a variable along with the 
    //todo      data type and all the aspects like u32 and u64 in the integer
    //todo      this also allows you to once again make the variable immutable

    //todo      but instead if you chose to opt for creating a single instance of the
    //todo      mutable variableit does not let you change the data types or its instances
    //todo      and just the value 
    

    //? another example from the official rust book

    let spaces = "   ";

    
    println!("spaces is {}.", spaces);

    let spaces = spaces.len();

    println!("spaces is {}", spaces);
}
