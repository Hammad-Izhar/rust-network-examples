use rand;
use rust_sockets::Message;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

fn send_message(socket: &mut TcpStream, message: Message) {
    match socket.write(&message.marshal()) {
        Ok(_) => (),
        Err(e) => eprintln!("Failed to write response to client: {}", e),
    }
}

fn handle_response(mut socket: TcpStream, secret_mutex: Arc<Mutex<u16>>) {
    loop {
        let mut buffer: Vec<u8> = vec![0; 3];
        match socket.read(&mut buffer) {
            Err(e) => {
                eprintln!("Failed to read response from socket: {}", e);
                continue;
            }
            Ok(0) => continue,
            Ok(_) => (),
        };

        let secret = *secret_mutex.lock().unwrap();
        match Message::unmarshal(&buffer) {
            Some(Message::Guess(x)) if x == secret => {
                send_message(&mut socket, Message::GuessCorrect);
            }
            Some(Message::Guess(x)) if x < secret => {
                send_message(&mut socket, Message::GuessTooLow)
            }
            Some(Message::Guess(x)) if x > secret => {
                send_message(&mut socket, Message::GuessTooHigh)
            }
            _ => eprintln!("Got unexpected message! {:?}", buffer),
        }
    }
}
fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <address> <port number>", args.get(0).unwrap());
        std::process::exit(1);
    }

    let port_number = &args[1];
    let socket = match TcpListener::bind(format!("localhost:{}", port_number)) {
        Ok(x) => x,
        Err(e) => panic!("Error starting server {}", e),
    };

    println!("Server started and listening at localhost:{}!", port_number);
    let secret_number = Arc::new(Mutex::from(rand::random::<u16>()));
    {
        let secret = *secret_number.lock().unwrap();
        println!("Shhh! The secret number is {}", secret);
    }

    for stream in socket.incoming() {
        match stream {
            Ok(s) => {
                let cloned_number = secret_number.clone();
                std::thread::spawn(move || handle_response(s, cloned_number));
            }
            Err(e) => {
                eprintln!("Failed to connect to incoming client: {}", e);
            }
        }
    }
}
