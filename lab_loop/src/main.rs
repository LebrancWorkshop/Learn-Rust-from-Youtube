// Lab: Loop in Rust. 

fn main() {
    // Section 1: Infinite Loop. 
    // let mut loop_num_1 = 1;

    // loop {
        // loop_num_1+= 1;
        // println!("Loop Number: {}", loop_num_1;
    // }

    // Section 2: Infinite Loop with Break.
    // let mut loop_num_2 = 0;

    // loop {
    //     loop_num_2 += 1;

    //     // Condition is working when the loop is running and the condition is true.
    //     if loop_num_2 > 10 {
    //         break;
    //     }

    //     println!("Loop Number: {}", loop_num_2);
    // }

    // Section 3: Infinite Loop with Condition. 
    let mut loop_num_3 = 0;

    loop {
        loop_num_3 += 1;

        if loop_num_3 == 5 {
            continue; // The Loop still continue working when pass this condition but it will skip the present loop when the condition is true and run the next loop. 
        }

        if loop_num_3 > 10 {
            break;
        }

        println!("Loop Number: {}", loop_num_3);
    }
}
