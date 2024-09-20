fn main() {
    let s1 = String::from("Acho!");
    // let s2 = s1; -> Ownership from s1 was moved to s2, So we cannot do something with s1 anymore.
    let s2 = s1.clone(); // Use clone to copy the value of s1 to s2, So the owner of s1 is still there.
    println!("{}", s1);
    println!("{}", s2);

    let name = String::from("Lebranc Convas");
    // let name_length = name.len();
    let name_ref = &name[1..name.len()];
    println!("{name_ref}");
}
