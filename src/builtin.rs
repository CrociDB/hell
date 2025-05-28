use crate::defs;

struct BuiltinCommand {
    command: &'static str,
    func: fn(&Vec<&str>),
}

const BUILTIN_COMMANDS: &[BuiltinCommand] = &[
    BuiltinCommand {
        command: "echo",
        func: builtin_echo,
    },
    BuiltinCommand {
        command: "exit",
        func: builtin_exit,
    },
    BuiltinCommand {
        command: "type",
        func: builtin_type,
    }
];

fn builtin_echo(line: &Vec<&str>) {
    if line.len() < 2 {
        println!(" ");
    }

    let joined: String = line[1..].join(" ");
    println!("{}", joined.trim());
}

fn builtin_exit(line: &Vec<&str>) {
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

fn builtin_type(line: &Vec<&str>) {
    if line.len() < 2 {
        eprintln!("Error: command `type` requirest one parameter");
        return;
    }

    let mut builtin_found = false;
    for builtin in BUILTIN_COMMANDS {
        if line[1].trim() == builtin.command {
            println!("{} is a shell builtin", builtin.command);
            builtin_found = true;
            break;
        }
    }

    if !builtin_found {
        println!("{}: not found", line[1].trim());
    }
}

pub fn check_builtins(line: &Vec<&str>) -> Result<(), defs::CheckerError> {
    for builtin in BUILTIN_COMMANDS {
        let comm = line[0].trim();
        if comm == builtin.command {
            (builtin.func)(line);
            return Ok(());
        }
    }

    Err(defs::CheckerError::NotFound)
}
