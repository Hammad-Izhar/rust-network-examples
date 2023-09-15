use std::net::UdpSocket;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <address> <port> <text>", args[0]);
    }

    let [address, port, message] = &args[1..=3] else {
        panic!("Failed to destructure command line arguments");
    };

    let socket = UdpSocket::bind(format!("{}:{}", address, 0)).expect("Error binding to socket");
    socket
        .connect(format!("{}:{}", address, port))
        .expect("Error connecting to remote address");

    let bytes_written = socket
        .send(message.as_bytes())
        .expect("Error sending message");

    println!("Sent {} bytes", bytes_written);
}
