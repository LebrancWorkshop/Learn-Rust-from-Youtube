// Lab: Array in Rust. 

fn main() {
    let array_num = [1, 2, 3, 4, 5];
    println!("{}", array_num[0]);

    println!("--------");

    for num in array_num.iter() {
        println!("{}", num);
    }

    println!("--------");

    for index in 0..array_num.len() {
        println!("Index: {}\nNum: {}\n\n", index, array_num[index]);
    }

    println!("--------");
    let repeat_num = [99; 5];

    for num in repeat_num.iter() {
        println!("{}", num);
    }
}
