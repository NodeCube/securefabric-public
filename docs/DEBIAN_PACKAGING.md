# Debian Packaging for SecureFabric Node

This document describes how to build and distribute Debian packages for the SecureFabric node binary.

> **Note**: The SecureFabric node is maintained in the private `securefabric-core` repository.
> This documentation is provided here for reference and transparency.

## Overview

The `securefabric-node` Debian package provides:

- **Binary**: `/usr/bin/securefabric-node` - The SecureFabric node daemon
- **Systemd unit**: `/lib/systemd/system/securefabric-node.service` - System service
- **User/Group**: `securefabric` - Dedicated service account
- **Data directory**: `/var/lib/securefabric` - Node data and state
- **Configuration**: `/etc/securefabric/.env` - Environment configuration

## Package Metadata

| Field | Value |
|-------|-------|
| Package | securefabric-node |
| Architecture | amd64, arm64 |
| Section | net |
| Priority | optional |
| Maintainer | NodeCube <contact@secure-fabric.io> |
| License | Proprietary - Commercial |

## Debian Directory Structure

In the `securefabric-core` repository, the following structure should be created:

```text
securefabric-core/
├── debian/
│   ├── control              # Package metadata and dependencies
│   ├── rules                # Build instructions
│   ├── changelog            # Debian changelog
│   ├── copyright            # License information
│   ├── install              # File installation mappings
│   ├── postinst             # Post-installation script
│   ├── prerm                # Pre-removal script
│   ├── postrm               # Post-removal script
│   └── securefabric-node.service  # Systemd unit file
├── scripts/
│   └── build_deb.sh         # Helper script to build .deb
└── Makefile                 # Build automation
```

## Debian Control File

Create `debian/control`:

```bash
Source: securefabric-node
Section: net
Priority: optional
Maintainer: NodeCube <contact@secure-fabric.io>
Build-Depends: debhelper-compat (= 13),
               cargo,
               rustc (>= 1.70),
               protobuf-compiler,
               libssl-dev,
               pkg-config
Standards-Version: 4.6.0
Homepage: https://secure-fabric.io
Vcs-Git: https://github.com/NodeCube/securefabric-core.git
Vcs-Browser: https://github.com/NodeCube/securefabric-core

Package: securefabric-node
Architecture: amd64 arm64
Depends: ${shlibs:Depends}, ${misc:Depends},
         adduser,
         systemd
Recommends: ca-certificates
Description: SecureFabric messaging node
 SecureFabric is a secure, low-latency messaging fabric designed for
 verified senders and end-to-end confidentiality.
 .
 This package contains the SecureFabric node daemon, which provides:
  - Peer-to-peer gossip protocol
  - Message routing and delivery
  - E2E encryption and authentication
  - Topic-based pub/sub
  - gRPC API for clients
```

## Debian Rules File

Create `debian/rules`:

```bash
#!/usr/bin/make -f
# SPDX-License-Identifier: Proprietary

export DH_VERBOSE = 1
export CARGO_HOME = $(CURDIR)/debian/cargo

%:
    dh $@

override_dh_auto_build:
    cargo build --release --locked

override_dh_auto_install:
    install -D -m 0755 target/release/securefabric-node \
        debian/securefabric-node/usr/bin/securefabric-node
    install -D -m 0644 debian/securefabric-node.service \
        debian/securefabric-node/lib/systemd/system/securefabric-node.service

override_dh_auto_clean:
    cargo clean || true
    rm -rf debian/cargo

override_dh_auto_test:
    # Skip tests during package build (run in CI instead)
```

Make it executable:

```bash
chmod +x debian/rules
```

## Debian Install File

Create `debian/install` (alternative to using rules):

```bash
target/release/securefabric-node usr/bin/
debian/securefabric-node.service lib/systemd/system/
```

## Debian Changelog

Create `debian/changelog`:

```text
securefabric-node (0.1.0-1) unstable; urgency=medium

  * Initial release
  * Support for peer-to-peer gossip protocol
  * Message routing with E2E encryption
  * gRPC API for clients
  * Systemd integration

 -- NodeCube <contact@secure-fabric.io>  Mon, 18 Nov 2024 12:00:00 +0000
```

