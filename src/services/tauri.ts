import { invoke } from "@tauri-apps/api/core";
import type { AppConfiguration, ArchiveInspection, BackupPreview, BackupResult, BeyondCompareBundleInspection, BeyondCompareBundlePreview, BeyondCompareReadiness, BeyondCompareRecoveryPreview, CodexPaths, ConversationDiscovery, DeletePreview, DeleteResult, DiagnosticsSnapshot, RestoreHistoryEntry, RestorePreview, RestoreResult, SourceTreeInspection, SourceTreePreview, SourceTreeReadiness, SourceTreeRecoveryPreview } from "../types/codex";

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
export const getBeyondCompareReadiness = (): Promise<BeyondCompareReadiness> => invoke("get_beyond_compare_readiness");
export const previewBeyondCompare = (secretExportDisabled: boolean): Promise<BeyondCompareBundlePreview | null> => invoke("preview_beyond_compare", { secretExportDisabled });
export const createBeyondCompareBundle = (token: string): Promise<{ bundleName: string; bytes: number }> => invoke("create_beyond_compare_bundle", { token });
export const inspectBeyondCompareBundle = (): Promise<BeyondCompareBundleInspection | null> => invoke("inspect_beyond_compare_bundle");
export const previewBeyondCompareRecovery = (token: string): Promise<BeyondCompareRecoveryPreview> => invoke("preview_beyond_compare_recovery", { token });
export const recoverBeyondCompare = (token: string, confirmation: string): Promise<{ recovered: boolean; stagingPath: string }> => invoke("recover_beyond_compare", { token, confirmation });
export const getSourceTreeReadiness = (): Promise<SourceTreeReadiness> => invoke("get_sourcetree_readiness");
export const previewSourceTree = (): Promise<SourceTreePreview> => invoke("preview_sourcetree");
export const createSourceTreeBundle = (token: string): Promise<{ bundleName: string; bytes: number }> => invoke("create_sourcetree_bundle", { token });
export const inspectSourceTreeBundle = (): Promise<SourceTreeInspection | null> => invoke("inspect_sourcetree_bundle");
export const previewSourceTreeRecovery = (token: string): Promise<SourceTreeRecoveryPreview> => invoke("preview_sourcetree_recovery", { token });
export const recoverSourceTree = (token: string, confirmation: string): Promise<{ recovered: boolean; stagingPath: string; manualOnly: boolean }> => invoke("recover_sourcetree", { token, confirmation });
export const getConfiguration = (): Promise<AppConfiguration> => invoke("get_configuration");
export const saveConfiguration = (configuration: AppConfiguration): Promise<void> =>
  invoke("save_configuration", { configuration });
