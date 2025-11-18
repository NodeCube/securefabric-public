#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0

"""
CI check script to ensure packaging metadata files exist and are valid.

This script verifies that all required packaging metadata files exist
and meet minimum quality standards before allowing builds to proceed.
"""

import sys
from pathlib import Path


def check_file_exists(file_path: Path, min_lines: int = 0) -> bool:
    """Check if a file exists and optionally has minimum line count."""
    if not file_path.exists():
        print(f"❌ MISSING: {file_path}")
        return False

    if min_lines > 0:
        with open(file_path, 'r', encoding='utf-8') as f:
            lines = [l for l in f.readlines() if l.strip()]
            if len(lines) < min_lines:
                print(f"❌ TOO SHORT: {file_path} has {len(lines)} non-empty lines, expected at least {min_lines}")
                return False

    print(f"✅ OK: {file_path}")
    return True


def check_license_content(file_path: Path, expected_license: str) -> bool:
    """Check if LICENSE file contains expected license identifier."""
    if not file_path.exists():
        print(f"❌ MISSING: {file_path}")
        return False

    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()
        if expected_license in content:
            print(f"✅ OK: {file_path} contains {expected_license}")
            return True
        else:
            print(f"❌ WRONG LICENSE: {file_path} does not contain '{expected_license}'")
            return False


def main():
    """Run all packaging metadata checks."""
    repo_root = Path(__file__).parent.parent
    errors = []

    print("=" * 60)
    print("Checking packaging metadata...")
    print("=" * 60)

    # Root repository checks
    print("\n📦 Root repository:")
    if not check_file_exists(repo_root / "LICENSE"):
        errors.append("Root LICENSE missing")
    if not check_license_content(repo_root / "LICENSE", "Apache License"):
        errors.append("Root LICENSE is not Apache-2.0")
    if not check_file_exists(repo_root / "README.md", min_lines=10):
        errors.append("Root README.md missing or too short")

    # Python SDK checks
    print("\n🐍 Python SDK (sdk/python):")
    python_dir = repo_root / "sdk" / "python"

    if not check_file_exists(python_dir / "pyproject.toml"):
        errors.append("Python pyproject.toml missing")
    if not check_file_exists(python_dir / "README.md", min_lines=10):
        errors.append("Python README.md missing or too short")
    if not check_file_exists(python_dir / "LICENSE"):
        errors.append("Python LICENSE missing")
    else:
        if not check_license_content(python_dir / "LICENSE", "Apache License"):
            errors.append("Python LICENSE is not Apache-2.0")

    # Check Python package structure
    if not check_file_exists(python_dir / "securefabric" / "__init__.py"):
        errors.append("Python __init__.py missing")
    if not check_file_exists(python_dir / "securefabric" / "client.py"):
        errors.append("Python client.py missing")

    # Specs checks
    print("\n📋 Specifications:")
    if not check_file_exists(repo_root / "specs" / "securefabric.proto"):
        errors.append("securefabric.proto missing")

    # Print summary
    print("\n" + "=" * 60)
    if errors:
        print(f"❌ FAILED: {len(errors)} error(s) found:")
        for error in errors:
            print(f"  - {error}")
        print("=" * 60)
        return 1
    else:
        print("✅ SUCCESS: All packaging metadata checks passed!")
        print("=" * 60)
        return 0


if __name__ == "__main__":
    sys.exit(main())