## Systemd Unit File

Create `debian/securefabric-node.service`:

```bash
[Unit]
Description=SecureFabric Node
Documentation=https://secure-fabric.io/docs
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
User=securefabric
Group=securefabric
WorkingDirectory=/var/lib/securefabric

# Environment configuration
EnvironmentFile=-/etc/securefabric/.env

# Binary and arguments
ExecStart=/usr/bin/securefabric-node

# Restart policy
Restart=always
RestartSec=10s
StartLimitInterval=5min
StartLimitBurst=3

# Security hardening
NoNewPrivileges=true
ProtectSystem=strict
ProtectHome=true
PrivateTmp=true
PrivateDevices=true
ProtectKernelTunables=true
ProtectKernelModules=true
ProtectControlGroups=true
RestrictAddressFamilies=AF_UNIX AF_INET AF_INET6
RestrictNamespaces=true
LockPersonality=true
MemoryDenyWriteExecute=true
RestrictRealtime=true
RestrictSUIDSGID=true
RemoveIPC=true
PrivateMounts=true

# Read-write paths
ReadWritePaths=/var/lib/securefabric

# Logging
StandardOutput=journal
StandardError=journal
SyslogIdentifier=securefabric-node

# Resource limits
LimitNOFILE=65536
TasksMax=4096

[Install]
WantedBy=multi-user.target
```

## Post-Installation Script

Create `debian/postinst`:

```bash
#!/bin/bash
# SPDX-License-Identifier: Proprietary

set -e

case "$1" in
    configure)
        # Create securefabric user if it doesn't exist
        if ! getent passwd securefabric > /dev/null; then
            adduser --system --group --no-create-home \
                --home /var/lib/securefabric \
                --gecos "SecureFabric Node" \
                securefabric
        fi

        # Create directories
        mkdir -p /var/lib/securefabric
        mkdir -p /etc/securefabric

        # Set permissions
        chown securefabric:securefabric /var/lib/securefabric
        chmod 0750 /var/lib/securefabric
        chmod 0750 /etc/securefabric

        # Create default environment file if it doesn't exist
        if [ ! -f /etc/securefabric/.env ]; then
            cat > /etc/securefabric/.env <<EOF
# SecureFabric Node Configuration
# Edit this file to configure your node

# Listen address and port
SECUREFABRIC_BIND=0.0.0.0:50051

# TLS certificate paths (required for production)
#SECUREFABRIC_TLS_CERT=/etc/securefabric/tls/cert.pem
#SECUREFABRIC_TLS_KEY=/etc/securefabric/tls/key.pem

# Peer nodes (comma-separated)
#SECUREFABRIC_PEERS=node1.example.com:50051,node2.example.com:50051

# Logging level (debug, info, warn, error)
SECUREFABRIC_LOG_LEVEL=info

# Data directory
SECUREFABRIC_DATA_DIR=/var/lib/securefabric
EOF
            chown root:securefabric /etc/securefabric/.env
            chmod 0640 /etc/securefabric/.env
        fi

        # Reload systemd
        systemctl daemon-reload

        # Enable but don't start (user must configure first)
        systemctl enable securefabric-node.service || true

        echo "=================================================="
        echo "SecureFabric Node installed successfully!"
        echo ""
        echo "Next steps:"
        echo "  1. Configure: /etc/securefabric/.env"
        echo "  2. Start: systemctl start securefabric-node"
        echo "  3. Status: systemctl status securefabric-node"
        echo "  4. Logs: journalctl -u securefabric-node -f"
        echo "=================================================="
        ;;

    *)
        ;;
esac

#DEBHELPER#

exit 0
```

Make it executable:

```bash
chmod +x debian/postinst
```

## Pre-Removal Script

Create `debian/prerm`:

```bash
#!/bin/bash
# SPDX-License-Identifier: Proprietary

set -e

case "$1" in
    remove|upgrade|deconfigure)
        # Stop service before removal
        systemctl stop securefabric-node.service || true
        ;;

    *)
        ;;
esac

#DEBHELPER#

exit 0
```

Make it executable:

