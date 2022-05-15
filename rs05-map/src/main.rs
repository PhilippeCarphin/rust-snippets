use std::collections::HashMap;
/*
 * Basic demonstration of the use of HashMap.
 *
 * Note that the types of the keys and values have to be constant
 * and also that the map can determine its types from the first
 * call to insert.
 */
fn main() {
    // https://doc.rust-lang.org/book/ch08-03-hash-maps.html

    let mut numbers = HashMap::new();

    numbers.insert(String::from("twelve"), 12);
    numbers.insert(String::from("five"), 5);

    let mut letters = HashMap::new();

    letters.insert(String::from("double you"), String::from("U"));
    letters.insert(String::from("queue"), String::from("Q"));

    println!("=========== letters ========================");
    for (k,v) in letters {
        println!("\x1b[1;31m{}\x1b[0m: \x1b[1;33m{}\x1b[0m", k, v);
    }
    // println!("letters = {}", letters);
    println!("=========== numbers ========================");
    println!("Hello, world!");
    for (k,v) in numbers {
        println!("{}: {}", k, v);
    }
}
