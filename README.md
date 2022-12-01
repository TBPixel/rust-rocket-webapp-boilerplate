# Rust Rocket WebApp Boilerplate

A (WIP) pure rust, monolithic implimentation of a generic multitenant webapp boilerplate platform.

## Quick Start

This project uses the [just](https://github.com/casey/just) command runner as a helper.

This boilerplate includes a command to quick `setup` the project.

```sh
just setup
```

This command:
- Verifies you have sqlite3 installed, as is required by this project
- Copies `.example.env` over to `.env`
- Installs `sqlx-cli`, which is used to create the database file and run migrations
- Runs an initial `cargo build` for you

Upon success of this command, you should be able to execute `cargo run` and have a live environment local to your machine.
