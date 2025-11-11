#!/bin/bash
# SPDX-License-Identifier: Apache-2.0
# Sync protocol specifications and regenerate SDK code

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
PROTO_FILE="${REPO_ROOT}/specs/securefabric.proto"

cd "${REPO_ROOT}"

echo "========================================="
echo "SecureFabric Protocol Sync & Codegen"
echo "========================================="
echo ""

# Check if protoc is installed
if ! command -v protoc &> /dev/null; then
    echo "❌ ERROR: protoc not found. Please install protobuf-compiler"
    exit 1
fi

# Validate protocol file exists
if [ ! -f "${PROTO_FILE}" ]; then
    echo "❌ ERROR: Protocol file not found: ${PROTO_FILE}"
    exit 1
fi

echo "✅ Protocol file: ${PROTO_FILE}"
echo ""

# Validate protocol syntax
echo "🔍 Validating protocol syntax..."
if protoc --proto_path=specs --descriptor_set_out=/dev/null specs/securefabric.proto; then
    echo "✅ Protocol syntax valid"
else
    echo "❌ Protocol validation failed"
    exit 1
fi
echo ""

# Track if any codegen fails
CODEGEN_FAILED=0

# Rust SDK codegen
echo "========================================="
echo "Rust SDK Codegen"
echo "========================================="
if [ -d "sdk/rust" ]; then
    cd "${REPO_ROOT}/sdk/rust"

    if [ -f "Cargo.toml" ]; then
        echo "Running Rust codegen via cargo build..."

        # Rust uses prost-build in build.rs, so just build
        if cargo build --verbose; then
            echo "✅ Rust codegen complete"

            # Run format
            if command -v rustfmt &> /dev/null; then
                cargo fmt || true
            fi
        else
            echo "❌ Rust codegen failed"
            CODEGEN_FAILED=1
        fi
    else
        echo "⚠️  No Cargo.toml found, skipping Rust codegen"
    fi
else
    echo "⚠️  Rust SDK not found, skipping"
fi
echo ""

# Python SDK codegen
echo "========================================="
echo "Python SDK Codegen"
echo "========================================="
if [ -d "sdk/python" ]; then
    cd "${REPO_ROOT}/sdk/python"

    # Check if protobuf tools are available
    if python3 -c "import grpc_tools.protoc" 2>/dev/null; then
        echo "Running Python codegen..."

        python3 -m grpc_tools.protoc \
            -I../../specs \
            --python_out=securefabric \
            --grpc_python_out=securefabric \
            --pyi_out=securefabric \
            ../../specs/securefabric.proto

        if [ $? -eq 0 ]; then
            echo "✅ Python codegen complete"

            # Format generated files
            if command -v black &> /dev/null; then
                black securefabric/securefabric_pb2.py securefabric/securefabric_pb2_grpc.py securefabric/securefabric_pb2.pyi 2>/dev/null || true
            fi
        else
            echo "❌ Python codegen failed"
            CODEGEN_FAILED=1
        fi
    else
        echo "⚠️  grpcio-tools not installed, skipping Python codegen"
        echo "    Install with: pip install grpcio-tools"
    fi
else
    echo "⚠️  Python SDK not found, skipping"
fi
echo ""

# JavaScript/TypeScript SDK codegen
echo "========================================="
echo "JavaScript/TypeScript SDK Codegen"
echo "========================================="
if [ -d "sdk/js" ]; then
    cd "${REPO_ROOT}/sdk/js"

    # Check if we have package.json with codegen script
    if [ -f "package.json" ]; then
        if jq -e '.scripts.codegen' package.json > /dev/null 2>&1; then
            echo "Running JS/TS codegen..."
            if npm run codegen; then
                echo "✅ JS/TS codegen complete"
            else
                echo "❌ JS/TS codegen failed"
                CODEGEN_FAILED=1
            fi
        else
            echo "⚠️  No codegen script in package.json"
            echo "    Add: \"codegen\": \"protoc --plugin=./node_modules/.bin/protoc-gen-ts_proto --ts_proto_out=./src/generated -I../../specs ../../specs/securefabric.proto\""
        fi
    else
        echo "⚠️  No package.json found, skipping JS/TS codegen"
    fi
else
    echo "⚠️  JavaScript/TypeScript SDK not found, skipping"
fi
echo ""

# Check for drift
echo "========================================="
echo "Checking for Drift"
echo "========================================="
cd "${REPO_ROOT}"

if [ -n "$(git status --porcelain specs/ sdk/)" ]; then
    echo "⚠️  Changes detected after codegen:"
    git status --porcelain specs/ sdk/
    echo ""
    echo "This is expected when protocol changes."
    echo "Commit these changes with your protocol update."
else
    echo "✅ No drift detected - generated code is up to date"
fi
echo ""

if [ $CODEGEN_FAILED -eq 1 ]; then
    echo "❌ Some codegen steps failed. See errors above."
    exit 1
else
    echo "========================================="
    echo "✅ Protocol sync complete!"
    echo "========================================="
fi
