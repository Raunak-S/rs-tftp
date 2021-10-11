use std::io::{self, Write};

struct Cli {
    destination: String,
    port: String,
}

enum Cmd<'a> {
    Get(Vec<&'a str>),
    Put(Vec<&'a str>),
    Quit,
}

impl<'a> Cmd<'a> {
    fn from_str(command: &str, args: Vec<&'a str>) -> Result<Self, &'static str> {
        match command {
            "put" => Ok(Cmd::Put(args)),
            "get" => Ok(Cmd::Get(args)),
            "quit" => Ok(Cmd::Quit),
            _ => Err("?Invalid command"),
        }
    }
}

fn main() {
    loop {
        print!("tftp> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");

        let split_input: Vec<&str> = input.split(' ').collect();

        let cmd = Cmd::from_str(split_input[0], split_input[1..split_input.len()].to_vec()).unwrap();

        match cmd {
            Cmd::Get(args) => {
                println("Received get command");

                // TODO: add match case for different args length. i.e. if user enters 1 filename, 2+ filenames, etc

            },
            Cmd::Put(args) => ,
            Quit => break
        }
    }
}
