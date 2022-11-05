fn main() {
    let tup01 = (1, 2, 3, 4, 5);
    let tup02 = (1, "Hello", 3, "World", 5, false);

    println!("Tuple 01: {:?}", tup01);
    println!("Tuple 02: {:?}", tup02);

    println!("Index 0 of Tuple 01: {}", tup01.0);

    // We can access element in tuple of tuple by: <tuple_name>.<index_of_tuple>.<index_of_element> or (<tuple_name>.<index_of_tuple>).<index_of_element>
    let tup03 = ((1,2,3), 10, true);
    println!("{}", (tup03.0).1);

    // We can "Deconstruct" the tuple. 
    let tup_avenger = ("Captain America", 10, 2010);
    let (name, age, year) = tup_avenger;
    println!("{} {} {}", name, age, year);
}
