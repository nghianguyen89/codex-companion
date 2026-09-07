# Validation — 2026-09-07

## Automated checks

All commands ran successfully in `D:\projects\my-github\tools\codex-companion` after the final source changes:

| Check | Result |
|---|---|
| `node node_modules/typescript/bin/tsc -b` | Pass |
| `node node_modules/eslint/bin/eslint.js .` | Pass |
| `node node_modules/vitest/vitest.mjs run` | 13 tests pass |
| `cargo test --manifest-path src-tauri/Cargo.toml --lib --offline` | 39 tests pass |
| `./build-publish.ps1` | Pass: Windows x64 NSIS bundle and portable executable |

Rust fixtures cover legacy metadata string/object sources, ZIP traversal, archive corruption, v1 session archive compatibility, delete safety archive recovery, conflict/no-overwrite restore, rollback and rollback failure, empty destination, source locks, Windows junction rejection, cleanup allowlist/change detection, and synthetic environment archive round trips.

Frontend fixtures cover typed translations, preview/selection equality and gated environment/cleanup actions. A browser UI fixture exercised the Vietnamese environment preview, excluded credential warning, restore statuses (`new`, `manual`, `conflict`), and cleanup scan/selection/confirmation gate. It deliberately did not execute a destructive restore or cleanup operation.

## Windows deliverables

The final build command must be re-run after a source change:

```powershell
./build-publish.ps1
```

It produces:

- `release/portable/codex-companion.exe` and `portable-mode`.
- `src-tauri/target/release/bundle/nsis/Codex Companion_0.2.0_x64-setup.exe`.

Final portable executable SHA-256:

```text
63E47854E5424D7C54BF617E78C2671B392E296B37A30950A952955BE3606131
```

## Not validated

No target-machine copy or Codex Desktop reindex/reopen test was possible. The application therefore does not claim full desktop-chat migration. See [STORAGE_AUDIT.md](STORAGE_AUDIT.md) and [USER_GUIDE.md](USER_GUIDE.md).

## Phase 0–1 personal bundle delta — 2026-09-07

After the Beyond Compare source changes, `pnpm lint`, `pnpm check`, `pnpm test`
(14 tests), and `pnpm build` passed.
`cargo check --manifest-path src-tauri/Cargo.toml` passed and
`cargo test --manifest-path src-tauri/Cargo.toml --lib` passed 46 tests,
including the existing session-v1, delete-v1, and environment-v2 readers plus
new personal-bundle acknowledgement, malicious-inventory, source-revalidation,
create-new, and rollback checks. `git diff --check` passed.

`pnpm tauri build --bundles nsis` passed and produced the Windows x64 NSIS
installer. The default `pnpm tauri build` also built the release executable but
the MSI step failed because local Windows Installer access prevented WiX ICE
validation; no MSI artifact is claimed for this delta.

## Phase 2 SourceTree delta — 2026-09-07

Completed checks: `pnpm lint`, `pnpm check`, `pnpm test` (15 tests), `pnpm build`,
`cargo check --manifest-path src-tauri/Cargo.toml`, and
`cargo test --manifest-path src-tauri/Cargo.toml --lib` (52 tests) all passed.
The Rust suite includes the session-v1, delete-v1, environment-v2 and Beyond
Compare regressions plus SourceTree valid/malformed/unknown XML, URL userinfo,
traversal/ADS/reserved paths, duplicate inventory, hash/size rejection,
process-running recognition, source-change, create-new conflict and rollback
coverage. The frontend fixture verifies the SourceTree exclusions, sensitive
transport warning and manual-only copy. `git diff --check` passed. The fresh
Windows x64 NSIS bundle passed at
`src-tauri/target/release/bundle/nsis/Codex Companion_0.2.0_x64-setup.exe`
(SHA-256 `1C2A1D96340AEDFA2262197544B0F197EC33D8B77F91487C755483FD16ACD445`).
MSI was not run for this delta.
