# Toothpaste

[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg)](https://conventionalcommits.org)
![License](https://img.shields.io/github/license/aeyoll/toothpaste)

A pastebin application written in Rust with a frontend built using Yew. It also includes a CLI tool to create pastes.

## Requirements

- [Rust](https://www.rust-lang.org/)
- [Trunk](https://trunkrs.dev/)

## Project Structure

The project is organized into three main crates:

1. `toothpaste-backend`: The Rust backend server
2. `toothpaste-frontend`: The Yew-based frontend
3. `toothpaste-cli`: The CLI tool to create pastes

## Installation and Setup

1. Set the `DATABASE_URL` environment variable (e.g., `DATABASE_URL=sqlite://sqlite.db`).

2. Initialize the database:

```shell
touch sqlite.db
```

## Build the frontend assets

```shell
cd crates/toothpaste-frontend

# Build the frontend static
# For development
TOOTHPASTE_API_URL=http://127.0.0.1:8080 trunk serve --port 8081
# For production
trunk build --release
```

## Build and run the backend server:

```shell
toothpaste-backend --ip 127.0.0.1 --port 8080 # default values
```

## Build and run the CLI tool:

```shell
toothpaste-cli --name toothpaste.txt --expire-after 86400 < your-content.txt # From stdin redirection
cat your-content.txt | toothpaste-cli --name toothpaste.txt --expire-after 86400 # From stdin indirection
toothpaste-cli --expire-after 86400 -f your-content.txt # From file
```

## Set up a cron job for paste expiration cleanup:

```shell
* * * * * wget -q -O /dev/null "http(s)://your_host/api/paste/cleanup"
```

## Sample nginx configuration

```nginx
server {
    server_name paste.foo.bar;

    root /path/to/toothpaste/frontend;

    location / {
        try_files $uri $uri/ /index.html;
    }

    location /api/ {
        proxy_pass http://127.0.0.1:8081;
        proxy_set_header Host $host;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
    }
}
```

## License

See the LICENSE file for details.
