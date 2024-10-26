
String: Owned, heap-allocated, and mutable.
&str: A borrowed reference to a string slice, which is not heap-allocated and is immutable.

U32, i32

workspace, package (binary or library), crate, module, sub-module

sqlx mig:
cargo sqlx migrate add {name}
cargo sqlx migrate run



workspace, package (binary or library), crate, module, sub-module
# Build only `crate_a`
cargo build -p crate_a
# Run tests for all crates
cargo test

Benefits of a Shared Cargo.lock in a Workspace
Dependency Management: By sharing a Cargo.lock file, all packages in the workspace use the same versions of dependencies. This ensures consistency and can help avoid "dependency hell" where different packages have conflicting dependency versions.
Efficiency: Sharing the target directory means that the compiled output of dependencies is reused across packages, speeding up the build process.

------
TODO:
write some benchmark
unit tests
