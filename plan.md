
TODO:
# Best Practice of Error Handling
# https://github.com/colin-kiegel/rust-derive-builder
Learn idiomatic error handling with Result and Option types. Avoid panicking in production code because it can be costly in terms of latency.
https://doc.rust-lang.org/cargo/
# Efficient Data Structures
Focus on using Vec, HashMap, and smart pointers (Box, Rc, and Arc) 

# ------------------
Mutex, RwLock, or one of the Atomic types: Shared references in Rust disallow mutation by default, and Arc is no exception: you cannot generally obtain a mutable reference to something inside an Arc. If you need to mutate through an Arc, use Mutex, RwLock, or one of the Atomic types.

Smart Pointers (Box<T>, Rc<T>, Arc<T>
References Lifetimes
Option instead of null pointers
Event-Broker between them.
Dereferencing, ownership, 
unit tests
write some benchmark
tracing https://github.com/tokio-rs/tracing
Rayon: A data parallelism library: https://github.com/rayon-rs/rayon

Copy trait https://doc.rust-lang.org/book/ch10-02-traits.html  
axum middleware https://docs.rs/axum/latest/axum/middleware/index.html#intro
Mutexes and Atomic Types: For data that must be mutable and shared across threads, Rust provides synchronization primitives like Mutex and RwLock
tokyo in depth
# Jobs
• Experience working with low-latency, concurrent systems, Real-time systems.
• Web3, blockchain, Crypto, Defi, smart contracts, consensus algorithms  experience, Zero-Knowledge Proof,  p2p networks,
  mechanics of the EVM
• Knowledge of Solana and its ecosystem
• Building low latency and highly parallelised systems, distributed systems, concurrency and multithreading.
• Cargo tooling
• authentication and authorization

------------------------------------
# Review NoSql- Mongo, Cosmos
