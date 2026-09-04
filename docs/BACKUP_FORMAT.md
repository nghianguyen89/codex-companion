# Backup and safety-backup format

Milestone 3 creates selected-session export archives only. Restore, deletion, migration, cleanup, and changes to Codex data are not implemented.

```text
codex-backup-YYYY-MM-DD_HHMMSS[-N].zip
├── manifest.json
├── sessions/
│   └── YYYY/MM/DD/rollout-*.jsonl
└── (no other Codex data)
```

`manifest.json` contains `formatVersion` (currently `1`), an RFC 3339 `createdAt`, source `platform`, `codexCliVersion` when the local read-only version probe succeeds, and `sessions`. Each session entry contains its ID, optional title and timestamps, ZIP-relative `archivePath`, and byte count. The archive contains `manifest.json` plus exactly those selected session files at their relative `sessions/` paths.

The destination is Companion's configured backup directory. A new filename is reserved with create-new semantics and an incrementing suffix if a same-second filename exists; an existing archive is never overwritten.

Before creation, the UI requests a Rust-generated preview showing the selected count, exact aggregate size, format version, and destination directory. The archive creator resolves selected IDs again from the supported discovery adapter rather than accepting a frontend path.

Restore requires a successful inspection token, a second validation immediately before preview/copy, and explicit session selection. Preview lists the canonical `CODEX_HOME/sessions` root, each destination, conflicts, total bytes, create-new operations, and safety-backup status. Only valid JSONL `session_meta` records whose IDs match the manifest can be copied. Existing files are skipped, never replaced; files created before a copy failure are rolled back.

When enabled and the execution-time restore snapshot finds conflicts, Companion creates a separate create-new archive named `codex-safety-backup-YYYY-MM-DD_HHMMSS[-N].zip` before it writes any session. It contains `safety-manifest.json` and only the conflicting regular files under `conflicts/sessions/...`; it never follows a conflict symlink. The manifest has `formatVersion` (currently `1`), RFC 3339 `createdAt`, `kind: "codex-companion-safety-backup"`, `reason: "pre-restore-conflicts"`, and each conflict's ID, archive path, and byte count. If the execution-time snapshot has no conflicts, no safety archive is created. A conflict created after that snapshot is skipped through create-new semantics and is never overwritten.

Restore reads and validates every selected entry fully before filesystem mutation. It uses an execution-time conflict snapshot rather than trusting the preview, rejects unsafe ZIP/path components and existing symlink parents, creates each destination with `create_new`, syncs it, and removes all files created by that operation if a later copy fails. Directory checks are re-run immediately before creation; an OS-level attacker who can replace a checked parent directory between those calls remains outside the guarantees of the standard-library implementation and is treated as a hostile local-machine limitation.

## Milestone 4 phase 1 inspection

Inspection is read-only and does not extract the ZIP. Before interpreting `manifest.json`, Companion rejects ZIP entry names containing absolute paths, drive prefixes, backslashes, empty or dot components, parent traversal, duplicates, or directory entries. The archive must contain one `manifest.json` and exactly the session files listed by manifest `sessions`; additional entries are invalid.

The manifest is strict JSON with no unknown fields. `formatVersion` must equal `1`; `createdAt` and optional session timestamps must be RFC 3339; `platform` and session IDs must not be empty; session `archivePath` values must be unique `sessions/.../*.jsonl` paths; and every listed byte count must equal the ZIP entry's uncompressed size. A missing `codexCliVersion` is reported as a warning, not an error. `manifest.json` is capped at 1 MiB during inspection.
