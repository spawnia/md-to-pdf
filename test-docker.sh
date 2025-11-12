#!/usr/bin/env bash

set -euo pipefail

IMAGE="${1:-md-to-pdf:test}"
PLATFORM="${2:-}"
PORT="${3:-8001}"
CONTAINER_NAME="md-to-pdf-test-$$"

PLATFORM_FLAG=""
if [ -n "${PLATFORM}" ]; then
  PLATFORM_FLAG="--platform=${PLATFORM}"
fi

echo "Testing Docker image: ${IMAGE}"
[ -n "${PLATFORM}" ] && echo "Platform: ${PLATFORM}"
echo

# =============================================================================
# Part 1: Validate tools
# =============================================================================

echo "==> Validating tools..."
echo

echo "Testing md-to-pdf binary..."
timeout 3 docker run --rm ${PLATFORM_FLAG} "${IMAGE}" md-to-pdf || [ $? -eq 124 ]
echo "✓ md-to-pdf binary works"

echo "Testing pandoc..."
docker run --rm ${PLATFORM_FLAG} "${IMAGE}" pandoc --version | head --lines=1
echo "✓ pandoc works"

echo "Testing wkhtmltopdf..."
docker run --rm ${PLATFORM_FLAG} "${IMAGE}" wkhtmltopdf --version
echo "✓ wkhtmltopdf works"

echo "Testing pdflatex..."
docker run --rm ${PLATFORM_FLAG} "${IMAGE}" pdflatex --version | head --lines=1
echo "✓ pdflatex works"

echo "Testing weasyprint..."
docker run --rm ${PLATFORM_FLAG} "${IMAGE}" weasyprint --version
echo "✓ weasyprint works"

echo
echo "All tools validated"

# =============================================================================
# Part 2: Test service
# =============================================================================

echo
echo "==> Testing service..."
echo

cleanup() {
  echo "Cleaning up container..."
  docker stop "${CONTAINER_NAME}" >/dev/null 2>&1 || true
  docker rm "${CONTAINER_NAME}" >/dev/null 2>&1 || true
}

trap cleanup EXIT

echo "Starting container on port ${PORT}..."
docker run --detach --name "${CONTAINER_NAME}" --publish "${PORT}:8000" ${PLATFORM_FLAG} "${IMAGE}"

echo "Waiting for service to be ready..."
for i in {1..30}; do
  if curl --silent --fail "http://localhost:${PORT}" >/dev/null 2>&1; then
    echo "Service is ready"
    break
  fi
  if [ $i -eq 30 ]; then
    echo "Service failed to start within 30 seconds"
    exit 1
  fi
  sleep 1
done

echo
echo "Testing API endpoint..."
curl --request POST \
  --data-urlencode "markdown=$(cat README.md)" \
  --data-urlencode "css=h1 { color: blue; }" \
  --output README.pdf \
  "localhost:${PORT}"

if [ -f README.pdf ] && [ -s README.pdf ]; then
  echo "✓ API request succeeded, PDF generated"
  rm README.pdf
else
  echo "✗ API request failed, no PDF generated"
  exit 1
fi

echo
echo "All tests passed"
