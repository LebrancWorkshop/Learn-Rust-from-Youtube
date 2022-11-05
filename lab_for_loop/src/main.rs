// Lab: For Loop in Rust. 

fn main() {
    // Goal: Loop from Start Number to Finish Number. 
    let start_num = 1;
    let finish_num = 10;

    let range_of_num = start_num..finish_num+1;

    let marvel_heroes = vec!["Iron Man", "Captain America", "Hulk", "Ant Man", "Captain Marvel", "Hawkeye", "Wolverine"];

    // for <loop_variable> in <start_number>..<finish_number + 1> {<code_block>}
    // Note: Loop Variable can instantly be initiated in for loop statement. 

    // First Way: Init Range in For Loop Statement. 
    for loop_num in start_num..finish_num+1 {
        println!("For Loop Number: {}", loop_num);
    }

    println!("--------");

    // Second Way: Init Range in Variable and use it in the for loop statement. 
    for loop_range_num in range_of_num {
        println!("For Loop Range Number: {}", loop_range_num);
    }

    println!("--------");

    // Third Way: Init Vector Variable and use it as a range in the for loop statement. 
    for loop_marvel_hero in marvel_heroes.iter() { // Use .iter() to still have an ownership in the vector variable.
        println!("Marvel Hero: {}", loop_marvel_hero);
    }

    println!("--------");

    // Fourth Way: Init Vector Variable and use it as a range in the for loop statement and get all Index and Value of the vector variable. 
    for (index, loop_marvel_hero) in marvel_heroes.iter().enumerate() {
        println!("Marvel Hero Index {}: {}", index, loop_marvel_hero);
    }
}
