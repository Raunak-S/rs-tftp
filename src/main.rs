use byteorder::{BigEndian, ReadBytesExt};
use std::fs::File;
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

fn get_opcode(packet: &[u8]) -> u16 {
    let mut slice = &packet[..2];
    slice.read_u16::<BigEndian>().unwrap()
}

fn get_block_num(packet: &[u8]) -> u16 {
    let mut slice = &packet[2..4];
    slice.read_u16::<BigEndian>().unwrap()
}

// TODO: find a better way to read data up to, but not including, the null terminator
fn get_data(packet: &[u8]) -> String {
    let mut data = String::from_utf8(packet[4..].to_vec()).unwrap();
    match data.find('\0') {
        None => data,
        Some(index) => {
            let _ = data.split_off(index);
            data
        }
    }
}

fn send_read_packet(args: &Cli, socket: &UdpSocket, file: &String, mode: &String) {
    let mut packet = vec![0u8, 1u8];
    let filename = &*file; 
    filename.push_str("\0");
    mode.push_str("\0");
    packet.extend_from_slice(filename.as_bytes());
    packet.extend_from_slice(mode.as_bytes());

    socket
        .send_to(&packet, format!("{}:{}", args.destination, args.port))
        .unwrap();
}

fn main() {
    let args = Cli::from_args();

    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();

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
            Cmd::Get(argv) => {
                println!("Received get command");

                // TODO: iterate through all filenames in args instead of using only the first argument
                // TODO: add match case for different args length. i.e. if user enters 1 filename, 2+ filenames, etc
                // TODO: add support for the modes other than octet: netascii and mail

                let filename = String::from(argv[0]);
                let mode = String::from("octet");
                send_read_packet(&args, &socket, &filename, &mode); 

                filename.pop().unwrap();
                let mut file = File::create(filename).unwrap();

                loop {
                    let mut buf = [0u8; 512];
                    let (amt, src) = socket.recv_from(&mut buf).unwrap();

                    file.write_all(get_data(&buf).as_bytes()).unwrap();

                    let mut ack = [0u8, 4u8, 0u8, 0u8];
                    let mut i = 2;
                    for byte in get_block_num(&buf).to_be_bytes() {
                        ack[i] = byte;
                        i += 1;
                    }

                    socket.send_to(&ack, src).unwrap();

                    if amt < 512 {
                        break;
                    };
                }
            }
            Cmd::Put(argv) => {
                println!("Received put command");


            }
            Cmd::Quit => break,
        }
    }
}
