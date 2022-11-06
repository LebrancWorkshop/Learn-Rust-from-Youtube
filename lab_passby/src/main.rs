// Lab: Pass by Value and Reference in Rust. 

// Pass by Reference can be effectively on some type like Struct. 
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

fn main() {
    println!("Hello, world!");
    
    let num: u32 = 90;
    get_num(num); // Output: 90
    get_num(num); // Output: 90
    
    let bg_color = Color{
        red: 100,
        green: 200,
        blue: 250
    };
    
    print_color(&bg_color);
    print_color(&bg_color);
}

fn get_num(num: u32) {
    println!("{}", num);
}

fn print_color(c: &Color) {
    println!("RGB({}, {}, {})", c.red, c.green, c.blue);
}
