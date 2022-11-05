// Lab: Enumerate (Enum) in Rust.

// enum <enum_name> {<enum_variant_1>, <enum_variant_2>, <enum_variant_3>, ..., <enum_variant_final>}

enum Weapon {
    Sword,
    Axe,
    Spear,
    Bow,
    Staff,
}

fn main() {
    // Variable Declaration with Enum: let <variable_name>: <enum_name> = <enum_name>::<enum_variant>;
    let player_weapon: Weapon = Weapon::Sword;
    //println!("{}", player_weapon); <- Error: `Weapon` doesn't implement `std::fmt::Display`

    // match <variable_name_declare_with_enum> {<enum_name>::<enum_variant_1> => <expression>; <enum_name>::<enum_variant_2> => <expression>; <enum_name>::<enum_variant_3> => <expression>; ..., <enum_name>::<enum_variant_final> => <expression>;}
    match player_weapon {
        Weapon::Sword => println!("Player Weapon: Sword"),
        Weapon::Axe => println!("Player Weapon: Axe"),
        Weapon::Spear => println!("Player Weapon: Spear"),
        Weapon::Bow => println!("Player Weapon: Bow"),
        Weapon::Staff => println!("Player Weapon: Staff")
    }

    // Note: We can assign a value into the enum. 
}
