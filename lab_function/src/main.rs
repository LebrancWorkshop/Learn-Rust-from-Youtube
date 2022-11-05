fn main() {
    print_num(10);
    println!("{}", return_num(10));
}

fn print_num(num: i64) {
    println!("{}", num);
}

// Return Function: fn <function_name>(<argument_name>:<argument_data_type>) -> <return_value_data_type> {<code_block>}
fn return_num(num: i64) -> i64 {
    return num;
}