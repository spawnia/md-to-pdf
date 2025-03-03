#!/bin/sh -eux

# Create a sample markdown file with multiple paragraphs
cat > sample.md << 'EOL'
# Sample Document with Censored Content

## Introduction
This is the first paragraph. It will remain visible and clear. This paragraph contains important information about the document structure and purpose.

## Main Content
{{CENSOR}}

This is the third paragraph. It will remain visible and clear. Here we discuss the implementation details and technical specifications.

## Confidential Section
{{CENSOR}}

## Conclusion
This is the final paragraph. It will remain visible and clear. We summarize the key points and next steps here.
EOL

# Send the request
curl --request POST \
  --data-urlencode "markdown=$(cat sample.md)" \
  --header "Content-Type: application/x-www-form-urlencoded" \
  --output censored-sample.pdf \
  localhost:8000

# Clean up the temporary markdown file
rm sample.md

echo "Test completed! Check censored-sample.pdf for the result." 