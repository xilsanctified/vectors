// Build a practice module to test knowledge

mod something {
    pub mod public {
        pub fn make_vector() -> Vec<Option<i32>> {
            let v: Vec<Option<i32>> = Vec::new();
            v
        }
        pub fn is_it_null(x: &Option<i32>) -> bool {
            match x {
                Some(_i) => false,
                None => true,
            }
        }
        pub fn output(x: &Option<i32>) -> String {
            match x {
                Some(i) => i.to_string(),
                None => String::from("None"),
            }
        }
    }
}

// Practice scope
use self::something::public;

fn main() {
    // practice using module function
    let mut v = public::make_vector();

    v.push(Some(1));
    v.push(None);
    v.push(Some(1));
    v.push(Some(5));

    // using vector macro which ingests values and infers a type
    // in this case, &str.
    let mut another_v = vec!["vectors", "are", "cool"];
    another_v.push("!!!");

    let third = &v[2];
    println!("Value null: {}", public::is_it_null(&third));

    match v.get(2) {
        Some(third) => println!("The third element: {}", public::output(third)),
        None => println!("There is no third element!"),
    }
    for x in another_v {
        print!("{} ", &x);
    }
    print!("\n");

    let mut numbers = Vec::new();

    for i in 1..=10 {
        numbers.push(i);
    }

    // Example of using a dereference operator (*)
    for i in &mut numbers {
        *i *= 10;
        println!("Number: {}", i);
    }

    // Using Enum to store multiple types in a vector

    enum MultiVar {
        Int(i32),
        Text(String),
        Bool(bool),
    }

    let row = vec![
        MultiVar::Int(3),
        MultiVar::Text(String::from("String")),
        MultiVar::Bool(false),
    ];

    for x in row {
        match x {
            MultiVar::Int(i) => println!("Value: {}", i),
            MultiVar::Text(s) => println!("Value: {}", s),
            MultiVar::Bool(b) => println!("Value: {}", b),
        }
    }
}

// Notes:
// Vectors can only store values of the same type.
