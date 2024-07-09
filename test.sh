#!/bin/sh -eux

curl --request POST \
  --data-urlencode "markdown=$(cat README.md)" \
  --data-urlencode "css=h1 { color: blue; }" \
  --output README.pdf \
  localhost:8000
