# Refactor CLI Signup to Axum REST APIs

This plan details the transition from a CLI-based signup process to a proper REST API using the Axum web framework. This is a critical foundational step before implementing JWT tokens or OTP.

## Background Context
Currently, the application runs as a CLI tool. The `main.rs` prompts the user for inputs using `println!` and reads responses via `std::io::stdin()`. The database operations are tightly coupled to this CLI flow. 
To build a scalable backend, we need to:
1. Start an asynchronous HTTP server using Axum.
2. Accept JSON payloads via HTTP `POST` requests instead of terminal input.
3. Manage database connections efficiently using a connection pool so that multiple API requests can be handled concurrently.

> [!IMPORTANT]
> **User Review Required**
> Diesel is a synchronous ORM. Using it directly inside an Axum async handler blocks the async runtime. The standard solution is to use a connection pool (like `r2d2`) and `tokio::task::spawn_blocking` to perform database queries. I will modify your `Cargo.toml` to add the `r2d2` feature for `diesel` to support connection pooling. 

## Open Questions

> [!WARNING]
> Please review the following design decisions and let me know if you want any changes before we proceed:
> 1. Do you agree to add the `r2d2` feature to `diesel` in your `Cargo.toml` for connection pooling?
> 2. For the API response, I plan to return standard JSON responses with HTTP status codes (e.g., `200 OK` on success, `400 Bad Request` on validation failure, `409 Conflict` if the phone number already exists). Does this work for you?

## Proposed Changes

---

### Core Dependencies

#### [MODIFY] [Cargo.toml](file:///Users/arunjot/user-arun/Learning/rust-projects/car-parts-projects/Cargo.toml)
- Add `r2d2` feature to the `diesel` dependency: `diesel = { version = "2.3.11", features = ["mysql", "chrono", "r2d2"] }`.
- Ensure `serde` has the `derive` feature enabled (currently `serde = "1.0.228"`, we will change to `serde = { version = "1.0", features = ["derive"] }`).

---

### Database Connection and State

#### [MODIFY] [src/connection.rs](file:///Users/arunjot/user-arun/Learning/rust-projects/car-parts-projects/src/connection.rs)
- Refactor `establish_connection` to initialize and return a `r2d2::Pool<r2d2::ConnectionManager<MysqlConnection>>` instead of a single connection. This pool will be shared across all API routes via Axum's `State`.

---

### API Payloads & Handlers

#### [MODIFY] [src/data.rs](file:///Users/arunjot/user-arun/Learning/rust-projects/car-parts-projects/src/data.rs)
- Add `serde::Deserialize` derives to structs where necessary.
- Create new structs `CustomerSignupRequest` and `ShopkeeperSignupRequest` to capture the raw input (which includes a raw `password` string instead of `hashed_password`).

#### [NEW] [src/signup/api.rs](file:///Users/arunjot/user-arun/Learning/rust-projects/car-parts-projects/src/signup/api.rs)
- Implement Axum handlers: `customer_signup_handler` and `shopkeeper_signup_handler`.
- These handlers will extract the JSON payload, hash the password (moving the hashing logic from `signup_users.rs`), and call the database functions inside `tokio::task::spawn_blocking`.

---

### Refactoring Signup Logic

#### [MODIFY] [src/signup/signup_users.rs](file:///Users/arunjot/user-arun/Learning/rust-projects/car-parts-projects/src/signup/signup_users.rs)
#### [MODIFY] [src/signup/signup_shopkeeper.rs](file:///Users/arunjot/user-arun/Learning/rust-projects/car-parts-projects/src/signup/signup_shopkeeper.rs)
- Remove `read_input()` calls and CLI prompts.
- Refactor to pure functions that take the raw request structs, validate them (email/phone), hash the password, and return the `NewUsers` / `NewSignupShopkeepers` models.

#### [MODIFY] [src/signup/database.rs](file:///Users/arunjot/user-arun/Learning/rust-projects/car-parts-projects/src/signup/database.rs)
- Update `handle_customer_signup` and `handle_shopkeeper_signup` to return a `Result<(), String>` (or custom error type) instead of printing directly to the terminal. This allows the Axum handlers to return proper HTTP responses.

---

### Server Entrypoint

#### [MODIFY] [src/main.rs](file:///Users/arunjot/user-arun/Learning/rust-projects/car-parts-projects/src/main.rs)
- Initialize the database connection pool.
- Setup an `axum::Router` with the routes `/api/signup/customer` and `/api/signup/shopkeeper`.
- Bind the server to `0.0.0.0:3000` and `serve` it.

---

## Verification Plan

### Automated Tests
- While we won't write full integration tests immediately, we will verify the code compiles (`cargo check`).

### Manual Verification
- We will start the server using `cargo run`.
- We will use `curl` commands to simulate HTTP POST requests for both a Customer and Shopkeeper signup to verify data is correctly inserted into the MySQL database.
- We will verify that duplicate phone numbers return a `409 Conflict` or equivalent error.
