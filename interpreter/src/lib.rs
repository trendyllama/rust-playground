
use rand::Rng;
use std::cmp::Ordering;
use std::io;


pub fn guess_num() -> Ordering{

    // let mut input: String = String::new();
    let secret_number = rand::rng().random_range(1..=100);

    let mut input = String::new();
    println!("Please enter a number between 1 and 100:");


    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let out = input.trim().parse::<u32>().unwrap();

    return out.cmp(&secret_number);


}

pub fn capture_input() -> Option<String> {

    let mut input: String = String::new();
    println!("Please enter a string:");

    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    match input.trim() {
        "exit" => {
            return None;
        }
        _ => {
            return Some(input)
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
