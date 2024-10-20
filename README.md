# Toothpaste

[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg)](https://conventionalcommits.org)
![License](https://img.shields.io/github/license/aeyoll/toothpaste)

A pastebin application written in Rust with a frontend built using Yew.

## Requirements

- [Rust](https://www.rust-lang.org/)
- [Node.js](https://nodejs.org/)
- [Trunk](https://trunkrs.dev/)

## Project Structure

The project is organized into two main crates:

1. `toothpaste-backend`: The Rust backend server
2. `toothpaste-frontend`: The Yew-based frontend

## Installation and Setup

1. Set the `DATABASE_URL` environment variable (e.g., `DATABASE_URL=sqlite://sqlite.db`).

2. Initialize the database:

```shell
touch sqlite.db
```

## Build the frontend assets

```shell
cd crates/toothpaste-frontend

# Build the frontend assets
# For development
TOOTHPASTE_API_URL=http://127.0.0.1:8080 trunk serve --port 8081
# For production
TOOTHPASTE_API_URL=http://127.0.0.1:8080 trunk build --release

# Compile the css
npm install
# For development
npm run watch
# For production
npm run build
```

## Build and run the backend server:

```shell
cargo run -p toothpaste-backend -- --ip 127.0.0.1 --port 8080 # default values
```

## Set up a cron job for paste expiration cleanup:

```shell
* * * * * wget -q -O /dev/null "http(s)://your_host/paste/cleanup"
```

## License

See the LICENSE file for details.
