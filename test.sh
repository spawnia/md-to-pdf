#!/bin/sh -eux

curl -X POST --data-urlencode "markdown=$(cat README.md)" --data-urlencode "css=h1 { color: blue; }" --output README.pdf localhost:8000
