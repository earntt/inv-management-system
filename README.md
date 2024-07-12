# inv-mgt-platform

## running a web server (local)

```sh
$ cargo run
$ cargo watch -x run
```

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


## initialize database
up docker-compose
```sh
cp .env.example .env
docker-compose up -d
```
