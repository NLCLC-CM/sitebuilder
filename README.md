# Development requirements

- Rust
- Postgres

# Initial setup

- Create database user with `createuser <user>`
- Create database with `createdb -O <user> <dbname>`
- Create environmental variable `DATABASE_URL` to be `postgres://<user>@localhost/<dbname>` and make sure
  it is in your session
- Run database migrations with `sqlx-cli` or the binary
    - `sqlx migrate run`
    - `cargo run --bin migrate`
