# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed
- Fixed Fly.io deployment lease conflict issues with concurrency control (#32)
  - Added concurrency control to prevent overlapping deployments
  - Added `--lease-timeout=120s` flag to handle lease conflicts better
  - Added `--wait-timeout=600` flag to allow enough time for deployments
  - Fixed timeout unit to be consistent with lease-timeout
  - Use static concurrency group for all Fly.io deployments

### Changed
- Updated indirect dependency `time` from 0.3.44 to 0.3.47
- Updated indirect dependency `bytes` from 1.10.1 to 1.11.1

### Added
- Added Copilot instructions for the repository

## [1.6.0] - 2025-11-12

### Added
- Multi-architecture Docker support
  - linux/amd64 (x86_64 systems - Intel/AMD processors)
  - linux/arm64 (ARM 64-bit systems - Apple Silicon M1/M2/M3/M4, AWS Graviton, Raspberry Pi 4/5, etc.)
- QEMU and Docker Buildx support for cross-platform builds
- Architecture-specific testing for all Docker images
- Automated multi-arch publishing on every release

### Changed
- Dockerfile now dynamically detects and installs correct wkhtmltopdf package for target architecture

### Removed
- ARM/v7 (32-bit ARM) support - wkhtmltopdf package not available for this architecture

## [1.5.0] - 2025-10-31

### Changed
- Upgraded to Debian Trixie from Debian 12
  - Fixes GLIBC 2.39 compatibility issues
  - Ensures compatibility with modern Rust toolchains
- Optimized Docker image for size
  - Use `--no-install-recommends` for package installation
  - Remove build dependencies after compilation
  - Install wkhtmltopdf from official releases
- Optimized Fly.io deployment
  - Switch to request-based concurrency (better load handling)
  - Increase timeouts for graceful shutdown
  - Update concurrency limits

### Added
- Comprehensive test-docker.sh script
- Validation for all PDF conversion tools (pandoc, wkhtmltopdf, pdflatex, weasyprint)
- End-to-end API functionality tests

## [1.4.5] - 2024-11-07

### Changed
- Bumped dependencies

## [1.4.4] - 2024-11-07

### Added
- Documentation for webserver configuration via ROCKET_ environment variables

## [1.4.3] - 2024-08-11

### Changed
- Optimized Dockerfile to reduce image size
  - Implement best practices for package management
  - Remove APT cache after installation
  - Prevent pip from creating cache files

## [1.4.2] - 2024-07-09

### Changed
- Always serve all IP addresses (0.0.0.0)

## [1.4.1] - 2024-07-09

### Changed
- Updated to Rocket 0.5

## [1.4.0] - 2024-05-09

### Changed
- Various improvements and updates

## [1.3.0] - 2024-04-03

### Changed
- Various improvements and updates

## [1.2.0] - 2023-11-16

### Changed
- Various improvements and updates

## [1.1.0] - 2022-11-25

### Changed
- Various improvements and updates

## [1.0.0] - 2022-10-07

### Added
- Initial release

[Unreleased]: https://github.com/spawnia/md-to-pdf/compare/v1.6.0...HEAD
[1.6.0]: https://github.com/spawnia/md-to-pdf/compare/v1.5.0...v1.6.0
[1.5.0]: https://github.com/spawnia/md-to-pdf/compare/v1.4.5...v1.5.0
[1.4.5]: https://github.com/spawnia/md-to-pdf/compare/v1.4.4...v1.4.5
[1.4.4]: https://github.com/spawnia/md-to-pdf/compare/v1.4.3...v1.4.4
[1.4.3]: https://github.com/spawnia/md-to-pdf/compare/v1.4.2...v1.4.3
[1.4.2]: https://github.com/spawnia/md-to-pdf/compare/v1.4.1...v1.4.2
[1.4.1]: https://github.com/spawnia/md-to-pdf/compare/v1.4.0...v1.4.1
[1.4.0]: https://github.com/spawina/md-to-pdf/compare/v1.3.0...v1.4.0
[1.3.0]: https://github.com/spawnia/md-to-pdf/compare/v1.2.0...v1.3.0
[1.2.0]: https://github.com/spawnia/md-to-pdf/compare/v1.1.0...v1.2.0
[1.1.0]: https://github.com/spawnia/md-to-pdf/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/spawnia/md-to-pdf/releases/tag/v1.0.0
