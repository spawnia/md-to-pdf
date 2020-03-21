.PHONY: test
test:
	curl -X POST --data 'markdown=# Heading 2' --data 'css=h1 { color: blue; }' --output test.pdf localhost:8000
