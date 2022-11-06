// Lab: Struct in Rust. 

// Struct Declaration: struct <Struct_Name> {<key_1>: <type_of_key_1>, <key_2>: <type_of_key_2>, <key_3>: <type_of_key_3>, ..., <key_final>: <type_of_key_final>}
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// Struct Tuple: struct <Struct_Name>(<key_1_data_type>, <key_2_data_type>, <key_3_data_type>, ..., <key_final_data_type>);
struct Color2(u8, u8, u8);

fn main() {
    // Section 1: Playing with normal Struct. 
    let code_color = Color{
        red: 200,
        green: 100,
        blue: 50
    };

    println!("Red: {}", code_color.red);
    println!("Green: {}", code_color.green);
    println!("Blue: {}", code_color.blue);

    println!("--------");
    
    // Section 2: Playing with Struct Tuple.
    let bg_color = Color2(100, 200, 250);

    println!("Red2: {}", bg_color.0);
    println!("Green2: {}", bg_color.1);
    println!("Blue2: {}", bg_color.2);
}
