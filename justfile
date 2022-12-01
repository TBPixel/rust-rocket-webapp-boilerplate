setup:
  if ! command -v sqlite3 &> /dev/null; then \
    echo "sqlite3 could not be found"; \
    echo "make sure to install sqlite3 via a package manager. On MacOS, you can do this with:"; \
    echo "brew install sqlite"; \
    exit; \
  fi
  cp .example.env .env;
  cargo install sqlx-cli --no-default-features --features rustls,sqlite;
  sqlx database create;
  sqlx migrate run;
  cargo build;
  echo "you're all setup! Run the project with:";
  echo "cargo run";

run:
  cargo run;

lint:
  cargo clippy;

test:
  cargo clippy;
  cargo test;
