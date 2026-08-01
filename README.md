# Overview

As a software engineer, my goal with this project is to build low-latency, systems-level microservices capable of processing real-time streaming data. To advance my understanding of systems programming, high-concurrency network architectures, and memory-safe design patterns, I built a native backend security tool using Rust.

I developed a real-time **Security Analytics & Frontrunning Detection Engine**. The application acts as a high-performance TCP client that establishes a persistent socket connection to a live mempool server on port `9000`. As raw transaction streams flow into the system, the Rust engine parses line-delimited JSON payloads using buffered memory readers, deserializes them into strongly-typed Rust structures, and evaluates gas fee spikes in real-time to flag potential frontrunning attacks directly in the console.

The purpose of writing this software was to master core systems programming paradigms in Rust—specifically safety, performance, zero-cost abstractions, and strict ownership models. By building a network-ingestion service from scratch, I gained hands-on experience handling low-level socket streams, manual line framing over non-message-based TCP protocols, and type-safe data deserialization without relying on runtime garbage collection.

[Software Demo Video](http://youtube.link.goes.here)

# Development Environment

To construct and test this software, I utilized the following tools:

* **Visual Studio Code:** Integrated Development Environment (IDE) configured with the `rust-analyzer` extension for code completion and inline compile checking.
* **Cargo:** Rust's official build system and package manager for handling project dependencies, compilation targets, and builds.
* **Node.js (v18+):** Used to run the mock mempool TCP server on port `9000` that streams live transaction payloads.
* **Git & GitHub:** Version control system for source code tracking and repository hosting.

The application was built using **Rust (2021 Edition)** along with standard and community libraries:

* **`std::net::TcpStream`:** Native standard library module used to open a persistent Layer-4 socket pipe to `127.0.0.1:9000`.
* **`std::io::{BufReader, BufRead}`:** Standard input/output utilities used to wrap raw network socket streams in memory buffers and split incoming byte streams at newline (`\n`) delimiters.
* **`serde` (v1.0):** Framework for serializing and deserializing Rust data structures efficiently.
* **`serde_json` (v1.0):** JSON data format support for `serde`, used to parse raw network JSON strings into strongly-typed Rust structs.

# Useful Websites

* [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
* [Rust Standard Library Documentation - std::net::TcpStream](https://doc.rust-lang.org/std/net/struct.TcpStream.html)
* [Serde JSON Documentation](https://docs.rs/serde_json/latest/serde_json/)
* [Understanding Rust Ownership and Borrowing](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)

# Future Work

* Implement multi-threading using Tokio (`tokio::net::TcpStream`) to process incoming mempool streams concurrently across multiple CPU threads.
* Integrate the asynchronous `mongodb` Rust driver to auto-persist flagged frontrunning transactions directly into a cloud database cluster.
* Implement a moving-average gas threshold calculation using `std::collections::VecDeque` to dynamically detect gas spikes based on historical averages rather than fixed numeric limits.