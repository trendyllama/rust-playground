use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn guess_num() -> Ordering {
    let secret_number = rand::rng().random_range(1..=100);

    let mut input = String::new();
    println!("Please enter a number between 1 and 100:");

    let out = match io::stdin().read_line(&mut input) {
        Ok(_) => input.trim().parse::<u32>(),
        Err(why) => panic!("Invalid Integer Input Entered: {why}"),
    };

    match out {
        Ok(out) => out.cmp(&secret_number),
        Err(_why) => panic!("Invalid Int Entered!"),
    }
}

pub fn capture_input() -> Option<String> {
    let mut input: String = String::new();
    println!("Please enter a string:");

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.trim() {
        "exit" => None,
        _ => Some(input),
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {}
}
