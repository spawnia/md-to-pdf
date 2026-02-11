# Release v1.6.1

## Bug Fixes

### Fly.io Deployment Improvements
Fixed critical issues with Fly.io deployments that were causing lease conflicts and deployment failures:

- **Added concurrency control** to prevent overlapping deployments to the same Fly.io app
- **Improved timeout handling**:
  - Added `--lease-timeout=120s` flag to better handle lease conflicts
  - Added `--wait-timeout=600` flag to allow sufficient time for deployments to complete
  - Fixed timeout unit inconsistency (now using seconds consistently)
- **Simplified concurrency configuration** to use static concurrency group for all deployments

These changes ensure more reliable deployments and eliminate the "lease conflict" errors that were preventing successful updates.

## Dependencies

Updated indirect dependencies for security and compatibility:
- `time`: 0.3.44 → 0.3.47
- `bytes`: 1.10.1 → 1.11.1

## Internal Changes

- Added Copilot instructions to improve AI-assisted development workflow

---

**Full Changelog**: https://github.com/spawnia/md-to-pdf/compare/v1.6.0...v1.6.1
