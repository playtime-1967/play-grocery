# types
String: Owned, heap-allocated, and mutable.
&str: A borrowed reference to a string slice, which is not heap-allocated and is immutable.
U32, i32


# workspace, package (binary or library), crate, module, sub-module

# sqlx mig:
cargo sqlx migrate add {name}
cargo sqlx migrate run

workspace, package (binary or library), crate, module, sub-module
# Build only `crate_a`
cargo build -p crate_a
# Run tests for all crates
cargo test

# Benefits of a Shared Cargo.lock in a Workspace
Dependency Management: By sharing a Cargo.lock file, all packages in the workspace use the same versions of dependencies. This ensures consistency and can help avoid "dependency hell" where different packages have conflicting dependency versions.
Efficiency: Sharing the target directory means that the compiled output of dependencies is reused across packages, speeding up the build process.

# if you run cargo from the workspace directory like cargo run -p product-service, then you need to find .env file like below
let env_path = std::path::Path::new("product-service/src/").join(".env");
dotenv::from_path(env_path).ok();

# Protocol Buffers 
it uses a compact binary format to transmit data. Instead of including field names (like id, name, etc.) in the encoded data, each field is represented by its number.
When the data is serialized, only the field number and value are included

# Arc
A thread-safe reference-counting pointer.
It provides shared ownership of a value of type T, allocated in the heap. Invoking clone on Arc produces a new Arc instance, which points to the same allocation on the heap as the source Arc.

# Concurrency 
Interleaving the tasks efficiently to give the appearance of multitasking.
Structuring tasks in a way that allows for efficient interleaving of multiple tasks without necessarily running them simultaneously.
# Parallelism
The actual simultaneous execution of tasks, usually on multiple cores of a CPU. Parallel tasks run truly "side by side," utilizing separate CPU resources at the same moment.

------
TODO:
tokio-postgree for order-micro
Dereferencing, ownership, 
unit tests
write some benchmark
Copy trait https://doc.rust-lang.org/book/ch10-02-traits.html  
axum middleware https://docs.rs/axum/latest/axum/middleware/index.html#intro

