// Lab: Data Type in Rust 
// Link: https://doc.rust-lang.org/book/ch03-02-data-types.html 

/**
 * Output:
        Min Integer 8 bits: -128
        Max Integer 8 bits: 127

        Min Integer 16 bits: -32768
        Max Integer 16 bits: 32767

        Min Integer 32 bits: -2147483648
        Max Integer 32 bits: 2147483647

        Min Integer 64 bits: -9223372036854775808
        Max Integer 64 bits: 9223372036854775807

        Min Integer 128 bits: -170141183460469231731687303715884105728
        Max Integer 128 bits: 170141183460469231731687303715884105727

        **********

        Min Unsigned Integer 8 bits: 0
        Max Unsigned Integer 8 bits: 255

        Min Unsigned Integer 16 bits: 0
        Max Unsigned Integer 16 bits: 65535

        Min Unsigned Integer 32 bits: 0
        Max Unsigned Integer 32 bits: 4294967295

        Min Unsigned Integer 64 bits: 0
        Max Unsigned Integer 64 bits: 18446744073709551615
        
        Min Unsigned Integer 128 bits: 0
        Max Unsigned Integer 128 bits: 340282366920938463463374607431768211455
 */

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

    println!("--------");

    let MIN_INT8 = std::i8::MIN;
    let MAX_INT8 = std::i8::MAX;

    let MIN_INT16 = std::i16::MIN;
    let MAX_INT16 = std::i16::MAX;

    let MIN_INT32 = std::i32::MIN;
    let MAX_INT32 = std::i32::MAX;

    let MIN_INT64 = std::i64::MIN;
    let MAX_INT64 = std::i64::MAX;

    let MIN_INT128 = std::i128::MIN;
    let MAX_INT128 = std::i128::MAX;

    let MIN_UINT8 = std::u8::MIN;
    let MAX_UINT8 = std::u8::MAX;

    let MIN_UINT16 = std::u16::MIN;
    let MAX_UINT16 = std::u16::MAX;

    let MIN_UINT32 = std::u32::MIN;
    let MAX_UINT32 = std::u32::MAX;

    let MIN_UINT64 = std::u64::MIN;
    let MAX_UINT64 = std::u64::MAX;

    let MIN_UINT128 = std::u128::MIN;
    let MAX_UINT128 = std::u128::MAX;

    println!("Min Integer 8 bits: {}", MIN_INT8);
    println!("Max Integer 8 bits: {}", MAX_INT8);

    println!("Min Integer 16 bits: {}", MIN_INT16);
    println!("Max Integer 16 bits: {}", MAX_INT16);

    println!("Min Integer 32 bits: {}", MIN_INT32);
    println!("Max Integer 32 bits: {}", MAX_INT32);

    println!("Min Integer 64 bits: {}", MIN_INT64);
    println!("Max Integer 64 bits: {}", MAX_INT64);

    println!("Min Integer 128 bits: {}", MIN_INT128);
    println!("Max Integer 128 bits: {}", MAX_INT128);

    println!("Min Unsigned Integer 8 bits: {}", MIN_UINT8);
    println!("Max Unsigned Integer 8 bits: {}", MAX_UINT8);

    println!("Min Unsigned Integer 16 bits: {}", MIN_UINT16);
    println!("Max Unsigned Integer 16 bits: {}", MAX_UINT16);

    println!("Min Unsigned Integer 32 bits: {}", MIN_UINT32);
    println!("Max Unsigned Integer 32 bits: {}", MAX_UINT32);

    println!("Min Unsigned Integer 64 bits: {}", MIN_UINT64);
    println!("Max Unsigned Integer 64 bits: {}", MAX_UINT64);

    println!("Min Unsigned Integer 128 bits: {}", MIN_UINT128);
    println!("Max Unsigned Integer 128 bits: {}", MAX_UINT128);

}

