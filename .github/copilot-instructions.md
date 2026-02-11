# Copilot Instructions for md-to-pdf

## Project Overview

This is a web service that converts Markdown to PDF using:
- **Language**: Rust (2021 edition)
- **Web Framework**: Rocket 0.5
- **PDF Conversion**: Pandoc with multiple engines (weasyprint, wkhtmltopdf, pdflatex)
- **Deployment**: Docker containers

## Tech Stack

- Rust
- Rocket web framework
- rocket_cors for CORS support
- tempfile for temporary file handling
- Pandoc (external dependency in Docker container)

## Development Commands

```bash
# Set up the local environment
make setup

# Format code
make fmt

# Build the application
make target/debug

# Serve the application
make serve

# Run tests (builds Docker image and runs test script)
make test

# Build production Docker image
make docker-build
```

## Code Style and Conventions

- Use `cargo fmt` for code formatting (enforced)
- Follow Rust 2021 edition idioms
- Use `#[macro_use]` for external crate imports (existing pattern)
- Use Rocket's attribute macros (`#[post]`, `#[launch]`, etc.)
- Error handling uses custom `ConvertError` enum implementing `Responder`
- Async functions for I/O operations (file handling)

## Project Structure

```
src/
  main.rs           - Main application entry point with conversion logic
static/             - Static files for web UI
Cargo.toml          - Rust dependencies
Dockerfile          - Production Docker image
rust.dockerfile     - Development Rust container
pandoc.dockerfile   - Pandoc container
docker-compose.yml  - Local development environment
Makefile            - Build and development commands
```

## Testing

- Run `make test` to build and test the Docker image
- Test script: `./test-docker.sh` validates the Docker image
- Always test Docker builds before deploying

## What to Modify

✅ **Safe to modify:**
- `src/main.rs` - Application logic
- `static/` - Web UI files
- `README.md` - Documentation
- Docker configuration files (with care)
- `Makefile` - Build automation

## What NOT to Modify

❌ **Do not modify:**
- `Cargo.lock` unless updating dependencies
- Production secrets or environment variables
- Rocket configuration without testing

## Git Workflow

- Format code with `cargo fmt` before committing
- Test Docker builds before finalizing changes
- Keep commits focused and atomic

## Deployment Notes

- Application runs on port 8000
- Configured via Rocket environment variables
- Docker image published to Docker Hub (spawnia/md-to-pdf)
- Deployed on Fly.io

## Security Considerations

- Be cautious with tempfile handling (already using proper API)
- Validate markdown input (current implementation trusts input)
- CORS is configured to allow all origins (intended for public API)
- PDF conversion happens in isolated process (Pandoc)
