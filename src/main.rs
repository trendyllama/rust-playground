use std::cmp::Ordering;
mod input;
mod primative_recursive_funcs;
use primative_recursive_funcs::fibonacci;
mod mapping;
use mapping::playing_w_vectors;

fn run_guess_num() {
    loop {

        match input::guess_num() {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You guessed it!"),
        }
    }
}

fn run_capture_input() {
    loop {
        match input::capture_input() {
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

    playing_w_vectors();

    let n = 10;
    println!("Fibonacci of {} is {}", n, fibonacci(n));

}
