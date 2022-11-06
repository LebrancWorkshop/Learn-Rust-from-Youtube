// Lab: Shadowing in Rust. 
// Shadowing maybe means assign the same variable name (by "let") with previous variable and the new variable will have a new value.

fn main() {
    let mut mut_num_1 = 10;
    let mut mut_num_2 = 20;

    {
       let mut_num_1 = 11;
    }

    println!("Mutable Number 1: {}", mut_num_1); // Output: 10
    println!("Mutable Number 2: {}", mut_num_2); // Output: 20
    
    let mut_num_2 = 21;
    println!("Mutable Number 1: {}", mut_num_1); // Output: 10
    println!("Mutable Number 2: {}", mut_num_2); // Output: 21
}
