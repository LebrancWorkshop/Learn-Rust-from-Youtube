// Lab: While Loop in Rust. 

fn main() {
    // Goal: Loop from Start Number to Finish Number. 
    let start_num = 1;
    let finish_num = 10;
    let mut num_while_loop = start_num;

    // while <condition> {<code_block>}
    while num_while_loop <= finish_num {
        println!("While Loop Number: {}", num_while_loop);
        num_while_loop += 1;
    }
}
