# SPDX-License-Identifier: Apache-2.0
# Makefile for SecureFabric Python SDK

PYTHON := python3
PROTO_SRC := ../../specs/securefabric.proto
PROTO_OUT := securefabric

.PHONY: help codegen build test lint fmt check clean install dev-install

help: ## Show this help message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

codegen: ## Generate code from protocol buffers
	@echo "Running Python codegen..."
	$(PYTHON) -m grpc_tools.protoc \
		-I../../specs \
		--python_out=$(PROTO_OUT) \
		--grpc_python_out=$(PROTO_OUT) \
		--pyi_out=$(PROTO_OUT) \
		$(PROTO_SRC)
	black $(PROTO_OUT)/securefabric_pb2.py $(PROTO_OUT)/securefabric_pb2_grpc.py $(PROTO_OUT)/securefabric_pb2.pyi
	@echo "✅ Codegen complete"

install: ## Install the SDK
	pip install .

dev-install: ## Install the SDK in development mode with dev dependencies
	pip install -e '.[dev]'

build: codegen ## Build the Python SDK
	$(PYTHON) -m build

test: ## Run tests
	pytest -v

test-conformance: ## Run conformance tests only
	pytest tests/test_conformance.py -v

lint: ## Run linters
	flake8 $(PROTO_OUT) tests
	mypy $(PROTO_OUT)

fmt: ## Format code
	black $(PROTO_OUT) tests
	isort $(PROTO_OUT) tests

check: fmt lint test ## Run all checks (format, lint, test)

clean: ## Clean build artifacts
	rm -rf build dist *.egg-info
	find . -type d -name __pycache__ -exec rm -rf {} +
	find . -type f -name '*.pyc' -delete
	find . -type d -name .pytest_cache -exec rm -rf {} +
	find . -type d -name .mypy_cache -exec rm -rf {} +

.DEFAULT_GOAL := help
