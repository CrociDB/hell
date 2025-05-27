#[allow(unused_imports)]
use std::io::{self, Write};

fn commands_exit(line: Vec<&str>) {
    if line.len() < 2 {
        std::process::exit(1);
    }

    let value: i32 = match line[1].trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!(
                "Error: Unable to parse exit parameter '{}' as an integer",
                line[1]
            );
            return;
        }
    };

    std::process::exit(value);
}

fn commands_echo(line: Vec<&str>) {
    if line.len() < 2 {
        println!("");
    }

    let joined: String = line[1..].join(" ");
    println!("{}", joined.trim());
}

fn interpret_line(input: String) {
    let cl: Vec<&str> = input.split(' ').collect();

    let comm = cl[0].trim();
    if comm == "exit" {
        commands_exit(cl);
    } else if comm == "echo" {
        commands_echo(cl);
    } else {
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
