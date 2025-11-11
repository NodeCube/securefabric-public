# Sanitizing Git History

If security scans detect secrets or sensitive data in the Git history, you'll need
to rewrite the repository history. This is a destructive operation that requires
force-pushing and coordinating with all contributors.

## Prerequisites

1. Install `git-filter-repo`:

```bash
# Using pip
pip install git-filter-repo

# On macOS with Homebrew
brew install git-filter-repo

# On Ubuntu/Debian
apt-get install git-filter-repo
```

2. **IMPORTANT**: Notify all contributors before rewriting history
3. Ensure you have a complete backup of the repository

## Detection

First, scan for secrets using automated tools:

```bash
# Using gitleaks
gitleaks detect --source . --verbose

# Using trufflehog
trufflehog git file://. --only-verified

# Manual search for common patterns
git log -p | grep -E "(password|secret|token|api[_-]?key|bearer)"
```

## Common Patterns to Remove

Create a `patterns.txt` file with sensitive patterns:

```
# API Keys
AKIA[0-9A-Z]{16}
[0-9a-zA-Z]{32,}

# Private keys
-----BEGIN (RSA|EC|PRIVATE) KEY-----

# Tokens
(ghp|github_pat)_[0-9a-zA-Z]{36,}

# Internal domains
@nodecube\.(io|hr)
10\.\d+\.\d+\.\d+
192\.168\.\d+\.\d+

# Specific secrets (add your own)
SF_BEARER=.+
SF_TOKEN=.+
```

## Sanitization Options

### Option 1: Remove Specific Files

If secrets are in specific files that should never have been committed:

```bash
git filter-repo --invert-paths \
  --path secrets/config.yaml \
  --path .env.production \
  --path certs/private.key \
  --force
```

### Option 2: Remove Content Matching Patterns

To remove specific secret values from all files:

```bash
# Create expressions file
cat > expressions.txt <<EOF
regex:AKIA[0-9A-Z]{16}==>${REDACTED_AWS_KEY}
regex:ghp_[0-9a-zA-Z]{36}==>${REDACTED_GITHUB_TOKEN}
regex:@nodecube\.(io|hr)==>@secure-fabric.io
regex:(https?://)?api\.securefabric\.io==>YOUR_ENDPOINT_HERE
EOF

git filter-repo --replace-text expressions.txt --force
```

### Option 3: Remove Entire Directories

If deployment/infrastructure directories were accidentally committed:

```bash
git filter-repo --invert-paths \
  --path-glob 'infra/**' \
  --path-glob 'deploy/**' \
  --path-glob 'ansible/**' \
  --path-glob 'terraform/**' \
  --path-glob 'k8s/**' \
  --path-glob '**/*.pem' \
  --path-glob '**/*.key' \
  --path-glob '**/*.crt' \
  --force
```

## Complete Sanitization Workflow

```bash
# 1. Create a fresh clone for safety
git clone --mirror git@github.com:NodeCube/securefabric-public.git securefabric-clean
cd securefabric-clean

# 2. Run filter-repo with all patterns
git filter-repo \
  --replace-text expressions.txt \
  --invert-paths --path-glob '**/*.pem' \
  --invert-paths --path-glob '**/*.key' \
  --invert-paths --path-glob '**/*.crt' \
  --invert-paths --path-glob 'infra/**' \
  --invert-paths --path-glob 'deploy/**' \
  --force

# 3. Verify the sanitization
git log -p | grep -E "(password|secret|token)" || echo "Clean!"

# 4. Force push to remote (DESTRUCTIVE!)
# WARNING: This will rewrite history for all collaborators
git push origin --force --all
git push origin --force --tags
```

## Post-Sanitization Steps

1. **Notify all contributors**:

```
IMPORTANT: The securefabric-public repository history has been rewritten
to remove sensitive data. Please follow these steps:

1. Delete your local clone: rm -rf securefabric-public
2. Re-clone the repository: git clone git@github.com:NodeCube/securefabric-public.git
3. Any open PRs will need to be recreated

If you have local branches with changes, please contact the maintainers
for guidance on rebasing your work.
```

2. **Rotate all exposed secrets**:
   - Regenerate API keys
   - Issue new bearer tokens
   - Replace TLS certificates
   - Update any hardcoded credentials in production

3. **Invalidate old secrets**:
   - Revoke exposed API keys
   - Disable old tokens
   - Remove old SSH keys

4. **Update protected branch rules**:
   - Re-apply branch protection after force push
   - Update CI/CD webhooks if needed

5. **Scan again to verify**:

```bash
gitleaks detect --source . --verbose
trufflehog git file://. --only-verified
```

## Prevention

After sanitization, ensure this never happens again:

1. Add pre-commit hooks:

```bash
# Install pre-commit
pip install pre-commit

# Add .pre-commit-config.yaml
cat > .pre-commit-config.yaml <<EOF
repos:
  - repo: https://github.com/gitleaks/gitleaks
    rev: v8.18.0
    hooks:
      - id: gitleaks
EOF

pre-commit install
```

2. Enable GitHub secret scanning:
   - Go to Settings → Code security and analysis
   - Enable secret scanning
   - Enable push protection

3. Use `.env.example` for templates:

```bash
# .env.example
SF_ENDPOINT=YOUR_ENDPOINT_HERE
SF_TOKEN=YOUR_TOKEN_HERE
```

4. Add comprehensive `.gitignore` (already done in this PR)

## Emergency Contacts

If you discover exposed secrets in the wild:

- Security team: [security@secure-fabric.io](mailto:security@secure-fabric.io)
- GitHub Security Advisory: https://github.com/NodeCube/securefabric-public/security/advisories/new

## References

- git-filter-repo documentation: https://github.com/newren/git-filter-repo
- GitHub: Removing sensitive data: https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/removing-sensitive-data-from-a-repository
- gitleaks: https://github.com/gitleaks/gitleaks
- trufflehog: https://github.com/trufflesecurity/trufflehog
