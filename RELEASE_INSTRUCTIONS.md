# Release Instructions for v1.6.1

This document provides step-by-step instructions for creating and publishing release v1.6.1.

## Prerequisites

- All changes have been merged to `master` branch
- CI/CD pipelines are passing on `master`
- Release notes have been prepared (see `RELEASE_NOTES_v1.6.1.md`)
- CHANGELOG.md has been updated

## Release Process

### Step 1: Verify Master Branch

```bash
git checkout master
git pull origin master
```

Ensure you're on the latest commit on master and all checks are passing.

### Step 2: Create and Push Git Tag

```bash
# Create an annotated tag for v1.6.1
git tag -a v1.6.1 -m "Release v1.6.1"

# Push the tag to GitHub
git push origin v1.6.1
```

### Step 3: Automated CI/CD

The push of the tag will automatically trigger:

1. **Docker Workflow** (`.github/workflows/docker.yml`):
   - Builds multi-architecture Docker images (linux/amd64, linux/arm64)
   - Pushes images to Docker Hub as:
     - `spawnia/md-to-pdf:v1.6.1`
     - `spawnia/md-to-pdf:latest`
   - Runs multi-architecture tests

### Step 4: Create GitHub Release

1. Go to https://github.com/spawnia/md-to-pdf/releases/new
2. Select tag: `v1.6.1`
3. Release title: `v1.6.1`
4. Copy content from `RELEASE_NOTES_v1.6.1.md` into the description
5. Verify "Set as the latest release" is checked
6. Click "Publish release"

### Step 5: Verify Deployment

1. **Docker Hub**: Verify images are published at https://hub.docker.com/r/spawnia/md-to-pdf/tags
2. **GitHub Actions**: Check that all workflows completed successfully
3. **Fly.io**: The service should automatically redeploy (if configured)
4. **Test**: Run a quick test:
   ```bash
   docker run --rm spawnia/md-to-pdf:v1.6.1 pandoc --version
   ```

### Step 6: Announce

Optionally announce the release:
- Update any external documentation if needed
- Notify users of significant changes

## Rollback

If issues are discovered:

1. Delete the tag locally and remotely:
   ```bash
   git tag -d v1.6.1
   git push origin :refs/tags/v1.6.1
   ```

2. Delete the GitHub release

3. Fix the issues and restart the process with a new patch version (v1.6.2)

## Notes

- The repository does not have a version field in `Cargo.toml` - versioning is done entirely through Git tags
- Docker images are automatically built for both AMD64 and ARM64 architectures
- The Docker workflow only builds multi-arch images on tag pushes (not on regular commits to master)

## What's in this Release

v1.6.1 includes:
- Fixed Fly.io deployment lease conflict issues
- Updated dependencies (time 0.3.47, bytes 1.11.1)
- Added Copilot instructions

See `CHANGELOG.md` and `RELEASE_NOTES_v1.6.1.md` for full details.