```bash
chmod +x debian/prerm
```

## Post-Removal Script

Create `debian/postrm`:

```bash
#!/bin/bash
# SPDX-License-Identifier: Proprietary

set -e

case "$1" in
    purge)
        # Remove user and group (only on purge, not remove)
        if getent passwd securefabric > /dev/null; then
            deluser --system securefabric || true
        fi

        # Remove data directory (only on purge)
        rm -rf /var/lib/securefabric
        rm -rf /etc/securefabric
        ;;

    remove|upgrade|failed-upgrade|abort-install|abort-upgrade|disappear)
        # Keep data on remove/upgrade
        ;;

    *)
        ;;
esac

#DEBHELPER#

exit 0
```

Make it executable:

```bash
chmod +x debian/postrm
```

## Build Script

Create `scripts/build_deb.sh`:

```bash
#!/bin/bash
# SPDX-License-Identifier: Proprietary
#
# Build Debian package for SecureFabric node

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Building SecureFabric Node Debian package...${NC}"

# Check dependencies
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: cargo not found. Install Rust toolchain.${NC}"
    exit 1
fi

if ! command -v dpkg-buildpackage &> /dev/null; then
    echo -e "${RED}Error: dpkg-buildpackage not found. Install: apt-get install dpkg-dev${NC}"
    exit 1
fi

# Build release binary first
echo -e "${YELLOW}Building release binary...${NC}"
cargo build --release --locked

# Create dist directory
mkdir -p dist/deb

# Build Debian package
echo -e "${YELLOW}Building Debian package...${NC}"
dpkg-buildpackage -us -uc -b

# Move .deb to dist directory
echo -e "${YELLOW}Moving package to dist/deb/...${NC}"
mv ../securefabric-node_*.deb dist/deb/ || true
mv ../securefabric-node_*.buildinfo dist/deb/ || true
mv ../securefabric-node_*.changes dist/deb/ || true

# Clean up
rm -f ../securefabric-node_*.deb ../securefabric-node_*.buildinfo ../securefabric-node_*.changes

echo -e "${GREEN}✅ Build complete!${NC}"
echo -e "${GREEN}Package location: dist/deb/${NC}"
ls -lh dist/deb/
```

Make it executable:

```bash
chmod +x scripts/build_deb.sh
```

## Makefile Integration

Add to `Makefile`:

```bash
.PHONY: deb deb-clean deb-install

# Build Debian package
deb:
    @bash scripts/build_deb.sh

# Clean Debian build artifacts
deb-clean:
    rm -rf dist/deb/
    rm -rf debian/securefabric-node/
    rm -rf debian/.debhelper/
    rm -rf debian/cargo/
    rm -f debian/files
    rm -f debian/debhelper-build-stamp
    rm -f debian/*.log
    rm -f debian/*.substvars

# Install built package locally
deb-install: deb
    sudo dpkg -i dist/deb/securefabric-node_*.deb
```

## GitHub Actions Workflow

Create `.github/workflows/deb-build.yml` in securefabric-core:

```yaml
name: Debian Package Build

on:
  push:
    tags:
      - 'v*'
  workflow_dispatch:

jobs:
  build-deb:
    name: Build Debian Package
    runs-on: ubuntu-latest

    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Install dependencies
        run: |
          sudo apt-get update
          sudo apt-get install -y \
            build-essential \
            debhelper \
            devscripts \
            dpkg-dev \
            protobuf-compiler \
            libssl-dev \
            pkg-config

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Build Debian package
        run: |
          bash scripts/build_deb.sh

      - name: Upload package as artifact
        uses: actions/upload-artifact@v4
        with:
          name: debian-package
          path: dist/deb/*.deb

      - name: Create Release
        if: startsWith(github.ref, 'refs/tags/v')
        uses: softprops/action-gh-release@v1
        with:
          files: dist/deb/*.deb
          draft: false
          prerelease: ${{ contains(github.ref_name, 'alpha') || contains(github.ref_name, 'beta') || contains(github.ref_name, 'rc') }}
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

## Building the Package

### Prerequisites

```bash
# Install build dependencies
sudo apt-get install -y \
    build-essential \
    debhelper \
    devscripts \
    dpkg-dev \
    cargo \
    rustc \
    protobuf-compiler \
    libssl-dev \
    pkg-config
