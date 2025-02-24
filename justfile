_list:
    @just --list

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

# Format project.
fmt:
    just --unstable --fmt
    nixpkgs-fmt .
    fd --hidden --extension=md --extension=yml --exec-batch prettier --write
    fd --hidden --extension=toml --exec-batch taplo format
    cargo +nightly fmt

# Deploy project using Shuttle.
deploy:
    cargo shuttle deploy
