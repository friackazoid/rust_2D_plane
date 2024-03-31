mod point;
mod point_manager;

use point_manager::PointManager;

use std::net::{UdpSocket, SocketAddr};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {

    #[clap(short, long, default_value = "10")]
    number: usize,

    #[clap(short, long, default_value = "8080")]
    port: u16,
}


fn main() -> std::io::Result<()> {

    let args = Args::parse();

    println!("Number of points: {}", args.number);
    let pm = PointManager::new(args.number);
    pm.print_points();

    let socket = UdpSocket::bind("127.0.0.1:8080")?;

    println!("Server listening on 127.0.0.1:8080");

    // Buffer to hold incoming data
    let mut buf = [0; 1024];

    loop {
        // Receive data from the socket
        let (amt, src) = socket.recv_from(&mut buf)?;
        let received_data = &buf[..amt];

         // Print the received data
        let received_str = std::str::from_utf8(received_data).unwrap();
        socket.send_to("accepted".as_bytes(), src)?;

        println!("Received data from {}: {}", src, received_str);

         let parts: Vec<&str> = received_str.split_whitespace().collect();
         println!("Received parts: {}, {}, {}", parts[0], parts[1], parts[2]);
    
    }
}
