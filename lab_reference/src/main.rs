// Lab: Reference in Rust. 
// Hard to Understand...

fn main() {
    let mut x = 10;
    let ref_x = &x;

    let mut change_num = 100;
    let change_num_ref = &mut change_num;
    *change_num_ref += 1;

    println!("{}", x); // Output: 10
    println!("{}", ref_x); // Output: 10
    println!("{}", change_num); // Output: 101
}
