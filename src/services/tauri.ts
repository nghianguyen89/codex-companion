import { invoke } from "@tauri-apps/api/core";
import type { AppConfiguration, ArchiveInspection, BackupPreview, BackupResult, CodexPaths, ConversationDiscovery, DeletePreview, DeleteResult, DiagnosticsSnapshot, RestoreHistoryEntry, RestorePreview, RestoreResult } from "../types/codex";

export const getDiagnostics = (): Promise<DiagnosticsSnapshot> => invoke("get_diagnostics");
export const getPaths = (): Promise<CodexPaths> => invoke("get_codex_paths");
export const discoverConversations = (): Promise<ConversationDiscovery> => invoke("discover_conversations");
export const previewBackup = (selectedIds: string[]): Promise<BackupPreview> => invoke("preview_backup", { selectedIds });
export const createBackup = (selectedIds: string[]): Promise<BackupResult> => invoke("create_backup", { selectedIds });
export const inspectBackupArchive = (): Promise<ArchiveInspection | null> => invoke("inspect_backup_archive");
export const previewRestore = (restoreToken: string, selectedIds: string[]): Promise<RestorePreview> => invoke("preview_restore", { restoreToken, selectedIds });
export const restoreArchive = (restoreToken: string, selectedIds: string[]): Promise<RestoreResult> => invoke("restore_archive", { restoreToken, selectedIds });
export const getRestoreHistory = (): Promise<RestoreHistoryEntry[]> => invoke("get_restore_history");
export const previewLocalDelete = (selectedIds: string[]): Promise<DeletePreview> => invoke("preview_local_delete", { selectedIds });
export const executeLocalDelete = (selectedIds: string[], confirmation: string): Promise<DeleteResult> => invoke("execute_local_delete", { selectedIds, confirmation });
export const getConfiguration = (): Promise<AppConfiguration> => invoke("get_configuration");
export const saveConfiguration = (configuration: AppConfiguration): Promise<void> =>
  invoke("save_configuration", { configuration });
