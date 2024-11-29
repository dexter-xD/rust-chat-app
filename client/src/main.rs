use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

const ADRESS: &str = "127.0.0.1:8080"; // The server's address
const BUFFER: usize = 32; // Fixed message size to simplify communication

fn main() {
  // Establish connection to the server
  let mut client = TcpStream::connect(ADRESS).expect("Stream failed to connect");
  // Enable non-blocking mode for asynchronous operations
  client
    .set_nonblocking(true)
    .expect("Failed to initiate non-blocking");

  // Create a channel to send messages between threads
  let (tx, rx) = mpsc::channel::<String>();

  // Spawn a separate thread to handle server communication
  thread::spawn(move || loop {
    let mut buff = vec![0; BUFFER];
    match client.read_exact(&mut buff) {
      Ok(_) => {
        // Process and display incoming messages
        let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
        println!("Message received: {:?}", msg);
      }
      Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
      Err(_) => {
        // Handle disconnection
        println!("Connection with server was severed");
        break;
      }
    }

    // Send messages from the channel to the server
    match rx.try_recv() {
      Ok(msg) => {
        let mut buff = msg.clone().into_bytes();
        buff.resize(BUFFER, 0);
        client.write_all(&buff).expect("Writing to socket failed");
        println!("Message sent: {:?}", msg);
      }
      Err(TryRecvError::Empty) => (),
      Err(TryRecvError::Disconnected) => break,
    }

    thread::sleep(Duration::from_millis(100)); // Prevent tight looping
  });

  println!("Write a Message:");
  loop {
    let mut buff = String::new();
    // Read user input and send it to the other thread
    io::stdin()
      .read_line(&mut buff)
      .expect("Reading from stdin failed");
    let msg = buff.trim().to_string();
    // Exit the loop on ":quit" or if the channel is disconnected
    if msg == ":quit" || tx.send(msg).is_err() {
      break;
    }
  }
  println!("Bye bye!");
}
