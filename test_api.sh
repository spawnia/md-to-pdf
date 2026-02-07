#!/usr/bin/env bash
# Integration test script for md-to-pdf Document Engine API
# Usage: ./test_api.sh [base_url]

set -euo pipefail

BASE_URL="${1:-http://localhost:8000}"
PASS=0
FAIL=0

green() { echo -e "\033[0;32m$1\033[0m"; }
red()   { echo -e "\033[0;31m$1\033[0m"; }

check() {
  local name="$1"
  local expected_code="$2"
  local actual_code="$3"
  if [ "$actual_code" = "$expected_code" ]; then
    green "  ✓ $name (HTTP $actual_code)"
    PASS=$((PASS + 1))
  else
    red "  ✗ $name — expected HTTP $expected_code, got $actual_code"
    FAIL=$((FAIL + 1))
  fi
}

echo "=== md-to-pdf API Integration Tests ==="
echo "Base URL: $BASE_URL"
echo

# -----------------------------------------------------------
# 1. Health check
# -----------------------------------------------------------
echo "--- GET /api/health ---"
CODE=$(curl -s -o /tmp/health.json -w "%{http_code}" "$BASE_URL/api/health")
check "health check" 200 "$CODE"
echo "  Response: $(cat /tmp/health.json)"
echo

# -----------------------------------------------------------
# 2. Legacy POST / (FormData, backward compat) — returns PDF
# -----------------------------------------------------------
echo "--- POST / (legacy FormData) ---"
CODE=$(curl -s -o /tmp/legacy.pdf -w "%{http_code}" \
  -F "markdown=# Hello World" \
  "$BASE_URL/")
check "legacy convert (PDF response)" 200 "$CODE"
echo

# -----------------------------------------------------------
# 3. Legacy POST / with client_id/pdf_name — returns JSON
# -----------------------------------------------------------
echo "--- POST / (legacy FormData, save) ---"
CODE=$(curl -s -o /tmp/legacy_save.json -w "%{http_code}" \
  -F "markdown=# Saved Document" \
  -F "client_id=test-client" \
  -F "pdf_name=test-legacy" \
  "$BASE_URL/")
check "legacy convert (save → JSON)" 200 "$CODE"
echo "  Response: $(cat /tmp/legacy_save.json)"
echo

# -----------------------------------------------------------
# 4. POST /api/convert — Markdown → PDF (JSON)
# -----------------------------------------------------------
echo "--- POST /api/convert ---"
CODE=$(curl -s -o /tmp/convert.json -w "%{http_code}" \
  -H "Content-Type: application/json" \
  -d '{
    "markdown": "# API Convert Test\n\nThis is a **test** document.",
    "client_id": "test-client",
    "pdf_name": "test-convert",
    "options": {
      "paper_size": "a4",
      "page_numbers": true
    }
  }' \
  "$BASE_URL/api/convert")
check "convert markdown → PDF" 200 "$CODE"
echo "  Response: $(cat /tmp/convert.json)"
echo

# -----------------------------------------------------------
# 5. POST /api/html-to-pdf — HTML → PDF (JSON)
# -----------------------------------------------------------
echo "--- POST /api/html-to-pdf ---"
CODE=$(curl -s -o /tmp/html2pdf.json -w "%{http_code}" \
  -H "Content-Type: application/json" \
  -d '{
    "html": "<html><body><h1>HTML to PDF</h1><p>Direct HTML conversion.</p></body></html>",
    "client_id": "test-client",
    "pdf_name": "test-html2pdf"
  }' \
  "$BASE_URL/api/html-to-pdf")
check "html-to-pdf" 200 "$CODE"
echo "  Response: $(cat /tmp/html2pdf.json)"
echo

# -----------------------------------------------------------
# 6. POST /api/render — Tera template → PDF
# -----------------------------------------------------------
echo "--- POST /api/render ---"
CODE=$(curl -s -o /tmp/render.json -w "%{http_code}" \
  -H "Content-Type: application/json" \
  -d '{
    "template": "<html><body><h1>{{ title }}</h1><p>Dear {{ name }},</p><p>{{ body }}</p></body></html>",
    "data": {
      "title": "Invoice #123",
      "name": "John Doe",
      "body": "Thank you for your purchase."
    },
    "client_id": "test-client",
    "pdf_name": "test-render"
  }' \
  "$BASE_URL/api/render")
