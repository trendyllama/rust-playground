
use std::io::{stdin};

struct Atom {
    symbol: String,

}

struct Operator {
    symbol: String,
    precedence: u8,

}

struct Expression {

}


pub fn capture_input() -> Result<(), Err> {

    let mut input = String::new();
    println!("Please enter a string:");
    stdin().read_line(&mut input);

    input.trim().to_string();

    println!("You entered: {}", input);

    match input.as_str() {
        "exit" => {
            println!("Exiting...");
            return Ok(());
        }
        _ => {
            println!("You entered: {}", input);
            return Ok(())
        }
    }


}


pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
