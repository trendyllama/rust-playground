

use  serde::{Deserialize, Serialize};
use serde_json;


#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,

}

impl Person {

    fn new(name: String, age: u8) -> Self {
        Person { name, age }
    }

    fn greet(&self) {
        println!("Hello, my name is {} and I am {} years old.", self.name, self.age);
    }


    fn serialize(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    // Implement deserialization if needed
    fn deserialize(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

}

macro_rules! greet {
    () => {
        println!("Greetings from the macro!");
    };
}

macro_rules! hello_world {
    () => {
        println!("Hello, world!")
    };
}

fn main() {
    hello_world!();

    let person = Person::new(String::from("Alice"), 30);
    person.greet();

    match person.serialize() {
        Ok(json) => println!("Serialized Person: {}", json),
        Err(e) => println!("Serialization error: {}", e),
    }

}
