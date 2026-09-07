// Manual browser fixture only. Not part of the production Vite entry or native bundle.
const entry = { path: "skills/demo/SKILL.md", group: "skills", bytes: 42, manual: false };
const session = { id: "synthetic", title: "Synthetic UI fixture", archivePath: "sessions/test.jsonl", bytes: 42, createdAt: null, updatedAt: null };
globalThis.__TAURI_INTERNALS__ = { invoke: async command => {
  await new Promise(resolve => globalThis.setTimeout(resolve, 250));
  switch (command) {
    case "get_configuration": return { theme: "system", portableMode: true, createSafetyBackups: true, language: "vi", logLevel: "warn" };
    case "get_diagnostics": return { operatingSystem: "windows", architecture: "x86_64", codexHome: "D:/synthetic/home", codexHomeExists: true, configDir: "D:/synthetic/config", backupDir: "D:/synthetic/backups", codexCliVersion: "fixture", skillsCount: 1, petsCount: 1 };
    case "get_restore_history": return [];
    case "discover_conversations": return { status: "ready", conversations: [{ ...session, projectName: "synthetic", projectPath: "D:/synthetic/project", source: "subagent:guardian" }], totalDiscovered: 1, successfullyParsed: 1, skipped: 0, unsupported: 0 };
    case "preview_environment": return { token: "synthetic-preview", groups: [{ id: "skills", files: 1, bytes: 42, reason: "Synthetic fixture" }], excluded: [{ id: "auth", files: 1, bytes: 100, reason: "Credentials excluded" }], entries: [entry] };
    case "create_environment": return { archivePath: "D:/synthetic/environment.zip", archiveBytes: 1024, files: 1 };
    case "inspect_environment": return { token: "synthetic-archive", groups: [], excluded: [], entries: [entry] };
    case "preview_environment_restore": return { token: "synthetic-restore", items: [{ path: entry.path, status: "new", bytes: 42 }, { path: "state_5.sqlite", status: "manual", bytes: 2000 }, { path: "pets/demo/pet.json", status: "conflict", bytes: 42 }] };
    case "restore_environment": return { restored: 1, skipped: 2, errors: [], rollbackRemaining: 0 };
    case "scan_cleanup": return { token: "synthetic-cleanup", location: "D:/synthetic/home/cache/remote_plugin_catalog", files: 2, bytes: 123456, skipped: 1, blocked: null };
    case "execute_cleanup": return { deleted: 1, skipped: 1, reclaimedBytes: 123, errors: [] };
    case "inspect_backup_archive": return { archiveName: "synthetic.zip", createdAt: null, platform: "windows", codexCliVersion: "fixture", sessionCount: 1, totalBytes: 42, validation: { valid: true, formatVersion: 1 }, warnings: [], errors: [], restoreToken: "fixture", sessions: [session] };
    case "preview_restore": return { sessionCount: 1, totalBytes: 42, destinationRoot: "D:/synthetic/home/sessions", sessions: [session], conflictCount: 0, plannedCreates: 1, safetyBackupWillBeCreated: false };
    case "restore_archive": return { restoredCount: 1, skippedConflicts: 0, totalBytes: 42, safetyBackupPath: null };
    default: throw new Error(`Unimplemented synthetic command: ${command}`);
  }
} };
await import("/src/main.tsx");
