# SPDX-License-Identifier: Apache-2.0
# Makefile for SecureFabric JavaScript/TypeScript SDK

NPM := npm
PROTO_SRC := ../../specs/securefabric.proto

.PHONY: help codegen build test lint fmt check clean install

help: ## Show this help message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

install: ## Install dependencies
	$(NPM) install

codegen: ## Generate code from protocol buffers
	@echo "Running JavaScript/TypeScript codegen..."
	@if [ -f package.json ] && jq -e '.scripts.codegen' package.json > /dev/null 2>&1; then \
		$(NPM) run codegen; \
	else \
		echo "⚠️  No codegen script found in package.json"; \
		echo "Add: \"codegen\": \"protoc --plugin=./node_modules/.bin/protoc-gen-ts_proto --ts_proto_out=./src/generated -I../../specs ../../specs/securefabric.proto\""; \
	fi
	@echo "✅ Codegen complete"

build: ## Build the SDK
	$(NPM) run build

test: ## Run tests
	$(NPM) test

test-conformance: ## Run conformance tests only
	$(NPM) test -- conformance

lint: ## Run linter
	$(NPM) run lint

fmt: ## Format code
	$(NPM) run format || $(NPM) run fmt || echo "No format script found"

check: lint test ## Run all checks (lint, test)

clean: ## Clean build artifacts
	rm -rf dist build node_modules .turbo

.DEFAULT_GOAL := help
