# Setup

1. Install Rust nightly

2. Install Diesel client for SQLite
```
cargo install diesel_cli --no-default-features --features sqlite
```

3. Initialize and run project
```
diesel migration run
cargo run
```
