use interpreter;

fn main() {

    interpreter::guess_num();
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
