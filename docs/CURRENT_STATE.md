# Current State

Concise repository snapshot. Update at meaningful milestones.

## Baseline

```text
Branch:
Baseline commit:
Release/version:
```

## Working Features

- TBD

### Phase 0–1 addition

- Windows-only Beyond Compare package bundle/recovery: one opaque user-selected
  `.bcpkg`, strict `personal-bundle-v1` inspection, SHA-256, create-new staging
  recovery, rollback, and manual native import.
- Existing Codex session-v1, delete-v1 and environment-v2 workflows remain
  separate and unchanged.

### Phase 2 addition

- Windows-only SourceTree `bookmarks.xml` bundle/recovery: strict fixture-only
  XML parsing, closed-process gate, source/version/hash revalidation,
  namespaced `personal-bundle-v1` ZIP inventory, create-new staging and manual
  destination placement only.
- SourceTree repositories, tabs, custom actions, `user.config`, hosted
  accounts, credentials, licenses and secrets remain excluded.

## In Progress

- TBD

## Known Issues

- TBD

- A `.bcpkg` cannot prove that its export omitted secrets; user acknowledgement
  is required and every personal bundle remains sensitive. No authenticated
  encryption, cloud transport, license migration or automatic Beyond Compare
  import is implemented.

## Technical Debt Worth Remembering

- TBD

## Validation Status

```text
Lint: pnpm lint (pass)
Type-check: pnpm check (pass)
Tests: pnpm test (15 pass); cargo test --lib (52 pass)
Build: pnpm build (pass); Windows x64 NSIS bundle (pass); MSI not run
```

Never mark validation as passing unless it was actually run.

## Important Recent Decisions

- TBD

## Next Recommended Work

- TBD
