#[allow(unused_imports)]
use std::io::{self, Write};

mod builtin;
mod defs;
mod exec;

fn interpret_line(input: String) {
    let cl: Vec<&str> = input.split(' ').collect();

    // let comm = cl[0].trim();
    if let Err(e) = builtin::check_builtins(&cl).or_else(|_| exec::check_exec(&cl)) {
        match e {
            defs::CheckerError::NotFound => println!("{}: command not found", input.trim()),
            defs::CheckerError::Io(ioe) => println!("IO Error: {}", ioe),
            defs::CheckerError::Other(se) => println!("Error: {}", se),
        }
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
