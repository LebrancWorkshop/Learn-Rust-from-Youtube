// Lab: Condition in Rust. 

n main() {
    let num01 = 102;
    let correct_num = 99;
    
    if num01 == correct_num {
        println!("Correct!");
    } else if num01 < correct_num {
        println!("Too Low!");
    } else if num01 > correct_num {
        println!("Too High!");
    } else {
        println!("Something Wrong!");
    }
}
