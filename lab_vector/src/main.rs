fn main() {
    let v1 = vec![1, 2, 3, 4, 5];
    let mut v2 = vec![];

    for v in v1 {
        if is_even(v) {
            v2.push(v);
        }
    }

    println!("Vector 2: {:?}", v2);
}

fn is_even(v: i32) -> bool { // Do not reference the value, because it is a primitive type.
    v % 2 == 0
}

