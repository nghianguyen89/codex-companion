# Project context

## Purpose and philosophy

Codex Companion is a lightweight, local, cross-platform control panel for an existing OpenAI Codex installation. It complements Codex Desktop rather than replacing it: Companion manages local metadata, diagnostics, backup, and organization while Codex Desktop owns chat, authentication, and cloud features. Safety, correctness, low idle cost, and portability take priority over convenience.

## Stack and platforms

Tauri 2, Rust, React, TypeScript, and Vite support Windows, macOS, and Linux from the same primary source tree. The app uses simple React state and small typed Tauri commands; it has no Redux, database, polling loop, or background worker.

## Architecture and integration

React calls typed commands through `src/services/tauri.ts`; Rust owns path resolution and Codex storage access. `CODEX_HOME` overrides the fallback `~/.codex`. On the investigated Windows installation, Codex CLI 0.153.0 stores legacy rollout sessions as `.jsonl` files below `sessions/YYYY/MM/DD/`; the first record is `session_meta` and contains IDs, timestamp, cwd, source, CLI version, and optional Git metadata. `session_index.jsonl` supplies an ID-to-thread-name mapping and update timestamp.

The Milestone 2 adapter reads only those metadata records. It never reads message records, writes Codex files, stores conversation content, or invokes mutating Codex CLI commands. A malformed or unsupported individual file is skipped and reported as a count.

## Safety, performance, and portable mode

All filesystem paths are centralized in `src-tauri/src/platform.rs`. Future writes must validate a known root, reject traversal, be explicitly confirmed, avoid silent overwrites, and provide a safety backup where applicable. Discovery runs on explicit UI refresh only; it performs no polling or idle disk activity. A `portable-mode` marker next to the executable makes Companion's own configuration and backups use adjacent `config/` and `backups/` folders; it never moves Codex data.

## Milestones and status

Milestone 4 phase 2 adds an explicit restore flow. An opaque token is issued only after successful inspection; preview and execution revalidate the archive. Users select archive sessions, see target paths, conflicts and byte totals, and restore uses create-new copies only. Execution re-snapshots conflicts and, when enabled, writes a separately versioned safety backup before copying. English and Vietnamese UI preferences are persisted in configuration and take effect as soon as save succeeds; Codex/session data and native error text are not translated.

## Important locations

- `src-tauri/src/platform.rs`: platform path resolution
- `src-tauri/src/codex.rs`: diagnostics and Codex-facing read-only logic
- `src-tauri/src/session_storage.rs`: session discovery adapter
- `src-tauri/src/backup.rs`: selected-session ZIP preview and creation
- `src-tauri/src/commands.rs`: Tauri command boundary
- `src/features/conversations/ConversationsPage.tsx`: local list, filtering, sorting, and refresh UI
- `docs/ARCHITECTURE.md`: system design

## Next recommended milestone

The next phase should add a controlled restore-history view and compatibility fixtures for new Codex session-storage variants. It must remain opt-in, preserve the no-overwrite rule, and must not copy arbitrary archive entries.
