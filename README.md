# Actix Web OAuth Example (GitHub)

GitHub OAuth login example built with Actix Web.

This app no longer uses Shuttle; it now runs as a standalone web server with environment-based configuration.

A preview is available at <https://actix-examples-oauth-github.x52.dev>.

## Local Development

1. Copy `.env.example` to `.env`.
2. Fill in `GH_CLIENT_ID` and `GH_CLIENT_SECRET`.
3. Start the app with `cargo run` or `just run`.

```env
HOST=0.0.0.0
PORT=8080
PUBLIC_BASE_URL=http://localhost:8080
GH_CLIENT_ID=your-github-client-id
GH_CLIENT_SECRET=your-github-client-secret
RUST_LOG=info
```

The app reads `.env` automatically in local development, and in production it reads the same settings from real environment variables.

## Configuration

The supported environment variables are:

- `HOST`
- `PORT`
- `PUBLIC_BASE_URL`
- `GH_CLIENT_ID`
- `GH_CLIENT_SECRET`
- `RUST_LOG`

Defaults:

- `HOST=0.0.0.0`
- `PORT=8080`
- `PUBLIC_BASE_URL=http://localhost:8080`
- `RUST_LOG=info`

## GitHub OAuth Setup

Create a GitHub OAuth app and configure the authorization callback URL to match:

```text
${PUBLIC_BASE_URL}/auth/github/callback
```

For local development, that means:

```text
http://localhost:8080/auth/github/callback
```
