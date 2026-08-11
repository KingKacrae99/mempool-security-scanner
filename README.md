# Overview
As a software engineer, my goal with this project is to build low-latency, systems-level microservices capable of processing real-time streaming data and persisting security analytics directly to a distributed cloud architecture. To advance my understanding of systems programming, high-concurrency network architectures, non-blocking I/O, and cloud data persistence, I built a native backend security tool using Rust.

I developed a real-time Security Analytics & Frontrunning Detection Engine integrated with Cloud MongoDB Atlas Persistence. The application acts as a high-performance TCP client that establishes a persistent socket connection to a live mempool server on port 9000. As raw transaction streams flow into the system, the Rust engine parses line-delimited JSON payloads using asynchronous buffered memory readers, deserializes them into strongly-typed Rust structures, evaluates gas fee spikes in real-time, and automatically persists threat records to a cloud-managed MongoDB database.

## Software Integration & Usage
Stream Ingestion & Analysis: The engine connects asynchronously to the TCP mempool stream on 127.0.0.1:9000. Incoming JSON streams are deserialized using serde_json.

Database Integration: The app establishes a connection pool to a MongoDB Atlas Cluster via the official asynchronous mongodb Rust driver.

### Execution Workflow:

Insert (Create): When a transaction's gas fee exceeds the threshold (e.g., > 50 Gwei), it is flagged as a frontrunning attack and written to the cloud threats collection. Simultaneously, it upserts a record in a related wallets collection to increment attack counts and update threat severity.

Query (Read): Queries and filters historical threat logs from the cluster based on gas fee thresholds or threat levels.

Update (Modify): Updates threat resolution statuses (e.g., marking a flagged transaction as "REVIEWED").

Delete (Remove): Prunes expired mempool logs or resolved security flags using field filters.

To run the software:

Ensure your .env file contains a valid DATABASE_URL pointing to your MongoDB Atlas cluster.

Start your mock TCP mempool server (e.g., Node.js streaming server on port 9000).

Build and execute the Rust application:

Bash
cargo run
Purpose
The purpose of writing this software was to master core systems programming paradigms in Rust specifically safety, zero-cost abstractions, non-blocking I/O, and asynchronous cloud data persistence. Building this end-to-end pipeline provided practical experience handling asynchronous network socket streams with tokio, manual line framing, and mapping relational document schemas across cloud database collections without relying on garbage collection.

Software Demo Video
[Module 1](https://www.loom.com/share/0830ec1bd5be4375b97051ea25ad7283)
[Module 2](https://www.loom.com/share/c793c09d3c194a2fa0173eafaf2f1d89)
[Module 3](https://www.loom.com/share/cb0dcd6cdd09498fb804a5fa69a322e6)

### Cloud Database
The application utilizes a cloud-managed MongoDB Atlas Cluster hosted on AWS. MongoDB Atlas was selected for its native support for horizontal scaling, low-latency JSON/BSON document persistence, and seamless async integration with Rust.

## Database Structure & Schema
The database consists of two referenced collections within the security_db database to implement a relational data model:

1. threats Collection (Primary Event Logs)
Stores individual flagged frontrunning attack transactions.

JSON
{
  "_id": ObjectId("..."),
  "tx_id": "0x9a8f...",
  "user_name": "TraderX",
  "sender": "0x71C...397",
  "receiver": "0x11B...882",
  "gasfee": 85,
  "status": "FLAGGED"
}
2. wallets Collection (Referenced Account Aggregations)
Stores aggregated threat profiles for flagged wallets linked by wallet_address (sender).

JSON
{
  "_id": ObjectId("..."),
  "wallet_address": "0x71C...397",
  "total_attacks": 4,
  "threat_level": "HIGH"
}
### Development Environment
To construct and test this software, I utilized the following tools:

Visual Studio Code: Integrated Development Environment (IDE) configured with the rust-analyzer extension for code completion, inline compile checking, and refactoring.

Cargo: Rust's official build system and package manager for handling project dependencies and builds.

MongoDB Atlas: Cloud-hosted NoSQL database cluster used for remote data storage and verification.

Node.js (v18+): Used to run the mock mempool TCP server on port 9000 streaming live transaction payloads.

Git & GitHub: Version control system for source code tracking and repository hosting.

The application was built using Rust (2021 Edition) along with the following standard and community crates:

tokio (v1.0): Asynchronous runtime providing non-blocking TCP streams (tokio::net::TcpStream) and async I/O utilities (BufReader).

mongodb (v2.8): Official asynchronous MongoDB driver for Rust built on top of tokio and bson.

bson (v2.8): Data serialization format used by MongoDB, including the doc! macro for constructing query filters.

serde & serde_json (v1.0): High-performance framework for serializing and deserializing Rust data structures and JSON strings.

dotenv (v0.15): Loads environment variables from a .env file to securely manage database connection strings.

futures (v0.3): Provides stream utilities (TryStreamExt) for processing cursor results from database queries.

### Useful Websites
* [The Rust Programming Language Book](https://doc.rust-lang.org/book/)

* [Official MongoDB Rust Driver Documentation](https://www.mongodb.com/docs/drivers/rust/current/)

* [Tokio Asynchronous Framework Documentation](https://tokio.rs/)

* [Serde JSON Documentation](https://docs.rs/serde_json/latest/serde_json/)

* [MongoDB Atlas Documentation](https://www.mongodb.com/docs/atlas/)

### Future Work
Dynamic Gas Thresholds: Implement a moving-average gas threshold calculation using std::collections::VecDeque to dynamically flag anomalies relative to network congestion instead of static thresholds.

Connection Pooling Optimization: Implement fine-grained driver tuning for connection pool sizes and read/write concerns under heavy mempool traffic.

Automated TTL Pruning: Configure Time-To-Live (TTL) indexes on MongoDB collections to automatically expire raw mempool logs after 30 days.