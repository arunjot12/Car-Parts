# Gearly - Car Parts E-Commerce 🚗⚙️

![Status](https://img.shields.io/badge/Status-In%20Progress-yellow)
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)

A backend API for a Car Parts marketplace built with Rust, Axum, and Diesel.

## Overview
Gearly is a platform connecting car parts shopkeepers with users. 
- **Shopkeepers** can register, manage their inventory, and list car parts.
- **Users** can browse parts, search for specific items, and place orders.

## Development Progress
We are actively developing this project. Check out the [ROADMAP.md](./ROADMAP.md) for a detailed breakdown of completed tasks and next steps.

## Tech Stack
- **Web Framework:** [Axum](https://github.com/tokio-rs/axum)
- **Database ORM:** [Diesel](https://diesel.rs/)
- **Database:** MySQL
- **Async Runtime:** Tokio
- **Authentication:** JWT & Argon2

## Running Locally

1. Set up your `.env` file with `DATABASE_URL=mysql://...`
2. Run migrations:
   ```bash
   diesel migration run
   ```
3. Start the server:
   ```bash
   cargo run
   ```
