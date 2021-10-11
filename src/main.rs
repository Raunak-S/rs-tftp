use std::io::{self, Write};
use std::net::UdpSocket;
use structopt::StructOpt;

#[derive(StructOpt)]
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
    let args = Cli::from_args();

    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket
        .connect(format!("{}:{}", args.destination, args.port))
        .unwrap();

    loop {
        print!("tftp> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");
        let input = input.trim();

        let split_input: Vec<&str> = input.split(' ').collect();

        let cmd =
            Cmd::from_str(split_input[0], split_input[1..split_input.len()].to_vec()).unwrap();

        match cmd {
            Cmd::Get(args) => {
                println!("Received get command");

                // TODO: iterate through all filenames in args instead of using only the first argument
                // TODO: add match case for different args length. i.e. if user enters 1 filename, 2+ filenames, etc
                // TODO: add support for the modes other than octet: netascii and mail

                let mut packet = vec![0u8, 1u8];
                let mut filename = String::from(args[0]);
                filename.push_str("\0");
                let mut mode = String::from("octet");
                mode.push_str("\0");
                packet.extend_from_slice(filename.as_bytes());
                packet.extend_from_slice(mode.as_bytes());
            }
            Cmd::Put(args) => {
                println!("Received put command")
            }
            Cmd::Quit => break,
        }
    }
}
