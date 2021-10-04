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

# API
Get pages
```
http://127.0.0.1:8000
```

Add a page with title
```
http://127.0.0.1:8000?_title=Title
```
