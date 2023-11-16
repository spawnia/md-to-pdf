dcrust=$$( [ -f /.dockerenv ] && echo "" || echo "docker-compose exec rust")
dcpandoc=$$( [ -f /.dockerenv ] && echo "" || echo "docker-compose exec pandoc")

.PHONY: it
it: fmt target/debug ## Perform common targets

.PHONY: help
help: ## Displays this list of targets with descriptions
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(firstword $(MAKEFILE_LIST)) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[32m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY: setup
setup: dc-build cargo-deps ## Set up the local environment

.PHONY: dc-build
dc-build: ## Build the local dev image
	docker-compose build --pull

.PHONY: up
up: ## Bring up the containers
	[ -f /.dockerenv ] || docker-compose up --detach

.PHONY: cargo-deps
cargo-deps: up ## Reinstall cargo dependencies
	${dcrust} cargo update

target/debug: up src ## Compile
	${dcrust} cargo build

.PHONY: rust
rust: up ## Enter an interactive shell into the rust container
	${dcrust} bash

.PHONY: pandoc
pandoc: up ## Enter an interactive shell into the pandoc container
	${dcpandoc} bash

.PHONY: serve
serve: up target/debug ## Serve the compiled application
	${dcpandoc} target/debug/md-to-pdf

.PHONY: fmt
fmt: up ## Format the rust code
	${dcrust} cargo fmt

.PHONY: test
test: ## Issue a dummy request against the API
	./test.sh
