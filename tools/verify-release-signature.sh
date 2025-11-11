#!/bin/bash
# SPDX-License-Identifier: Apache-2.0
#
# Verify SecureFabric release signatures
#
# Usage: ./verify-release-signature.sh <binary-file> <signature-file>

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Public key for verifying releases (ed25519)
# NOTE: This is a placeholder. Replace with actual public key from secure-fabric.io
PUBLIC_KEY="AAAAC3NzaC1lZDI1NTE5AAAAPLACEHOLDER_REPLACE_WITH_ACTUAL_KEY"

usage() {
    cat <<EOF
Usage: $0 <binary-file> <signature-file>

Verify the cryptographic signature of a SecureFabric release binary.

Arguments:
  binary-file     Path to the SecureFabric node binary
  signature-file  Path to the corresponding .sig file

Example:
  $0 securefabric-node-v1.0.0-linux-amd64 securefabric-node-v1.0.0-linux-amd64.sig

Requirements:
  - openssl or gpg for signature verification
  - curl or wget for fetching public keys

Download:
  Binaries and signatures are available at:
  https://secure-fabric.io/downloads
EOF
    exit 1
}

check_requirements() {
    local missing_tools=()

    if ! command -v openssl &> /dev/null && ! command -v gpg &> /dev/null; then
        missing_tools+=("openssl or gpg")
    fi

    if [ ${#missing_tools[@]} -gt 0 ]; then
        echo -e "${RED}Error: Missing required tools: ${missing_tools[*]}${NC}"
        echo "Install with:"
        echo "  Ubuntu/Debian: apt-get install openssl"
        echo "  macOS: brew install openssl"
        echo "  RHEL/CentOS: yum install openssl"
        exit 1
    fi
}

verify_with_openssl() {
    local binary_file="$1"
    local signature_file="$2"
    local public_key="$3"

    echo "Verifying signature with OpenSSL..."

    # Create temporary file for public key
    local pubkey_file=$(mktemp)
    echo "$public_key" > "$pubkey_file"

    # Verify signature
    if openssl dgst -sha256 -verify "$pubkey_file" -signature "$signature_file" "$binary_file" &>/dev/null; then
        rm -f "$pubkey_file"
        return 0
    else
        rm -f "$pubkey_file"
        return 1
    fi
}

main() {
    if [ $# -ne 2 ]; then
        usage
    fi

    local binary_file="$1"
    local signature_file="$2"

    # Check if files exist
    if [ ! -f "$binary_file" ]; then
        echo -e "${RED}Error: Binary file not found: $binary_file${NC}"
        exit 1
    fi

    if [ ! -f "$signature_file" ]; then
        echo -e "${RED}Error: Signature file not found: $signature_file${NC}"
        exit 1
    fi

    check_requirements

    echo "==============================================="
    echo "SecureFabric Release Signature Verification"
    echo "==============================================="
    echo ""
    echo "Binary:    $binary_file"
    echo "Signature: $signature_file"
    echo ""

    # Calculate checksum
    echo "SHA256:    $(sha256sum "$binary_file" | cut -d' ' -f1)"
    echo ""

    # Verify signature
    if verify_with_openssl "$binary_file" "$signature_file" "$PUBLIC_KEY"; then
        echo -e "${GREEN}✓ Signature verification PASSED${NC}"
        echo ""
        echo "This binary is authentically signed by SecureFabric."
        echo "It is safe to proceed with installation."
        exit 0
    else
        echo -e "${RED}✗ Signature verification FAILED${NC}"
        echo ""
        echo "WARNING: This binary may have been tampered with!"
        echo ""
        echo "Possible causes:"
        echo "  - Binary was modified after signing"
        echo "  - Signature file is corrupted"
        echo "  - Binary is from an untrusted source"
        echo ""
        echo "DO NOT install or run this binary!"
        echo ""
        echo "Action items:"
        echo "  1. Re-download from official source: https://secure-fabric.io/downloads"
        echo "  2. Verify checksums match official release notes"
        echo "  3. Contact security@secure-fabric.io if issue persists"
        exit 1
    fi
}

main "$@"
