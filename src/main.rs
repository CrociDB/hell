#[allow(unused_imports)]
use std::io::{self, Write};

use hell::CommandHandle;

mod builtin;
mod exec;
mod hell;

fn interpret_line(input: String) -> hell::CommandHandle {
    let cl: Vec<&str> = input.split(' ').map(|c| c.trim()).collect();

    match builtin::check_builtins(&cl).or_else(|_| exec::check_exec(&cl)) {
        Ok(comm) => comm,
        Err(e) => {
            match e {
                hell::CheckerError::NotFound => println!("{}: command not found", input.trim()),
                hell::CheckerError::Io(ioe) => println!("IO Error: {}", ioe),
                hell::CheckerError::Other(se) => println!("Error: {}", se),
            };
            CommandHandle {
                child: None,
                ret: Some(-1),
            }
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
        let mut handle = interpret_line(input);

        // This will wait until the process is finally over
        if let Some(mut child) = handle.child {
            let child_status = child.wait();
            if let Ok(status) = child_status {
                handle.ret = status.code();
            }
        }
    }
}
