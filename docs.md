# sqlx mig:
cargo sqlx migrate add {name}
cargo sqlx migrate run

# Build only `crate_a`
cargo build -p crate_a

# build a bin
When you have multiple bins, you need to specify which bin you want to run.
cargo run --bin {name} 

# if you run cargo from the workspace directory like cargo run -p product-service, then you need to find .env file like below
let env_path = std::path::Path::new("product-service/src/").join(".env");
dotenv::from_path(env_path).ok();

# mod {name};
declares a module. declare it in one place, typically in lib.rs or main.rs.

# run unit tests
cargo test
cargo test {test_name}
cargo test {test_name_pattern} for example all tests's name that contain 'panic': cargo test panic
cargo test --test {file_name} testing a specifi integration test


# Identify the Dependency Tree
Use the following command(powershell) to see which dependency is pulling in idna:
cargo tree | Select-String "idna"
