macro_rules! haiya {
    ($something:expr) => {
        println!("Haiya!, {}", $something);
    };
}

macro_rules! fuiyoh {
    ($something:expr) => {
        println!("Fuiyoh!, {}", $something);
    };
}

// This Macro is still WIP. 
macro_rules! names {
    ($($name:expr), *) => {
        println!("Hi! {:?}", $name);
    };
}

macro_rules! forloop {
    ($($startnum:expr, $endnum:expr)) => {
        for i in $startnum..$endnum+1 {
            println!("{}", i);
        }
    }
}

fn main() {
   haiya!("You don\'t use MSG. What is this? What is this?");
   fuiyoh!("You use Galangal, Only White People use Ginger.");
   //names!("Alex", "Amin", "Rory"); //<- Error: variable 'name' is still repeating at this depth. 
   forloop!(1, 10);
}
