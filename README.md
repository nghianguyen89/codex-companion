# Codex Companion

Codex Companion is a lightweight, cross-platform desktop control panel for a local OpenAI Codex environment. It is an independent community utility, not an official OpenAI product.

It does not replace Codex Desktop, alter its binaries, embed it, or use undocumented network APIs. Codex data changes are explicit: create-only recovery, selected local session deletion with safety archives, and allowlisted cache cleanup.

## Version 0.2.0

Windows environment archives now cover selected chat files/metadata, settings, skills/plugin resources and pets. SHA-256 verification, offline source locks and no-overwrite restore protect the file workflow. **Desktop chat/database merging remains unfinished and unverified.** Database/index/settings are archived for manual migration; copying files does not establish a usable migrated Desktop environment.

Temporary cleanup is on-demand and limited to the verified remote plugin catalog cache. Installed plugin resources are protected. Session v1 and existing delete safety archives remain readable.

See [User guide](docs/USER_GUIDE.md), [storage inventory and limitations](docs/STORAGE_AUDIT.md), [build instructions](BUILD.md), and [validation](docs/VALIDATION.md).

## Historical milestone 5

Milestone 5 adds safe restore history and explicit storage compatibility boundaries. A read-only local audit records the time, archive filename, selected session IDs, restored/skipped counts, safety-backup path, outcome, and stable error code for each restore attempt. It never stores session content, archive source paths, inspection tokens, credentials, or native error text. History retains the newest 100 entries. Restore remains limited to explicitly selected sessions from a successfully inspected Companion archive: Rust revalidates archives, limits destinations to `CODEX_HOME/sessions`, rejects unsafe paths and symlink escapes, uses create-new writes, skips conflicts without overwrite, and rolls back files created by a failed restore.

## Supported platforms

The source is designed for Windows, macOS, and Linux. Native bundles must be produced on their matching operating systems through CI.

## Development

Prerequisites: Node.js, pnpm, Rust stable, and the platform prerequisites required by Tauri 2.

```sh
pnpm install
pnpm dev
pnpm lint
pnpm check
pnpm test
pnpm build
pnpm tauri dev
```

See [Development notes](docs/DEVELOPMENT.md), [architecture](docs/ARCHITECTURE.md), and the [backup format](docs/BACKUP_FORMAT.md).

## Roadmap

1. Application shell, diagnostics, configuration, and platform abstraction.
2. Read-only session discovery behind a versioned storage adapter.
3. Selected-session, versioned backup archive creation.
4. Read-only inspection and safe restore with safety backups.
5. Restore history and storage compatibility fixtures (complete).
6. Skills and pets metadata management.
# Milestone 6: Safe local conversation deletion

Companion can delete only supported legacy Codex session files discovered under `CODEX_HOME/sessions`; it never calls a cloud API and never deletes Codex Desktop or cloud chats. The UI requires a metadata-only preview and typing `DELETE`. Rust re-resolves IDs, rejects symlinks/path traversal, writes and validates a create-new versioned ZIP safety archive before removal, and restores with create-new semantics if removal fails. Safety archives are retained in Companion's local quarantine until manually removed.
