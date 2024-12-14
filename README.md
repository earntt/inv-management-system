# Inventory Management System - Backend

This project is the backend for an **Inventory Management System**, developed during my internship in **2024**. The backend service is built using **Rust** with the **Axum** framework, and **PostgreSQL** for database management. The project provides API endpoints to handle product and order data, focusing on efficient inventory tracking, order management, and user authentication.

---

## initialize database
up docker-compose
```sh
cp .env.example .env
docker-compose up -d
```

---

## sqlx prepare & migration

https://crates.io/crates/sqlx-cli

```sh
cargo sqlx prepare -- --all-targets --all-features
```

sqlx migration

```sh
sqlx migrate add -r {migration_name}
sqlx migrate run
```

---

## running a web server (local)

```sh
$ cargo run
$ cargo watch -x run
```

---


