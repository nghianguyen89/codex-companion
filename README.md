# Codex Companion

Codex Companion is a lightweight, cross-platform desktop control panel for a local OpenAI Codex environment. It is an independent community utility, not an official OpenAI product.

It does not replace Codex Desktop, alter its binaries, embed it, or use undocumented network APIs. The first milestone is intentionally read-only for Codex data.

## Status

Milestone 4, phase 2 adds safe restore for explicitly selected sessions from a successfully inspected Companion archive. Rust revalidates archives, limits destinations to `CODEX_HOME/sessions`, rejects unsafe paths and symlink escapes, uses create-new writes, skips conflicts without overwrite, and rolls back files created by a failed restore. Conflicts can be captured first in a separately versioned safety-backup ZIP; no safety backup is made when there are no conflicts. The UI supports typed English and Tiếng Việt strings, persisted locally and applied immediately after settings save. Delete, migration, cleanup, and archive/unarchive session features remain out of scope.

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
4. Read-only backup archive inspection (phase 1 complete), followed by a separately designed restore workflow with safety backups.
5. Skills and pets metadata management.
