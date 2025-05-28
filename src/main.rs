#[allow(unused_imports)]
use std::io::{self, Write};

mod builtin;

fn interpret_line(input: String) {
    let cl: Vec<&str> = input.split(' ').collect();

    // let comm = cl[0].trim();
    if !builtin::check_builtins(cl) {
        println!("{}: command not found", input.trim());
    }
}

fn main() {
    loop {
        // Uncomment this block to pass the first stage
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        interpret_line(input);
    }
}
