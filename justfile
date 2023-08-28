_list:
    @just --list

# Check project.
check:
    just --unstable --fmt --check
    npx -y prettier --check '**/*.md'
    taplo lint
    cargo +nightly fmt -- --check

# Format project.
fmt:
    just --unstable --fmt
    npx -y prettier --write '**/*.md'
    taplo format
    cargo +nightly fmt

# Deploy project using Shuttle.
deploy:
    cargo shuttle deploy
