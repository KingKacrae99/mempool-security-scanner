use std::io::{BufRead, BufReader};
use std::net::TcpStream;

// Declare the module and import the Transaction struct
mod transaction;
use transaction::Transaction;

fn main() {
    println!("Starting Rust Security Scanner...");

    // 1. Connect to Node.js TCP Server on Port 9000
    let stream = TcpStream::connect("127.0.0.1:9000")
        .expect("Could not connect to TCP server. Is Node.js running?");

    println!("Connected to Mempool Stream on port 9000!\n");

    let reader = BufReader::new(stream);

    // 2. Loop through incoming lines from the stream
    for line in reader.lines() {
        match line {
            Ok(json_str) => {
                // 3. Deserialize JSON string into Rust Struct from external module
                match serde_json::from_str::<Transaction>(&json_str) {
                    Ok(tx) => tx.analyze_security(), // Call struct impl method
                    Err(e) => eprintln!("Failed to parse JSON: {}", e),
                }
            }
            Err(e) => {
                eprintln!("Error reading stream: {}", e);
                break;
            }
        }
    }
}