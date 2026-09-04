export type OperatingSystem = "windows" | "macos" | "linux" | "unknown";

export interface CodexPaths {
  codexHome: string;
  configDir: string;
  backupDir: string;
  portableMode: boolean;
}

export interface DiagnosticsSnapshot {
  operatingSystem: OperatingSystem;
  architecture: string;
  codexHome: string;
  codexHomeExists: boolean;
  configDir: string;
  backupDir: string;
  codexCliVersion: string | null;
  skillsCount: number;
  petsCount: number;
}

export interface AppConfiguration {
  theme: "system" | "light" | "dark";
  portableMode: boolean;
  createSafetyBackups: boolean;
  language: "en" | "vi";
  logLevel: "error" | "warn" | "info" | "debug";
}

export type ConversationDiscoveryStatus = "ready" | "codexHomeMissing" | "sessionDirectoryMissing" | "permissionDenied" | "filesystemUnavailable";

export interface ConversationSummary {
  id: string;
  title: string | null;
  createdAt: string | null;
  updatedAt: string | null;
  projectPath: string | null;
  projectName: string | null;
  source: string | null;
}

export interface ConversationDiscovery {
  status: ConversationDiscoveryStatus;
  conversations: ConversationSummary[];
  totalDiscovered: number;
  successfullyParsed: number;
  skipped: number;
  unsupported: number;
}

export interface BackupSession {
  id: string;
  title: string | null;
  createdAt: string | null;
  updatedAt: string | null;
  archivePath: string;
  bytes: number;
}

export interface BackupPreview {
  formatVersion: number;
  sessionCount: number;
  totalBytes: number;
  backupDirectory: string;
  sessions: BackupSession[];
}

export interface BackupResult {
  archivePath: string;
  sessionCount: number;
  totalBytes: number;
}

export interface ArchiveInspection {
  archiveName: string;
  createdAt: string | null;
  platform: string | null;
  codexCliVersion: string | null;
  sessionCount: number;
  totalBytes: number;
  validation: { valid: boolean; formatVersion: number | null };
  warnings: string[];
  errors: string[];
  restoreToken: string | null;
  sessions: BackupSession[];
}
export interface RestorePreview { sessionCount: number; totalBytes: number; destinationRoot: string; sessions: Array<{ id: string; archivePath: string; destinationPath: string; bytes: number; conflict: boolean }>; conflictCount: number; plannedCreates: number; safetyBackupWillBeCreated: boolean; }
export interface RestoreResult { restoredCount: number; skippedConflicts: number; totalBytes: number; safetyBackupPath: string | null; }
