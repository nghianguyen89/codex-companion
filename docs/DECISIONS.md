# Architecture decisions

## 2026-09-04 — Use Tauri 2 instead of Electron

### Decision

Use Tauri 2 with React, TypeScript, Vite, and Rust.

### Reason

- lower idle memory use and distribution size
- native filesystem access can stay in Rust
- one main codebase can support Windows, macOS, and Linux

### Consequences

- native build toolchains are required
- platform-specific installers must be produced on matching CI runners

## 2026-09-04 — Discover legacy rollout sessions from local metadata files

### Decision

For Milestone 2, use a Rust adapter to read only the first `session_meta` record from legacy `sessions/**/*.jsonl` rollout files and optional labels from `session_index.jsonl`.

### Reason

- Codex CLI 0.153.0 exposes mutating session commands but no documented, structured read-only list command suitable for the UI
- the discovered files contain the needed IDs, timestamps, cwd, source, and optional Git metadata without reading conversation content
- isolating the format in one adapter makes later compatibility changes contained

### Consequences

- the adapter must tolerate malformed files and changed formats per file
- newer non-rollout storage formats are reported as unsupported rather than guessed
- no session content is cached or exposed to the frontend

## 2026-09-04 — Backup only explicitly selected, re-resolved session files

### Decision

Milestone 3 exports ZIP archives from a Rust command that accepts only discovered session IDs. It re-runs the supported discovery adapter, canonicalizes each resulting file beneath the Codex `sessions/` root, and writes `manifest.json` plus only those exact files to a create-new archive in Companion's backup directory.

### Reason

Frontend paths and arbitrary directory copies would weaken the safety boundary. Re-resolution prevents a stale or forged UI path from selecting another file, while create-new semantics prevent accidental archive replacement.

### Consequences

- React receives only preview and result DTOs; it never has filesystem paths to supply back to Rust.
- The manifest's format version is independent of the app release and records the source platform and CLI version when available.
- Backup is an export operation only; restore, deletion, migration, cleanup, and session state changes remain out of scope.

## 2026-09-04 — Inspect backup archives in Rust before any restore design

### Decision

Milestone 4 phase 1 opens a user-selected ZIP only for read-only inspection. Rust validates every ZIP path before using it, then strictly deserializes `manifest.json`, supports only format version 1, and cross-checks each listed session path and uncompressed byte count against the archive. It returns a compact inspection DTO with metadata and validation messages; it never extracts the ZIP.

### Reason

An archive is untrusted input. Performing validation at the native boundary prevents browser-side ZIP parsing, path traversal risks, and any future temptation to copy unvalidated entries into `CODEX_HOME`.

### Consequences

- The native file dialog is the only archive selection UI; archive content and selected-path details are not rendered to React.
- Invalid archives remain inspectable as a validation result, but cannot drive restore because restore does not exist yet.
- The inspection reads only `manifest.json` (capped at 1 MiB) and ZIP metadata for session entries.

## 2026-09-04 — Restore only validated, explicitly selected sessions

Restore uses an opaque inspection token and revalidates on every preview/execution. Destination copies are create-new, traversal and symlink escapes are rejected, and partial creates are rolled back. Conflicts are skipped; replace is intentionally absent. The execution-time conflict snapshot, not the UI preview, drives safety backup creation. A safety archive is a separately named, manifest-bearing ZIP of conflicting regular files only; with no conflicts, it is intentionally omitted.

## 2026-09-04 — Dependency-free typed English/Vietnamese UI

Language preference is stored as `language` in `AppConfiguration`, defaults to English, and selects typed dictionaries in `src/i18n`. The saved DTO replaces in-memory configuration immediately after save, so the provider rerenders without a reload. Native data and errors are not interpreted for translation.

## 2026-09-04 — Bounded, privacy-minimized restore audit history

Restore history is a Companion-owned versioned JSON document, retained locally as the newest 100 summaries. It records only time, archive filename, selected session IDs, restored/skipped counts, optional safety-backup path, outcome, and stable failure code. Session content, source archive paths, tokens, credentials, and native error strings are deliberately excluded. If history storage is invalid or from a future version, Companion does not interpret it; if recording fails, a completed restore still remains completed.

## 2026-09-04 — Explicit legacy storage compatibility over format inference

The supported Codex metadata contract is limited to the observed legacy first-line JSONL `session_meta` record with `payload.session_id` or `payload.id`. Fixtures lock both accepted variants and invalid/future variants. Unknown record types, malformed IDs, and future backup format versions are rejected explicitly and cannot reach restore; backup format v1 remains the sole supported archive format until an explicit migration is designed and tested.
# Milestone 6 decisions

- Deletion targets are IDs, never client-provided filesystem paths.
- Quarantine uses ZIP format version 1 with a separate `delete-manifest.json`; a future format requires an explicit compatibility contract.
- Quarantine retention is manual and indefinite: Companion does not silently purge recovery artifacts.
- Audit history stores safe IDs, counts, outcome, stable error code, and quarantine path—not session content, source paths, secrets, or native errors.

## 2026-09-07 — Windows environment archive and conservative recovery

User scope expands beyond local sessions. v2 stores selected environment components with SHA-256, offline Windows deny-write/delete handles and source inventory checks. Keep installed plugin resources; exclude auth and potential credential-bearing settings intact. Preserve v1/delete archive readers. No automatic SQLite/JSON/TOML merge: archive manual components and expose the limitation. Target usability remains a separate verification milestone.

Cleanup is limited to the observed, upstream-defined remote catalog cache. Unknown paths/schema, active Codex, changed files and reparse points cannot authorize deletion. No cleanup retention scheduler or quarantine purge is added.
