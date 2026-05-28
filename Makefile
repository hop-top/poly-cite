.PHONY: help setup test test-go test-ts test-py test-rs test-php test-parity \
	build build-go build-ts build-py build-rs build-php \
	package package-go package-ts package-py package-rs package-php \
	lint lint-docs lint-rs lint-py lint-whitespace check clean

PYTHON ?= python3
PNPM ?= pnpm
COMPOSER ?= composer
GOCACHE ?= /tmp/cite-go-build

help: ## Show available targets
	@awk 'BEGIN {FS = ":.*## "; printf "Targets:\n"} /^[a-zA-Z0-9_-]+:.*## / {printf "  %-18s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

setup: ## Install local dependencies for every language package
	cd ts && $(PNPM) install --ignore-scripts
	cd py && $(PYTHON) -m pip install -U pip pytest build
	cd php && $(COMPOSER) install --no-interaction --no-progress

check: lint test build ## Run docs lint, all tests, and all builds

test: test-go test-ts test-py test-rs test-php test-parity ## Run all language tests and parity checks

test-go: ## Run Go tests
	cd go && env -u GOROOT GOCACHE=$(GOCACHE) go test ./...

test-ts: ## Run TypeScript tests
	cd ts && $(PNPM) dlx --config.ignore-scripts=true vitest@2.1.8 run

test-py: ## Run Python tests
	cd py && PYTHONPATH=src $(PYTHON) -m pytest -q

test-rs: ## Run Rust tests
	cd rs && cargo test

test-php: ## Run PHP tests
	cd php && ./vendor/bin/phpunit

test-parity: build-ts ## Prove Go/TS/Python/Rust/PHP parity against spec fixtures
	$(PYTHON) tools/parity/parity.py

build: build-go build-ts build-py build-rs build-php ## Build all language packages

build-go: ## Verify Go packages compile
	cd go && env -u GOROOT GOCACHE=$(GOCACHE) go test ./... -run '^$$'

build-ts: ## Build TypeScript package
	cd ts && $(PNPM) install --ignore-scripts && $(PNPM) build

build-py: ## Build Python source and wheel distributions
	cd py && $(PYTHON) -m pip install -q build && $(PYTHON) -m build

build-rs: ## Build Rust package
	cd rs && cargo build

build-php: ## Validate PHP package autoload metadata
	cd php && $(COMPOSER) validate --strict

package: package-go package-ts package-py package-rs package-php ## Create package artifacts without publishing

package-go: ## Validate Go module package list
	cd go && env -u GOROOT GOCACHE=$(GOCACHE) go list ./...

package-ts: build-ts ## Pack TypeScript package
	cd ts && $(PNPM) pack

package-py: build-py ## Build Python package

package-rs: ## Validate Rust package contents
	cd rs && cargo package --allow-dirty --no-verify

package-php: ## Validate PHP package metadata
	cd php && $(COMPOSER) validate --strict

lint: lint-docs lint-rs lint-py lint-whitespace ## Run available linters

lint-docs: ## Validate docs frontmatter spacing and required keys
	@bad=0; \
	for f in $$(find docs -type f -name '*.md' | sort); do \
		if [ "$$(sed -n '1p' "$$f")" = '---' ]; then \
			second="$$(sed -n '2p' "$$f")"; \
			if [ -z "$$second" ]; then echo "blank-after-opening $$f"; bad=1; fi; \
			close="$$(awk 'NR>1 && $$0=="---" {print NR; exit}' "$$f")"; \
			next="$$((close+1))"; \
			line="$$(sed -n "$${next}p" "$$f")"; \
			if [ -n "$$line" ]; then echo "missing-blank-after-closing $$f"; bad=1; fi; \
			for key in doc_type subtype status title summary owner created updated audience confidentiality tags; do \
				if ! sed -n '1,/^---$$/p' "$$f" | grep -Eq "^$$key:"; then echo "missing-$$key $$f"; bad=1; fi; \
			done; \
		fi; \
	done; \
	exit $$bad

lint-rs: ## Check Rust formatting and clippy warnings
	cd rs && cargo fmt --check
	cd rs && cargo clippy --all-targets --all-features -- -D warnings

lint-py: ## Check Python syntax without writing pycache into user cache dirs
	cd py && PYTHONPYCACHEPREFIX=/tmp/cite-pycache $(PYTHON) -m compileall -q src tests

lint-whitespace: ## Check whitespace and conflict-marker issues in the git diff
	git diff --check

clean: ## Remove generated local build artifacts
	rm -rf py/dist py/*.egg-info rs/target ts/dist php/vendor
