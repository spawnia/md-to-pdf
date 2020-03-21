#!/bin/sh -eux

curl -X POST --data "markdown=$(cat README.md)" --data "css=h1 { color: blue; }" --output README.pdf localhost:8000
