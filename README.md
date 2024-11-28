# Rust Chat App 🚀  

A **multi-client chat application** built in **Rust**, featuring a TCP server and client logic for real-time messaging. This project demonstrates the power of Rust's concurrency and networking capabilities while maintaining safe, efficient, and scalable code.  

## Features 🌟  
- **Multi-client support**: Connect multiple clients to the server simultaneously.  
- **Real-time messaging**: Messages are broadcast to all connected clients.  
- **Thread-safe communication**: Utilizes Rust's `mpsc` channels for safe message passing.  
- **Asynchronous handling**: Non-blocking socket operations for seamless performance.  

## Table of Contents  
1. [Project Overview](#project-overview)  
2. [Setup and Installation](#setup-and-installation)  
3. [Server Logic](#server-logic)  
4. [Client Logic](#client-logic)  
5. [How to Run](#how-to-run)  
6. [Future Enhancements](#future-enhancements)  
7. [Contributing](#contributing)  

---

## Project Overview  

This chat application is divided into two parts:  
1. **Server**: A TCP-based server that manages client connections and broadcasts messages to all connected clients.  
2. **Client**: A TCP client that connects to the server and allows users to send and receive messages in real time.  

---

## Setup and Installation  

### Prerequisites  
- Rust installed on your system. You can install it via [rustup](https://rustup.rs/).  
- A terminal to run the server and client instances.  

### Clone the Repository  
```bash  
git clone https://github.com/dexter-xD/tcp_server_rust.git  
cd tcp_server_rust  
```  

---

## Server Logic  

The server listens on a specified address and port (`127.0.0.1:8080` by default) and handles the following:  
1. Accepting incoming client connections.  
2. Receiving messages from clients.  
3. Broadcasting messages to all connected clients.  

**Code Highlights**:  
- **Non-blocking Sockets**: Allows seamless communication without halting the server.  
- **Concurrency with Threads**: Each client connection is handled in its own thread.  
- **Message Broadcasting**: Uses an `mpsc` channel to safely share messages across threads.  

You can find the server code in the [`server.rs`](server.rs) file.

---

## Client Logic  

The client connects to the server, enabling users to:  
1. Send messages to the server.  
2. Receive messages broadcast by the server.  

**Features**:  
- Handles server disconnection gracefully.  
- Provides an intuitive terminal-based interface for chatting.  

Client logic will be implemented in **Part 2** of this project. Stay tuned for updates!  

---

## How to Run  

### Running the Server  
1. Navigate to the project directory.  
2. Run the server:  
   ```bash  
   cargo run --bin server  
   ```  
3. The server will start listening for connections on `127.0.0.1:8080`.  

### Running the Client  
The client logic will be added in **Part 2**.  

---

Stay tuned for updates and happy coding! 🦀  

--- 