check "render template → PDF" 200 "$CODE"
echo "  Response: $(cat /tmp/render.json)"
echo

# -----------------------------------------------------------
# 7. POST /api/preview — Markdown → PNG
# -----------------------------------------------------------
echo "--- POST /api/preview ---"
CODE=$(curl -s -o /tmp/preview.png -w "%{http_code}" \
  -H "Content-Type: application/json" \
  -d '{
    "markdown": "# Preview Test\n\nThis should produce a PNG."
  }' \
  "$BASE_URL/api/preview")
check "preview markdown → PNG" 200 "$CODE"
echo

# -----------------------------------------------------------
# 8. POST /api/merge — Merge PDFs
# -----------------------------------------------------------
echo "--- POST /api/merge ---"
CODE=$(curl -s -o /tmp/merge.json -w "%{http_code}" \
  -H "Content-Type: application/json" \
  -d '{
    "pdfs": [
      "/download/test-client/test-convert.pdf",
      "/download/test-client/test-html2pdf.pdf"
    ],
    "client_id": "test-client",
    "pdf_name": "test-merged"
  }' \
  "$BASE_URL/api/merge")
check "merge PDFs" 200 "$CODE"
echo "  Response: $(cat /tmp/merge.json)"
echo

# -----------------------------------------------------------
# 9. POST /api/watermark — Add watermark
# -----------------------------------------------------------
echo "--- POST /api/watermark ---"
CODE=$(curl -s -o /tmp/watermark.json -w "%{http_code}" \
  -H "Content-Type: application/json" \
  -d '{
    "pdf": "/download/test-client/test-convert.pdf",
    "text": "DRAFT",
    "opacity": 0.1,
    "angle": -45,
    "client_id": "test-client",
    "pdf_name": "test-watermarked"
  }' \
  "$BASE_URL/api/watermark")
check "watermark PDF" 200 "$CODE"
echo "  Response: $(cat /tmp/watermark.json)"
echo

# -----------------------------------------------------------
# 10. POST /api/protect — Password protect
# -----------------------------------------------------------
echo "--- POST /api/protect ---"
CODE=$(curl -s -o /tmp/protect.json -w "%{http_code}" \
  -H "Content-Type: application/json" \
  -d '{
    "pdf": "/download/test-client/test-convert.pdf",
    "password": "secret123",
    "client_id": "test-client",
    "pdf_name": "test-protected"
  }' \
  "$BASE_URL/api/protect")
check "protect PDF" 200 "$CODE"
echo "  Response: $(cat /tmp/protect.json)"
echo

# -----------------------------------------------------------
# 11. GET /download — Download saved PDF
# -----------------------------------------------------------
echo "--- GET /download (saved PDF) ---"
CODE=$(curl -s -o /tmp/downloaded.pdf -w "%{http_code}" \
  "$BASE_URL/download/test-client/test-convert.pdf")
check "download saved PDF" 200 "$CODE"
echo

# -----------------------------------------------------------
# Error cases
# -----------------------------------------------------------
echo "--- Error cases ---"

# Bad request: merge with < 2 PDFs
CODE=$(curl -s -o /tmp/err_merge.json -w "%{http_code}" \
  -H "Content-Type: application/json" \
  -d '{"pdfs": ["/download/test-client/test-convert.pdf"]}' \
  "$BASE_URL/api/merge")
check "merge with 1 PDF → 400" 400 "$CODE"

# Bad request: render with non-object data
CODE=$(curl -s -o /tmp/err_render.json -w "%{http_code}" \
  -H "Content-Type: application/json" \
  -d '{"template": "<h1>test</h1>", "data": "not an object"}' \
  "$BASE_URL/api/render")
check "render with bad data → 400" 400 "$CODE"

# Not found: download non-existent file
CODE=$(curl -s -o /dev/null -w "%{http_code}" \
  "$BASE_URL/download/no-such-client/no-such-file.pdf")
check "download non-existent → 404" 404 "$CODE"

echo
echo "========================================="
echo "Results: $PASS passed, $FAIL failed"
echo "========================================="

[ "$FAIL" -eq 0 ] && exit 0 || exit 1
