// Lab: Data Type in Rust 
// Link: https://doc.rust-lang.org/book/ch03-02-data-types.html 

fn main() {
    // Variable Declaration with Data Type: let <variable_name>: <data_type> = <sxpression>;

    // Scalar Data Type: Data Type that represent the value of variable in "Single Value".
    // E.g. Interger (Number), Floating-Point (Number), Characters, and Boolean. 
    // Integer devided into two parts: Signed and Unsigned Interger. 
    // Integer Data Type: i8, i16, i32, i64, i128 (Signed Integer: Have an negative integer.) / u8, u16, u32, u64, u128 (Unsigned Integer: Only have a positive integer.)
    // Floating-Point Data Type: f32, f64 (Floating-Point Number: Have a decimal point.)
    
    let num = 10; // Default Type of Number is i32. (equal to "let num: i32 = 10;")
    let num_int_32: i32 = 10; 
    println!("Num: {}", num);
    println!("Num (Integer 32 bits): {}", num_int_32);

    let num_signed: i8 = -1;
    // let num_unsigned: u8 = -1; <- Error: Cannot assign the negative number to the unsigned integer

    let num_unsigned: u8 = 1;

    println!("Singed Integer: {}", num_signed);
    println!("Unsigned Integer: {}", num_unsigned);
}
