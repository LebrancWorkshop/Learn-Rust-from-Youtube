fn main() {
    // Variable Declaration: let <variable_name> = <expression>;
    let number01 = 1;
    // Section 01: Variable Declaration with no Mutable. 

    // println!(number01); <- Error: Because you can't print a variable without a format string.
    println!("{}", number01);
    println!("Number 01: {}", number01);

    // number01 = 2; <- Error because you can't change the value of a variable that doesn't have mut (mutable) when it's declared. 
    // println!("New Number 01: {}", number01);

    println!("--------");

    // Section 02: Variable Declaration with Mutable.
    let mut number_mut01 = 10;
    println!("Number Mut 01: {}", number_mut01);
    number_mut01 = 20;
    println!("New Number Mut 01: {}", number_mut01);
}
