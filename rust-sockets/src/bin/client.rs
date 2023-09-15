use rust_sockets::Message;
use std::io::{self, Read, Write};
use std::net::TcpStream;

fn handle_response(mut socket: TcpStream) {
    loop {
        let mut buffer: Vec<u8> = vec![0; 3];
        match socket.read(&mut buffer) {
            Err(e) => eprint!("Failed to read response from socket: {}", e),
            Ok(0) => continue,
            Ok(_) => (),
        }

        match Message::unmarshal(&buffer) {
            Some(Message::GuessTooLow) => println!("Too low!"),
            Some(Message::GuessCorrect) => {
                println!("You got it right!");
                std::process::exit(0);
            }
            Some(Message::GuessTooHigh) => println!("Too high!"),
            _ => eprintln!("Received invalid reply, {:?}", buffer),
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <address> <port number>", args.get(0).unwrap());
        std::process::exit(1);
    }

    let [address, port_number] = &args[1..=2] else {
         panic!("Couldn't destructure address and port_number")
        };

    let mut socket = match TcpStream::connect(format!("{}:{}", address, port_number)) {
        Ok(x) => x,
        Err(e) => panic!("Error connecting: {}", e),
    };

    println!("Connected to {}:{}!", address, port_number);

    let cloned_socket = socket.try_clone().unwrap();
    std::thread::spawn(move || handle_response(cloned_socket));

    loop {
        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer) {
            Ok(_) => (),
            Err(e) => eprintln!("Unable to read from stdin: {}", e),
        };
        let buffer = buffer.trim();

        let guess = match buffer.parse::<u16>() {
            Ok(x) => x,
            Err(_) => {
                println!("Invalid guess: {}", buffer);
                continue;
            }
        };

        match socket.write(&Message::Guess(guess).marshal()) {
            Ok(_) => (),
            Err(e) => eprintln!("Failed to write to socket: {}", e),
        };
    }
}
