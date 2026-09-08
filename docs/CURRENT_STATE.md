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

### Phase 3 addition

- Windows-only XAMPP personal bundle: explicitly selected direct `htdocs`
  projects plus four reviewed text configuration files, SHA-256 inventory,
  strict size/count/namespace validation, stopped-process gate, and
  create-new rollback-safe staging recovery.
- XAMPP binaries, MariaDB data/logical dumps, credentials, keys, logs, caches,
  repository metadata, reparse points, overwrite, automatic placement and
  MariaDB import remain excluded/manual-only.

### Dev Companion branding and Beyond Compare credential selection

- Visible product, installer and portable-executable branding is Dev Companion;
  the legacy `codex-companion` application-data directory and established
  archive formats remain unchanged for compatibility.
- Beyond Compare accepts an explicit user declaration that its opaque `.bcpkg`
  includes saved passwords or FTP/SSH credentials, records that declaration,
  and remains manual-import/staging-only. The ZIP is not password-protected.

## In Progress

- TBD

## Known Issues

- TBD

- A `.bcpkg` cannot prove whether its native export includes saved passwords or
  FTP/SSH credentials; the user's recorded choice is not verification and every
  personal bundle remains sensitive. No authenticated encryption, cloud
  transport, license migration or automatic Beyond Compare import is implemented.

## Technical Debt Worth Remembering

- TBD

## Validation Status

```text
Lint: pnpm lint (pass)
Type-check: pnpm check (pass)
Tests: pnpm test (16 pass); cargo test --lib (57 pass)
Build: pnpm build (pass); Windows x64 NSIS bundle and portable executable (pass); MSI not run
```

Never mark validation as passing unless it was actually run.

## Important Recent Decisions

- TBD

## Next Recommended Work

- TBD
