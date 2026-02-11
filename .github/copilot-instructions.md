# Copilot Instructions for md-to-pdf

## Project Overview

Rust web service (Rocket 0.5) that converts Markdown to PDF using Pandoc with multiple engines (weasyprint, wkhtmltopdf, pdflatex).

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

- Use `make fmt` for code formatting (enforced)
- Error handling uses custom `ConvertError` enum implementing `Responder`

## Project Structure

```
src/main.rs           - Main application entry point with conversion logic
static/               - Static files for web UI
Makefile              - Build and development commands
```

## Testing

- Run `make test` to build and test the Docker image
- Always test Docker builds before deploying

## What to Modify

- `src/main.rs` - Application logic
- `static/` - Web UI files

## What NOT to Modify

- `Cargo.lock` unless updating dependencies
- Production secrets or environment variables

## Git Workflow

- Format code with `make fmt` before committing
- Test Docker builds before finalizing changes

## Deployment Notes

- Application runs on port 8000
- Configured via Rocket environment variables
- Deployed on Fly.io

## Security Considerations

- CORS is configured to allow all origins (intended for public API)
- PDF conversion happens in isolated process (Pandoc)
