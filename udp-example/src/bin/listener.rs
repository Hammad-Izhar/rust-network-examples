use std::net::UdpSocket;

const MAX_MESSAGE_SIZE: u16 = 1400;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <port>", &args[0])
    }

    let port = &args[1];
    let socket = UdpSocket::bind(format!("localhost:{}", port)).expect("Error resolving address");

    loop {
        let mut buffer = vec![0; MAX_MESSAGE_SIZE.into()];
        let (number_of_bytes, address) = socket
            .recv_from(&mut buffer)
            .expect("Error reading from UDP socket");

        let message = String::from_utf8(buffer).expect("Error decoding buffer");
        println!(
            "Received {} bytes from {}: {}",
            number_of_bytes,
            address.to_string(),
            message
        )
    }
}
