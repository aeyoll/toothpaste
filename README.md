# Toothpaste

A pastebin written in Rust.

Requirements
---

- [https://crates.io/crates/sqlx-cli](sqlx-cli): Database management
- [https://yarnpkg.com/](yarn): Asset management

Install
---

Set a `DATABASE_URL` environment variable (eg `DATABASE_URL=sqlite://sqlite.db`). Then, init the database:

```sh
sqlx database create
sqlx migrate run
```

Build static assets:

```sh
yarn
yarn run build
```

Development
---

For easier development, use [https://github.com/watchexec/cargo-watch](cargo-watch) to auto-compile on change:

```sh
cargo watch -x 'run'
```

Auto-compile static assets:

```sh
yarn run start
```
