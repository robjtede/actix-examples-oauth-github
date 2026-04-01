image_ref := "actix-examples-oauth-github:latest"

_list:
    @just --list

# Initialize the project.
init:
    [ -f .env ] || cp .env.example .env

# Run the web server.
run:
    cargo run

# Build the production container image.
build: init
    docker build -t {{ image_ref }} .

# Run the production container image locally with the local .env file mounted read-only.
run-docker: build
    docker run --rm \
        --publish 8080:8080 \
        --volume "$(pwd)/.env:/app/.env:ro" \
        {{ image_ref }}

# Lint workspace.
clippy:
    cargo clippy --workspace --all-features -- -D warnings

# Lint workspace and watch for changes.
clippy-watch:
    cargo watch -- cargo clippy --workspace --all-features

# Apply possible linting fixes in the workspace.
clippy-fix *args:
    cargo clippy --workspace --all-features --fix {{ args }}
    cargo +nightly fmt

# Test workspace.
test:
    cargo nextest run --workspace --all-features

# Check project formatting.
check: && clippy
    just --unstable --fmt --check
    nixpkgs-fmt .
    fd --hidden --extension=md --extension=yml --exec-batch prettier --check
    fd --hidden --extension=toml --exec-batch taplo format --check
    fd --hidden --extension=toml --exec-batch taplo lint
    cargo +nightly fmt -- --check
    cargo machete --with-metadata

# Format project.
fmt:
    just --unstable --fmt
    nixpkgs-fmt .
    fd --hidden --extension=md --extension=yml --exec-batch prettier --write
    fd --hidden --extension=toml --exec-batch taplo format
    cargo +nightly fmt
