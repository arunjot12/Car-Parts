# Car Parts Project - Development Roadmap

This document outlines the step-by-step instructions for what you need to implement next in your car parts project. You have already completed the Signup flow; the next major milestone is Authentication.

## Phase 1: Authentication & JWT (Current Priority)

You need to allow users and shopkeepers to log in securely and protect your API routes.

- [ ] **Add JWT Dependency**
  - Open `Cargo.toml`.
  - Add `jsonwebtoken = "9.3.0"` to your dependencies.
- [ ] **Create JWT Utility Module (`src/auth/jwt.rs`)**
  - Create structs for your JWT Claims (e.g., `user_id`, `role` [User/Shopkeeper], `exp`).
  - Write a function to `encode` a JWT (generate token).
  - Write a function to `decode` a JWT (validate token).
- [ ] **Create Login Endpoints (`src/login/mod.rs`)**
  - Create `/login_user` and `/login_shopkeeper` endpoints (or a unified `/login`).
  - Accept credentials (email/phone and password).
  - Query the database to find the user/shopkeeper.
  - Verify the provided password against the hashed password in the DB using `argon2`.
  - If valid, generate a JWT and return it in the JSON response.
- [ ] **Create Authentication Middleware (`src/auth/middleware.rs`)**
  - Implement an Axum extractor (e.g., `impl<S> FromRequestParts<S> for Authe   the `Authorization: Bearer <token>` header, decode the JWT, and make the user ID and role available to your route handlers.

## Phase 2: Car Parts Management (Shopkeepers)

Once authentication is working, allow shopkeepers to manage their inventory.

- [ ] **Create `parts` Database Table**
  - Run `diesel migration generate create_parts`.
  - Add columns: `id`, `shopkeeper_id` (foreign key), `name`, `description`, `price`, `stock_quantity`, `category`.
  - Run `diesel migration run`.
- [ ] **Implement Part Models & Schema**
  - Update `schema.rs` and create structs in `models.rs` (`Part`, `NewPart`).
- [ ] **Create Shopkeeper Endpoints (`src/parts/shopkeeper.rs`)**
  - `POST /shopkeeper/parts`: Add a new part (requires Shopkeeper JWT).
  - `PUT /shopkeeper/parts/:id`: Update price/stock (requires Shopkeeper JWT).
  - `DELETE /shopkeeper/parts/:id`: Remove a part.

## Phase 3: Browsing & Searching (Users)

Allow users to find the parts they need.

- [ ] **Create Public/User Endpoints (`src/parts/user.rs`)**
  - `GET /parts`: List all available parts (with pagination).
  - `GET /parts/search?q=brakes`: Search parts by name or category.
  - `GET /parts/:id`: Get details of a specific part.

## Phase 4: Cart and Orders

Handle the checkout process.

- [ ] **Create `orders` and `order_items` Tables**
  - Run migrations for both tables. An order belongs to a user, and contains multiple order items (parts).
- [ ] **Implement Order Endpoints (`src/orders/mod.rs`)**
  - `POST /orders`: Submit a new order (requires User JWT).
  - Deduct the `stock_quantity` from the `parts` table when an order is placed.
  - `GET /orders`: View order history for the logged-in user.

## Phase 5: Payment Integration

Integrate a third-party service to handle actual money.

- [ ] **Set up Stripe (or similar provider)**
  - Add `stripe-rust` crate or use `reqwest` to call their HTTP API.
- [ ] **Implement Checkout Flow**
  - When `POST /orders` is called, generate a Stripe Checkout session.
  - Return the checkout URL to the client.
- [ ] **Implement Webhooks**
  - Create an endpoint `POST /webhooks/stripe` to listen for successful payment events.
  - Mark the order as `Paid` in the database once the webhook confirms it.
