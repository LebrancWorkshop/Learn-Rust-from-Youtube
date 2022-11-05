// Lab: Constant in Rust. 

// Constant Declaration: const <VARIABLE_NAME>: <data_type> = <expression>;
const PI: f32 = 3.14159265359;
const MAX_NUM: u8 = 10;

fn main() {
    println!("PI: {}", PI); // Output: 3.1415927 <- Quite Interesting !? 

    println!("--------");

    for i in 1..MAX_NUM+1 {
        println!("Number: {}", i);
    }
}
