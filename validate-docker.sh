#!/usr/bin/env bash

set -euo pipefail

IMAGE="${1:-md-to-pdf:test}"

echo "Validating Docker image: ${IMAGE}"
echo

# Validate md-to-pdf binary can start (timeout after 3s, exit code 124 means it started successfully)
echo "Testing md-to-pdf binary..."
timeout 3 docker run --rm "${IMAGE}" md-to-pdf || [ $? -eq 124 ]
echo "✓ md-to-pdf binary works"
echo

# Validate pandoc
echo "Testing pandoc..."
docker run --rm "${IMAGE}" pandoc --version | head --lines=1
echo "✓ pandoc works"
echo

# Validate wkhtmltopdf
echo "Testing wkhtmltopdf..."
docker run --rm "${IMAGE}" wkhtmltopdf --version
echo "✓ wkhtmltopdf works"
echo

# Validate pdflatex
echo "Testing pdflatex..."
docker run --rm "${IMAGE}" pdflatex --version | head --lines=1
echo "✓ pdflatex works"
echo

# Validate weasyprint
echo "Testing weasyprint..."
docker run --rm "${IMAGE}" weasyprint --version
echo "✓ weasyprint works"
echo

echo "All validations passed"
