use std::{
    collections::{HashMap, HashSet},
    fs,
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::platform;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationSummary {
    pub id: String,
    pub title: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub project_path: Option<String>,
    pub project_name: Option<String>,
    pub source: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationDiscovery {
    pub status: DiscoveryStatus,
    pub conversations: Vec<ConversationSummary>,
    pub total_discovered: usize,
    pub successfully_parsed: usize,
    pub skipped: usize,
    pub unsupported: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DiscoveryStatus {
    Ready,
    CodexHomeMissing,
    SessionDirectoryMissing,
    PermissionDenied,
    FilesystemUnavailable,
}

#[derive(Debug, Deserialize)]
struct SessionRecord {
    #[serde(rename = "type")]
    record_type: String,
    timestamp: Option<String>,
    payload: SessionMetadata,
}

#[derive(Debug, Deserialize)]
struct SessionMetadata {
    session_id: Option<String>,
    id: Option<String>,
    timestamp: Option<String>,
    cwd: Option<String>,
    source: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SessionIndexRecord {
    id: String,
    thread_name: Option<String>,
    updated_at: Option<String>,
}

pub fn discover_sessions() -> ConversationDiscovery {
    discover_sessions_in(&platform::codex_home())
}

pub fn discover_sessions_in(home: &Path) -> ConversationDiscovery {
    if !home.is_dir() {
        return empty(DiscoveryStatus::CodexHomeMissing);
    }

    let sessions = home.join("sessions");
    if !sessions.is_dir() {
        return empty(DiscoveryStatus::SessionDirectoryMissing);
    }

    let index = read_session_index(&home.join("session_index.jsonl"));
    let mut files = Vec::new();
    let mut unsupported = 0;
    match collect_session_files(&sessions, &mut files, &mut unsupported) {
        Ok(()) => {}
        Err(error) if error.kind() == io::ErrorKind::PermissionDenied => {
            return empty(DiscoveryStatus::PermissionDenied)
        }
        Err(_) => return empty(DiscoveryStatus::FilesystemUnavailable),
    }

    let total_discovered = files.len();
    let mut conversations = Vec::with_capacity(total_discovered);
    let mut skipped = 0;
    for file in files {
        match parse_session_metadata(&file, &index) {
            Ok(summary) => conversations.push(summary),
            Err(ParseError::Unsupported) => unsupported += 1,
            Err(ParseError::Malformed) | Err(ParseError::Unreadable) => skipped += 1,
        }
    }
    conversations.sort_by(|left, right| right.updated_at.cmp(&left.updated_at));
    let successfully_parsed = conversations.len();
    ConversationDiscovery {
        status: DiscoveryStatus::Ready,
        conversations,
        total_discovered,
        successfully_parsed,
        skipped,
        unsupported,
    }
}

#[derive(Debug, Clone)]
pub(crate) struct SelectedSession {
    pub summary: ConversationSummary,
    pub source_path: PathBuf,
    pub archive_path: String,
}

pub(crate) fn selected_sessions(
    home: &Path,
    requested_ids: &[String],
) -> Result<Vec<SelectedSession>, String> {
    if requested_ids.is_empty() {
        return Err("Select at least one discovered conversation.".into());
    }
    if requested_ids.iter().any(|id| id.is_empty()) {
        return Err("A selected conversation ID is invalid.".into());
    }
    if requested_ids.iter().collect::<HashSet<_>>().len() != requested_ids.len() {
        return Err("Duplicate conversation selections are not allowed.".into());
    }
    let sessions_root = home.join("sessions");
    let root = sessions_root
        .canonicalize()
        .map_err(|_| "The Codex sessions directory is unavailable.".to_string())?;
    let index = read_session_index(&home.join("session_index.jsonl"));
    let mut files = Vec::new();
    let mut ignored = 0;
    collect_session_files(&root, &mut files, &mut ignored)
        .map_err(|_| "The Codex sessions directory could not be read.".to_string())?;
    let mut by_id = HashMap::new();
    for path in files {
        let Ok(path) = path.canonicalize() else {
            continue;
        };
        if !path.starts_with(&root) {
            continue;
        }
        if let Ok(summary) = parse_session_metadata(&path, &index) {
            by_id.insert(summary.id.clone(), (summary, path));
        }
    }
    let mut selected = Vec::with_capacity(requested_ids.len());
    for id in requested_ids {
        let (summary, source_path) = by_id
            .remove(id)
            .ok_or_else(|| format!("Selected conversation is unavailable or unsupported: {id}"))?;
        let source_path = source_path
            .canonicalize()
            .map_err(|_| format!("Selected conversation cannot be read: {id}"))?;
        let relative = source_path.strip_prefix(&root).map_err(|_| {
            "Rejected a session path outside the Codex sessions directory.".to_string()
        })?;
        let archive_path = format!("sessions/{}", relative.to_string_lossy().replace('\\', "/"));
        if relative.components().any(|part| {
            matches!(
                part,
                std::path::Component::ParentDir
                    | std::path::Component::RootDir
                    | std::path::Component::Prefix(_)
            )
        }) {
            return Err("Rejected an unsafe session path.".into());
        }
        selected.push(SelectedSession {
            summary,
            source_path,
            archive_path,
        });
    }
    Ok(selected)
}

fn empty(status: DiscoveryStatus) -> ConversationDiscovery {
    ConversationDiscovery {
        status,
        conversations: Vec::new(),
        total_discovered: 0,
        successfully_parsed: 0,
        skipped: 0,
        unsupported: 0,
    }
}

#[derive(Default)]
struct SessionIndex(HashMap<String, SessionIndexRecord>);

impl SessionIndex {
    fn get(&self, id: &str) -> Option<&SessionIndexRecord> {
        self.0.get(id)
    }
}

fn read_session_index(path: &Path) -> SessionIndex {
    let Ok(file) = fs::File::open(path) else {
        return SessionIndex::default();
    };
    let mut records = HashMap::new();
    for line in BufReader::new(file).lines().map_while(Result::ok) {
        if let Ok(record) = serde_json::from_str::<SessionIndexRecord>(&line) {
            records.insert(record.id.clone(), record);
        }
    }
    SessionIndex(records)
}

fn collect_session_files(
    directory: &Path,
    files: &mut Vec<PathBuf>,
    unsupported: &mut usize,
) -> io::Result<()> {
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_session_files(&path, files, unsupported)?;
        } else if path
            .extension()
            .is_some_and(|extension| extension == "jsonl")
        {
            files.push(path);
        } else {
            *unsupported += 1;
        }
    }
    Ok(())
}

#[derive(Debug)]
enum ParseError {
    Unsupported,
    Malformed,
    Unreadable,
}

fn parse_session_metadata(
    path: &Path,
    index: &SessionIndex,
) -> Result<ConversationSummary, ParseError> {
    let file = fs::File::open(path).map_err(|_| ParseError::Unreadable)?;
    let mut lines = BufReader::new(file).lines();
    let first_line = lines
        .next()
        .ok_or(ParseError::Malformed)?
        .map_err(|_| ParseError::Unreadable)?;
    let record: SessionRecord =
        serde_json::from_str(&first_line).map_err(|_| ParseError::Malformed)?;
    if record.record_type != "session_meta" {
        return Err(ParseError::Unsupported);
    }
    normalize_session_metadata(record, index)
}

fn normalize_session_metadata(
    record: SessionRecord,
    index: &SessionIndex,
) -> Result<ConversationSummary, ParseError> {
    let id = record
        .payload
        .session_id
        .clone()
        .or(record.payload.id.clone())
        .filter(|value| !value.is_empty())
        .ok_or(ParseError::Malformed)?;
    let project_path = record.payload.cwd.clone().filter(|value| !value.is_empty());
    let project_name = project_path
        .as_deref()
        .and_then(|path| Path::new(path).file_name())
        .map(|name| name.to_string_lossy().to_string());
    let index_record = index.get(&id);
    Ok(ConversationSummary {
        id,
        title: index_record
            .and_then(|record| record.thread_name.clone())
            .filter(|value| !value.is_empty()),
        created_at: record.payload.timestamp.clone().or(record.timestamp),
        updated_at: index_record.and_then(|record| record.updated_at.clone()),
        project_path,
        project_name,
        source: record
            .payload
            .source
            .clone()
            .filter(|value| !value.is_empty()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_legacy_session_metadata() {
        let record = r#"{"timestamp":"2026-09-04T10:00:00Z","type":"session_meta","payload":{"session_id":"abc","cwd":"/work/demo","source":"cli"}}"#;
        let parsed: SessionRecord = serde_json::from_str(record).unwrap();
        assert_eq!(parsed.record_type, "session_meta");
        assert_eq!(parsed.payload.session_id.as_deref(), Some("abc"));
    }

    #[test]
    fn normalizes_missing_optional_metadata() {
        let record: SessionRecord =
            serde_json::from_str(r#"{"type":"session_meta","payload":{"id":"abc"}}"#).unwrap();
        let summary = normalize_session_metadata(record, &SessionIndex::default()).unwrap();
        assert_eq!(summary.id, "abc");
        assert!(summary.title.is_none());
        assert!(summary.project_path.is_none());
    }

    #[test]
    fn normalizes_index_title_and_project() {
        let record: SessionRecord = serde_json::from_str(r#"{"timestamp":"2026-09-04T10:00:00Z","type":"session_meta","payload":{"session_id":"abc","cwd":"/work/demo","source":"cli"}}"#).unwrap();
        let index = SessionIndex(HashMap::from([(
            "abc".to_string(),
            SessionIndexRecord {
                id: "abc".to_string(),
                thread_name: Some("Synthetic title".to_string()),
                updated_at: Some("2026-09-04T11:00:00Z".to_string()),
            },
        )]));
        let summary = normalize_session_metadata(record, &index).unwrap();
        assert_eq!(summary.title.as_deref(), Some("Synthetic title"));
        assert_eq!(summary.project_name.as_deref(), Some("demo"));
        assert_eq!(summary.updated_at.as_deref(), Some("2026-09-04T11:00:00Z"));
    }

    #[test]
    fn malformed_metadata_is_rejected() {
        assert!(serde_json::from_str::<SessionRecord>("{").is_err());
    }

    #[test]
    fn rejects_non_metadata_records() {
        let record: SessionRecord =
            serde_json::from_str(r#"{"type":"event_msg","payload":{}}"#).unwrap();
        assert_ne!(record.record_type, "session_meta");
    }

    #[test]
    fn project_name_uses_final_path_component() {
        assert_eq!(
            Path::new("/work/demo")
                .file_name()
                .unwrap()
                .to_string_lossy(),
            "demo"
        );
    }

    #[test]
    fn resolves_only_selected_synthetic_session_file() {
        let root = std::env::temp_dir().join(format!(
            "codex-companion-session-test-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let directory = root.join("sessions/2026/09/04");
        fs::create_dir_all(&directory).unwrap();
        fs::write(directory.join("rollout-a.jsonl"), "{\"type\":\"session_meta\",\"payload\":{\"session_id\":\"synthetic-a\"}}\n{\"type\":\"event_msg\",\"payload\":{\"message\":\"not read\"}}")
            .unwrap();
        fs::write(
            directory.join("rollout-b.jsonl"),
            r#"{"type":"session_meta","payload":{"session_id":"synthetic-b"}}"#,
        )
        .unwrap();
        let selected = selected_sessions(&root, &["synthetic-a".to_string()]).unwrap();
        assert_eq!(selected.len(), 1);
        assert_eq!(selected[0].summary.id, "synthetic-a");
        assert_eq!(
            selected[0].archive_path,
            "sessions/2026/09/04/rollout-a.jsonl"
        );
        let _ = fs::remove_dir_all(root);
    }
}
