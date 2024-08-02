use std::io;

fn main(){
    println!("you have been taken to the first security check of latvaria immigration");
    loop {
        println!("enter you name to create the profile with you age, name and purpose to stay")

        let mut NAME = String::new()
        let mut AGE: u32
        io::stdin()
            .read_line(&mut NAME)

        println!("your name is {NAME}")
    }
}