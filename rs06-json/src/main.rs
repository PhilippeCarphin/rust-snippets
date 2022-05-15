use serde::{Deserialize, Serialize};
/*
 * Demonstrates the basic use of serde_json to parse JSON files into
 * structs that we have defined.
 */
#[derive(Debug, Deserialize, Serialize)]
struct Person {
    name: String,
    age: usize,
    verified: bool,
}

fn main() {
    let json = r#"
        {
          "name": "George",
          "age": 27,
          "verified": false
        }
    "#;

    let json_bad = r#"
        {
          "name": "George",
          "age": 27,
          "perified": false
        }
    "#;

    let person: Result<Person,_> = serde_json::from_str(json);
    if let Ok(person) = person {
        println!("{:?}", person);
    }

    // let person_bad : Person = serde_json::from_str(json_bad).unwrap();
    let person_bad: Result<Person,_> = serde_json::from_str(json_bad);
    if let Ok(person) = person_bad {
        println!("{:?}", person);
    } else if let Err(e) = person_bad {
        println!("Could not parse : {:?}", e);
    }
}
