# Toothpaste

[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg)](https://conventionalcommits.org)
![License](https://img.shields.io/github/license/aeyoll/toothpaste)

A pastebin written in Rust.

Requirements
---

- [https://yarnpkg.com/](yarn): Asset management

Install
---

Set a `DATABASE_URL` environment variable (eg `DATABASE_URL=sqlite://sqlite.db`). Then, init the database:

```sh
touch sqlite.db
```

Build static assets:

```sh
yarn
yarn run build
```

Launch the http server:

```sh
toothpaste --ip 127.0.0.1 --port 8080 # default values
```

Finally, setup a cron every minute for paste expire cleanup:

```sh
* * * * * wget -q -O /dev/null "http(s)://your_host/paste/cleanup"
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
