# Storage audit — 2026-09-07

Read-only observations on the development Windows machine. No production restore/delete/cleanup was performed.

## Evidence

- Enumerated CODEX_HOME top-level names, session first-record `source` variants, pets/plugin directory names, and catalog schema keys; did not print chat messages or credential contents.
- Opened `state_5.sqlite` with Python SQLite URI `mode=ro`; queried `sqlite_master` for `threads` and `_sqlx_migrations`, and migration version numbers only. 52 migrations were present.
- `threads` includes `id`, `rollout_path`, `cwd`, title, archive flags, `history_mode`, project references, pin/section metadata. This proves a sessions-only export does not cover all local indexing/state.
- Observed databases: `state_5.sqlite`, `thread_history_1.sqlite`, `goals_1.sqlite`, `queue_1.sqlite`, `memories_1.sqlite`, their WAL/SHM companions, and files below `sqlite/`. `logs_2.sqlite` is separated as diagnostic storage. Database restoration/merging is not implemented.
- Observed metadata sources: string `vscode`, object `subagent.other = guardian`, and `subagent.thread_spawn` with parent ID, depth, agent path/nickname/role. Fixtures use synthetic values and reject unknown source variants.
- Observed remote catalog: 16-hex `.json` name, about 17 MB, schema keys `schema_version`, `fetched_at`, `plugins`. Cleanup supports only this verified category.

Official configuration context: [Advanced configuration](https://learn.chatgpt.com/docs/config-file/config-advanced). This documentation does not establish a safe Desktop database merge contract.
Catalog evidence: [OpenAI Codex catalog cache implementation](https://github.com/openai/codex/blob/main/codex-rs/core-plugins/src/remote/catalog_cache.rs) defines `cache/remote_plugin_catalog`, version 1, cache TTL and missing-cache handling. Keep source/version drift in mind; unknown schema is never cleaned.

## v2 inventory

| Group | Included | Restore policy |
|---|---|---|
| Chat | `sessions/`, `archived_sessions/`, `attachments/` | Create new files only |
| Chat state | `session_index.jsonl`, `.codex-global-state.json`, the five named DB families above including WAL/SHM, `sqlite/` | Archive only; manual migration |
| Settings | `config.toml`, `AGENTS.md`, `hooks.json`, `rules/`, `prompts/` | Archive only; manual configuration; files with credential indicators excluded intact |
| Skills | `skills/`, `plugins/cache/` including supporting resources | Create new files; conflict skip; plugin registry/reconnection separate |
| Pets | `pets/` and its assets | Create new files; conflict skip |

Excluded: auth/credential store, `auth.json`, `cap_sid`, installation identity, logs/diagnostic DB, regenerable cache, tmp/crash/incomplete downloads outside the allowlist, old backup/quarantine names, `.git`, and everything outside supported groups. Preview accounts for excluded regular files by reason; no source file is removed. Symlinks/junctions abort inventory rather than quietly following them.

`plugins/cache` is intentionally retained with skills. It is installed resource storage, not treated as disposable merely because of its name. Plugin runtime/staging/registry outside this tree is excluded; reinstall/reconnect on destination. External scripts, SDKs, Node/Python dependencies outside included directories, linked skills (including external `.agents/skills`), project-level configs, shell credentials and executables require separate setup.

Backups, projects/worktrees, uncommitted files, generated artifacts in other folders, automations, memories directories, remote/cloud chats and OS credential stores are outside automatic migration. Referenced absolute file/project paths are not rewritten. Entire environment usability is therefore **not yet verified**.

## Consistency and limitations

Windows v2 creation checks that known Codex processes are stopped, holds deny-write/delete read handles on every selected file including DB/WAL/SHM, compares the selected inventory with preview and again before publication, syncs ZIP, then validates every SHA-256. This is an offline file-set snapshot, not SQLite online backup or semantic database validation. No process is killed. A source already corrupt cannot be made healthy by byte integrity checks.

The filesystem boundary checks every existing ancestor and rejects Windows reparse points, traversal, device paths, ADS, duplicate case-folded v2 ZIP names and links. The standard-library check/create/delete sequence is not a defense against a hostile local process replacing directory ancestors in the tiny interval after checks; use a trusted local directory with Codex stopped.

Current limits: 1 GiB per environment file, 4 MiB environment manifest, 32 in-memory plan tokens. Preview is synchronous at the Tauri boundary and large skill/resource trees may take time. Archive authentication/encryption, automatic SQLite merge, atomic multi-file plugin installation and target Desktop reindex/reopen verification are not implemented.
