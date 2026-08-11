use mongodb::Client;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::TcpStream;
use std::env;

mod securitydb;
mod transaction;
use transaction::Transaction;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Rust Security Scanner & Cloud Persistence Engine...");

    // Load environment variables
    dotenv::dotenv().ok();
    let uri = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

    // Connect asynchronously to MongoDB Atlas (Requirement 1)
    println!("Connecting to MongoDB Atlas...");
    let client = Client::with_uri_str(&uri).await?;
    let db = client.database("security_db");
    println!("Connected to MongoDB database 'security_db'!\n");

    // Connect to Node.js TCP Server on Port 9000
    let stream = TcpStream::connect("127.0.0.1:9000")
        .await
        .expect("Could not connect to TCP server. Is Node.js running?");

    println!("Connected to Mempool Stream on port 9000!\n");

    let mut reader = BufReader::new(stream);
    let mut line_buffer = String::new();

    // Stream and process transactions
    while reader.read_line(&mut line_buffer).await? > 0 {
        let json_str = line_buffer.trim();
        if !json_str.is_empty() {
            match serde_json::from_str::<Transaction>(json_str) {
                Ok(tx) => {
                    if let Err(e) = tx.analyze_security(&db).await {
                        eprintln!("Error logging transaction to database: {}", e);
                    }
                }
                Err(e) => eprintln!("Failed to parse JSON: {}", e),
            }
        }
        line_buffer.clear();
    }

    Ok(())
}