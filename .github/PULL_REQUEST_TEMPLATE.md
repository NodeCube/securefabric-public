<!-- SPDX-License-Identifier: Apache-2.0 -->

## Description

<!-- Provide a clear description of the changes -->

Fixes #<!-- issue number -->

## Type of Change

<!-- Mark the relevant option with an 'x' -->

- [ ] Bug fix (non-breaking change fixing an issue)
- [ ] New feature (non-breaking change adding functionality)
- [ ] Breaking change (fix or feature causing existing functionality to change)
- [ ] Documentation update
- [ ] Protocol specification change
- [ ] Refactoring (no functional changes)
- [ ] CI/tooling improvement

## Changes Made

<!-- List the specific changes -->

-
-
-

## Testing

<!-- Describe the tests you ran -->

- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] Manual testing performed
- [ ] All existing tests pass

**Test results:**

```
<!-- Paste test output here -->
```

## Documentation

- [ ] Documentation updated (if needed)
- [ ] Examples updated (if API changed)
- [ ] CHANGELOG.md updated (for user-facing changes)
- [ ] API documentation generated/updated

## Protocol Changes

<!-- Only if modifying /specs/ -->

- [ ] Protocol version bumped (if breaking)
- [ ] Backward compatibility analyzed
- [ ] All SDKs updated to match
- [ ] Migration guide provided (if breaking)

## Checklist

<!-- All must be checked before merge -->

- [ ] Code follows project style guidelines
- [ ] Self-review completed
- [ ] Comments added for complex logic
- [ ] No new warnings introduced
- [ ] Tests added/updated and passing
- [ ] Documentation updated (if needed)
- [ ] No server/node code added (SDK repo only!)
- [ ] SPDX headers present in new files
- [ ] Commit messages follow Conventional Commits
- [ ] Ready for review

## SDK-Specific Checks

### Rust SDK

- [ ] `cargo fmt --check` passes
- [ ] `cargo clippy` passes (no warnings)
- [ ] `cargo test` passes
- [ ] Cargo.toml version updated (if needed)

### JavaScript/TypeScript SDK

- [ ] `npm run lint` passes
- [ ] `npm run build` succeeds
- [ ] `npm test` passes
- [ ] Type definitions updated

### Python SDK

- [ ] `black --check .` passes
- [ ] `mypy` type checking passes
- [ ] `pytest` passes
- [ ] Version updated in pyproject.toml (if needed)

## Breaking Changes

<!-- If this is a breaking change, describe: -->
<!-- 1. What breaks and why -->
<!-- 2. Migration path for users -->
<!-- 3. Version bump strategy -->

## Additional Notes

<!-- Any additional context, concerns, or follow-up items -->

## Screenshots/Logs

<!-- If applicable, add screenshots or relevant log output -->
