# SPDX-License-Identifier: Apache-2.0
# Makefile for SecureFabric Rust SDK

.PHONY: help codegen build test lint fmt check clean

help: ## Show this help message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

codegen: ## Generate code from protocol buffers
	@echo "Running Rust codegen..."
	cargo build --verbose
	@echo "✅ Codegen complete"

build: ## Build the Rust SDK
	cargo build --verbose

build-release: ## Build the Rust SDK in release mode
	cargo build --release --verbose

test: ## Run tests
	cargo test --verbose

test-conformance: ## Run conformance tests only
	cargo test conformance --verbose

lint: ## Run clippy linter
	cargo clippy --all-features -- -D warnings

fmt: ## Format code
	cargo fmt

check: fmt lint test ## Run all checks (format, lint, test)

clean: ## Clean build artifacts
	cargo clean

audit: ## Run security audit on dependencies
	cargo audit

.DEFAULT_GOAL := help
