use std::cmp::Ordering;
use interpreter;

fn run_guess_num() {
    loop {

        match interpreter::guess_num() {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You guessed it!"),
        }
    }
}

fn run_capture_input() {
    loop {
        match interpreter::capture_input() {
            Some(_) => {
                println!("Input captured.");
            }
            None => {
                println!("Exiting...");
                break;
            }
        }
    }
}

fn main() {



    run_capture_input();

}
