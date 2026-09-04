# Architecture

Codex Companion is split into a React view layer and a Tauri/Rust boundary. The frontend never directly reads the Codex filesystem. It invokes narrow Rust commands which return small, serializable view models.

```text
React features -> services/tauri.ts -> Tauri commands -> codex/session_storage/platform/config modules -> local filesystem
```

## Frontend

- `src/app`: application shell and global, dependency-free styles.
- `src/features`: independently owned pages for dashboard, conversations, backup, skills, pets, diagnostics, and settings.
- `src/services`: typed bridge to Tauri commands.
- `src/types`: stable DTOs shared by UI-facing service calls.

## Native layer

- `platform.rs`: the only place that resolves OS-dependent paths. `CODEX_HOME` overrides the default `~/.codex` location.
- `codex.rs`: read-only diagnostics and CLI version probes.
- `session_storage.rs`: version-conscious, read-only legacy rollout-session adapter. It recursively locates `.jsonl` files below the resolved `sessions/` root, reads only each file's first JSON record, requires `type: session_meta`, and normalizes its ID, timestamps, cwd/project, and source. Optional labels and update timestamps come from `session_index.jsonl`. It never reads message records.
- `backup.rs`: export, inspection, and restore boundary. Restore accepts an opaque token from a successful inspection, revalidates the ZIP immediately before preview and copying, allows only explicitly selected manifest sessions, verifies JSONL `session_meta` IDs, re-snapshots conflicts at execution, emits structured `{ code, message }` failures, and writes under canonical `CODEX_HOME/sessions` through create-new semantics with rollback. Safety backups are independently versioned ZIPs with their own manifest.
- `config.rs`: small JSON settings file, with safe defaults.
- `commands.rs`: the allowlisted interface available to the frontend.
- `logging.rs`: warning-level output by default; no periodic writer or background worker.

## Portable mode

Portable mode requires a `portable-mode` marker file adjacent to the executable. When enabled in settings and the marker is present, Companion stores its own config and backups alongside the executable in `config/` and `backups/`. It does not move or rewrite Codex data.

## Safety boundary

Restore never overwrites: existing destination files are reported as conflicts and skipped. Rust validates destination roots, relative paths and parent symlinks, then rolls back files created in a failed operation. React receives DTOs and never reads ZIPs or writes the filesystem. `src/i18n` provides typed, dependency-free UI dictionaries; settings writes update the in-memory language value immediately after the native save completes.
