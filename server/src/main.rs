
// Imports
use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;

const ADDRESS: &str = "127.0.0.1:8080";
const BUFFER: usize = 32; // buffer size for reading messages

// a short delay in the loop
fn sleep() {
    thread::sleep(::std::time::Duration::from_millis(100));
}


fn main() {
    let server = TcpListener::bind(ADDRESS).expect("Listener failes to bind");

    // set the server to non-blocking mode to handle multiple connections
    server
        .set_nonblocking(true)
        .expect("Failed to init non-blocking");
    let mut clients = vec![];
    let (tx, rx) = mpsc::channel::<String>();

    loop {
        if let Ok((mut socket, addr)) = server.accept() {
            println!("Client {} connected", addr);

            // clone the sender end of the channel for the new clients
            let tx = tx.clone();

            clients.push(socket.try_clone().expect("Failed to clone client"));

            thread::spawn(move || loop {
                let mut buff = vec![0; BUFFER];

                match socket.read_exact(&mut buff) {
                    Ok(_) => {
                        let msg = buff.into_iter()
                            .take_while(|&x| x != 0).collect::<Vec<_>>();

                        let msg = String::from_utf8(msg).expect("Invalid utf8 message");

                        // log the message received
                        println!("{} : {:?}", addr, msg);

                        // send the message to the main thread via the channel
                        tx.send(msg).expect("Failed to send msg to rx");
                    }
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    Err(_) => {
                        println!("closing connection with : {}", addr);
                        break;
                    }
                }

                sleep();
            });
        }

        // receive the message from the channel
        if let Ok(msg) = rx.try_recv() {
            clients = clients
                .into_iter()
                .filter_map(|mut client| {
                    let mut buff = msg.clone().into_bytes();
                    buff.resize(BUFFER, 0);

                    client.write_all(&buff).map(|_| client).ok()
                }).collect::<Vec<_>>();
        }
        sleep();
    }
}
