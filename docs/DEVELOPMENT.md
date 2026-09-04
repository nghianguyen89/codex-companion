# Development notes

## Prerequisites

Install a current Node.js release, pnpm, Rust stable, and the official Tauri 2 prerequisites for your platform. Windows additionally needs the Microsoft C++ build tools and WebView2 runtime.

## Commands

```sh
pnpm install
pnpm lint
pnpm check
pnpm test
pnpm build
cargo test --manifest-path src-tauri/Cargo.toml
cargo check --manifest-path src-tauri/Cargo.toml
```

`pnpm tauri dev` starts Vite and the native application. No watcher is started by production builds or while the application is idle. On a Windows machine with Tauri prerequisites available, additionally run `pnpm tauri build` to validate the native bundle.

## Restore and i18n checks

Rust synthetic fixture tests cover valid restore, skipped conflicts, invalid archives, traversal, and rollback. Frontend tests cover English default, Vietnamese selection, typed interpolation, and missing-key fallback. UI strings belong in `src/i18n/en.ts` and `src/i18n/vi.ts`; use typed keys rather than component-local copy. Do not translate Codex data, manifests, session contents, or native error diagnostics.

## Local discovery investigation

On the initial Windows development machine, `codex --version` reported `codex-cli 0.153.0`. The CLI exposes `doctor`, `archive`, `delete`, and session-related commands, but Companion invokes none of its mutating commands.

The local home contained 167 legacy rollout files at `sessions/YYYY/MM/DD/rollout-*.jsonl`. Every inspected file began with a `session_meta` JSON record. Its payload included `session_id`/`id`, `timestamp`, `cwd`, `source`, `cli_version`, and Git metadata; subsequent JSONL records contain conversation data and are deliberately not read. `session_index.jsonl` uses `id`, `thread_name`, and `updated_at`; Companion uses it only for optional title and update metadata. This is an observed compatibility target, not a promise that every Codex release will keep the format.

## Logging

Set `CODEX_COMPANION_LOG=info` or `debug` only while diagnosing. Do not log conversation content, credentials, or auth tokens.