```

### Build Commands

```bash
# Using the build script
bash scripts/build_deb.sh

# Or using make
make deb

# Or manually
dpkg-buildpackage -us -uc -b
```

The resulting `.deb` file will be in `dist/deb/`.

## Installing the Package

```bash
# Install
sudo dpkg -i dist/deb/securefabric-node_0.1.0-1_amd64.deb

# Or using make
make deb-install

# Install dependencies if needed
sudo apt-get install -f
```

## Using the Package

### Configure the node

```bash
sudo nano /etc/securefabric/.env
```

### Start the service

```bash
sudo systemctl start securefabric-node
```

### Check status

```bash
sudo systemctl status securefabric-node
```

### View logs

```bash
sudo journalctl -u securefabric-node -f
```

### Enable auto-start

```bash
sudo systemctl enable securefabric-node
```

## Setting Up an APT Repository

### Simple File-Based Repository

Create a simple APT repository structure:

```bash
# Create repository structure
mkdir -p apt-repo/dists/stable/main/binary-amd64
mkdir -p apt-repo/pool/main

# Copy .deb files
cp dist/deb/securefabric-node_*.deb apt-repo/pool/main/

# Generate Packages file
cd apt-repo
dpkg-scanpackages --multiversion pool/ > dists/stable/main/binary-amd64/Packages
gzip -k dists/stable/main/binary-amd64/Packages

# Generate Release file
cd dists/stable
cat > Release <<EOF
Origin: SecureFabric
Label: SecureFabric
Suite: stable
Codename: stable
Architectures: amd64 arm64
Components: main
Description: SecureFabric Node Repository
EOF

# Generate Release signature (optional, requires GPG key)
gpg --default-key contact@secure-fabric.io -abs -o Release.gpg Release
gpg --default-key contact@secure-fabric.io --clearsign -o InRelease Release
```

### Hosting the Repository

#### Option 1: GitHub Releases

```bash
# Users add the repository
echo "deb [trusted=yes] https://github.com/NodeCube/securefabric-releases/releases/download/apt stable main" | \
    sudo tee /etc/apt/sources.list.d/securefabric.list

sudo apt-get update
sudo apt-get install securefabric-node
```

#### Option 2: Static HTTP Server

Host the `apt-repo/` directory on a web server:

```bash
# Users add the repository
echo "deb [trusted=yes] https://apt.secure-fabric.io stable main" | \
    sudo tee /etc/apt/sources.list.d/securefabric.list

sudo apt-get update
sudo apt-get install securefabric-node
```

#### Option 3: Using aptly

```bash
# Install aptly
sudo apt-get install aptly

# Create repository
aptly repo create -distribution=stable -component=main securefabric

# Add packages
aptly repo add securefabric dist/deb/securefabric-node_*.deb

# Publish
aptly publish repo -architectures="amd64,arm64" securefabric
```

## Uninstalling

```bash
# Remove package but keep config
sudo apt-get remove securefabric-node

# Remove package and config (purge)
sudo apt-get purge securefabric-node
```

## Troubleshooting

### Build fails with missing dependencies

```bash
sudo apt-get install -f
```

### Service won't start

```bash
# Check logs
sudo journalctl -u securefabric-node -n 50

# Check configuration
sudo cat /etc/securefabric/.env

# Verify permissions
ls -la /var/lib/securefabric
ls -la /etc/securefabric
```

### Permission denied errors

```bash
# Fix data directory permissions
sudo chown -R securefabric:securefabric /var/lib/securefabric
sudo chmod 0750 /var/lib/securefabric
```

## References

- [Debian New Maintainers' Guide](https://www.debian.org/doc/manuals/maint-guide/)
- [Debian Policy Manual](https://www.debian.org/doc/debian-policy/)
- [debhelper(7)](https://manpages.debian.org/testing/debhelper/debhelper.7.en.html)
- [systemd.service(5)](https://www.freedesktop.org/software/systemd/man/systemd.service.html)
