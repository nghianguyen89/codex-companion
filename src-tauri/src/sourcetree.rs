//! Strict Windows-only SourceTree bookmark backup. No repository contents or app configuration is copied.
use std::{
    collections::{HashMap, HashSet},
    fs::{self, File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicU64, Ordering},
        Mutex, OnceLock,
    },
};

use quick_xml::{events::Event, Reader};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use zip::{write::SimpleFileOptions, CompressionMethod, ZipArchive, ZipWriter};

use crate::{fs_safety, platform};

type Result<T> = std::result::Result<T, String>;
const MANIFEST: &str = "personal-manifest.json";
const ARTIFACT: &str = "apps/sourcetree/bookmarks.xml";
const MAX_MANIFEST: u64 = 1024 * 1024;
const MAX_BOOKMARKS: u64 = 16 * 1024 * 1024;
const APP: &str = "sourcetree";

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Artifact {
    app_id: String,
    adapter_format_version: u32,
    source_app_version: String,
    kind: String,
    archive_path: String,
    files: u32,
    bytes: u64,
    sha256: String,
    restore_mode: String,
    repository_paths: Vec<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Manifest {
    format_version: u32,
    kind: String,
    created_at: String,
    platform: String,
    sensitive: bool,
    artifacts: Vec<Artifact>,
}
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Readiness {
    pub supported: bool,
    pub bookmarks_found: bool,
}
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Preview {
    pub token: String,
    pub source_app_version: String,
    pub bookmark_count: usize,
    pub repository_paths: Vec<String>,
    pub bytes: u64,
    pub sensitive: bool,
}
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Created {
    pub bundle_name: String,
    pub bytes: u64,
}
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Inspection {
    pub token: String,
    pub bundle_name: String,
    pub created_at: String,
    pub source_app_version: String,
    pub bookmark_count: usize,
    pub bytes: u64,
    pub sensitive: bool,
}
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecoveryPreview {
    pub token: String,
    pub staging_path: String,
    pub destination_path: String,
    pub destination_conflict: bool,
    pub manual_only: bool,
}
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecoveryResult {
    pub recovered: bool,
    pub staging_path: String,
    pub manual_only: bool,
}

#[derive(Clone, Debug, PartialEq)]
struct Identity {
    bytes: u64,
    sha256: String,
    version: String,
    paths: Vec<String>,
}
#[derive(Clone)]
enum Plan {
    Create {
        source: PathBuf,
        identity: Identity,
    },
    Inspect {
        bundle: PathBuf,
        hash: String,
    },
    Recover {
        bundle: PathBuf,
        hash: String,
        target: PathBuf,
        destination: PathBuf,
        destination_conflict: bool,
    },
}
static PLANS: OnceLock<Mutex<HashMap<String, Plan>>> = OnceLock::new();
static NEXT: AtomicU64 = AtomicU64::new(1);
fn put(plan: Plan) -> String {
    let token = format!("sourcetree-{}", NEXT.fetch_add(1, Ordering::Relaxed));
    let mut plans = PLANS
        .get_or_init(Default::default)
        .lock()
        .expect("plan lock");
    if plans.len() >= 32 {
        plans.clear();
    }
    plans.insert(token.clone(), plan);
    token
}
fn get(token: &str) -> Result<Plan> {
    PLANS
        .get_or_init(Default::default)
        .lock()
        .map_err(|_| "Preview expired; preview again.".to_string())?
        .get(token)
        .cloned()
        .ok_or_else(|| "Preview expired; preview again.".into())
}
fn hash_reader(mut reader: impl Read, limit: u64) -> Result<(String, u64)> {
    let mut hash = Sha256::new();
    let mut bytes = 0u64;
    let mut buffer = [0; 65536];
    loop {
        let count = reader
            .read(&mut buffer)
            .map_err(|_| "Cannot read SourceTree bookmarks.".to_string())?;
        if count == 0 {
            break;
        }
        bytes = bytes
            .checked_add(count as u64)
            .ok_or("SourceTree bookmarks exceed the size limit.")?;
        if bytes > limit {
            return Err("SourceTree bookmarks exceed the size limit.".into());
        }
        hash.update(&buffer[..count]);
    }
    Ok((format!("{:x}", hash.finalize()), bytes))
}
fn hash_file(path: &Path) -> Result<(String, u64)> {
    hash_reader(
        File::open(path).map_err(|_| "Cannot read SourceTree bookmarks.".to_string())?,
        MAX_BOOKMARKS,
    )
}
fn timestamp() -> Result<String> {
    time::OffsetDateTime::now_utc()
        .format(&time::format_description::well_known::Rfc3339)
        .map_err(|_| "Cannot create bundle timestamp.".into())
}
fn bundle_name(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("sourcetree-bundle.zip")
        .to_owned()
}

pub fn bookmarks_path() -> Result<PathBuf> {
    std::env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .map(|root| {
            root.join("Atlassian")
                .join("SourceTree")
                .join("bookmarks.xml")
        })
        .ok_or_else(|| "Cannot locate the Windows LocalAppData folder.".into())
}
fn open_regular(path: &Path) -> Result<File> {
    fs_safety::check(path)?;
    let metadata =
        fs::symlink_metadata(path).map_err(|_| "Cannot read SourceTree bookmarks.".to_string())?;
    if fs_safety::linked(&metadata) || !metadata.is_file() || metadata.len() > MAX_BOOKMARKS {
        return Err("SourceTree bookmarks are not a supported regular file.".into());
    }
    let mut options = OpenOptions::new();
    options.read(true);
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        options.share_mode(1);
    }
    options
        .open(path)
        .map_err(|_| "Cannot read SourceTree bookmarks. Close SourceTree and retry.".into())
}
fn is_reserved(part: &str) -> bool {
    let stem = part.split('.').next().unwrap_or("").to_ascii_uppercase();
    matches!(stem.as_str(), "CON" | "PRN" | "AUX" | "NUL")
        || (stem.len() == 4
            && (stem.starts_with("COM") || stem.starts_with("LPT"))
            && stem.as_bytes()[3].is_ascii_digit())
}
fn safe_repository_path(value: &str) -> Result<()> {
    if value.is_empty()
        || value.len() > 8192
        || value.contains('\0')
        || value.starts_with("\\\\?\\")
        || value.starts_with("\\\\.\\")
    {
        return Err("SourceTree bookmark contains an unsafe path.".into());
    }
    if let Some((scheme, rest)) = value.split_once("://") {
        if scheme.is_empty()
            || rest.split('/').next().unwrap_or("").contains('@')
            || rest.split('/').any(|part| part == "..")
        {
            return Err("SourceTree bookmark contains a credential-bearing or unsafe URL.".into());
        }
        return Ok(());
    }
    if value.contains(':')
        && !(value.len() >= 3
            && value.as_bytes()[1] == b':'
            && value.as_bytes()[2] == b'\\'
            && !value[2..].contains(':'))
    {
        return Err("SourceTree bookmark contains an unsafe path.".into());
    }
    if value
        .split(['\\', '/'])
        .any(|part| part == ".." || part.ends_with(['.', ' ']) || is_reserved(part))
    {
        return Err("SourceTree bookmark contains an unsafe path.".into());
    }
    Ok(())
}
fn no_attributes(event: &quick_xml::events::BytesStart<'_>) -> Result<()> {
    if event.attributes().next().is_some() {
        Err("Unsupported SourceTree bookmarks schema.".into())
    } else {
        Ok(())
    }
}
fn xml_text(text: quick_xml::events::BytesText<'_>) -> Result<String> {
    let decoded = text
        .decode()
        .map_err(|_| "Malformed SourceTree bookmarks.xml.")?;
    quick_xml::escape::unescape(&decoded)
        .map(|value| value.into_owned())
        .map_err(|_| "Malformed SourceTree bookmarks.xml.".into())
}
fn parse_bookmarks(bytes: &[u8]) -> Result<Vec<String>> {
    let mut reader = Reader::from_reader(bytes);
    reader.config_mut().trim_text(false);
    reader.config_mut().check_end_names = true;
    let mut buffer = Vec::new();
    let mut state = 0;
    let mut paths = Vec::new();
    let mut current = String::new();
    loop {
        match reader
            .read_event_into(&mut buffer)
            .map_err(|_| "Malformed or unsupported SourceTree bookmarks.xml.".to_string())?
        {
            Event::Decl(_) if state == 0 => (),
            Event::Start(event) => {
                let name = event.name();
                if state == 0 && name.as_ref() == b"ArrayOfBookmark" {
                    no_attributes(&event)?;
                    state = 1;
                } else if state == 1 && name.as_ref() == b"Bookmark" {
                    no_attributes(&event)?;
                    state = 2;
                } else if state == 2 && name.as_ref() == b"Name" {
                    no_attributes(&event)?;
                    state = 3;
                    current.clear();
                } else if state == 4 && name.as_ref() == b"Path" {
                    no_attributes(&event)?;
                    state = 5;
                    current.clear();
                } else {
                    return Err("Unsupported SourceTree bookmarks schema.".into());
                }
            }
            Event::Text(text) if state == 3 || state == 5 => current.push_str(&xml_text(text)?),
            Event::End(event) => {
                let name = event.name();
                if state == 3 && name.as_ref() == b"Name" {
                    if current.is_empty() {
                        return Err("Unsupported SourceTree bookmarks schema.".into());
                    }
                    state = 4;
                } else if state == 5 && name.as_ref() == b"Path" {
                    safe_repository_path(&current)?;
                    paths.push(current.clone());
                    state = 6;
                } else if state == 6 && name.as_ref() == b"Bookmark" {
                    state = 1;
                } else if state == 1 && name.as_ref() == b"ArrayOfBookmark" {
                    state = 7;
                } else {
                    return Err("Malformed or unsupported SourceTree bookmarks.xml.".into());
                }
            }
            Event::Eof => break,
            Event::Text(text) if xml_text(text.clone())?.trim().is_empty() => (),
            _ => return Err("Malformed or unsupported SourceTree bookmarks.xml.".into()),
        }
        buffer.clear();
    }
    if state != 7 || paths.is_empty() || paths.len() > 10000 {
        return Err("Malformed or unsupported SourceTree bookmarks.xml.".into());
    }
    Ok(paths)
}
fn snapshot(source: &Path, version: String) -> Result<Identity> {
    let input = open_regular(source)?;
    let mut bytes = Vec::new();
    input
        .take(MAX_BOOKMARKS + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| "Cannot read SourceTree bookmarks.".to_string())?;
    if bytes.len() as u64 > MAX_BOOKMARKS {
        return Err("SourceTree bookmarks exceed the size limit.".into());
    }
    let paths = parse_bookmarks(&bytes)?;
    Ok(Identity {
        bytes: bytes.len() as u64,
        sha256: format!("{:x}", Sha256::digest(&bytes)),
        version,
        paths,
    })
}
fn source_version() -> Result<String> {
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        for key in [
            r"HKCU\Software\Microsoft\Windows\CurrentVersion\Uninstall\SourceTree",
            r"HKLM\Software\Microsoft\Windows\CurrentVersion\Uninstall\SourceTree",
            r"HKLM\Software\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall\SourceTree",
        ] {
            if let Ok(output) = std::process::Command::new("reg.exe")
                .args(["query", key, "/v", "DisplayVersion"])
                .creation_flags(0x08000000)
                .output()
            {
                if output.status.success() {
                    if let Some(value) = String::from_utf8_lossy(&output.stdout)
                        .lines()
                        .find_map(|line| line.split("REG_SZ").nth(1).map(str::trim))
                        .filter(|value| !value.is_empty())
                    {
                        return Ok(value.to_owned());
                    }
                }
            }
        }
        Err("Cannot determine the installed SourceTree version.".into())
    }
    #[cfg(not(windows))]
    {
        Err("SourceTree migration currently supports Windows only.".into())
    }
}
fn has_process(output: &str) -> bool {
    output.lines().any(|line| {
        line.split(',')
            .next()
            .unwrap_or("")
            .trim_matches('"')
            .eq_ignore_ascii_case("SourceTree.exe")
    })
}
pub fn require_closed() -> Result<()> {
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        let output = std::process::Command::new("tasklist.exe")
            .args(["/FO", "CSV", "/NH"])
            .creation_flags(0x08000000)
            .output()
            .map_err(|_| "Cannot check whether SourceTree is closed.")?;
        if !output.status.success() {
            return Err("Cannot check whether SourceTree is closed.".into());
        }
        if has_process(&String::from_utf8_lossy(&output.stdout)) {
            return Err("Close SourceTree before preview, creation, or recovery. Companion never closes it automatically.".into());
        }
        Ok(())
    }
    #[cfg(not(windows))]
    {
        Err("SourceTree migration currently supports Windows only.".into())
    }
}
pub fn readiness() -> Readiness {
    Readiness {
        supported: cfg!(windows),
        bookmarks_found: bookmarks_path().is_ok_and(|path| {
            fs::symlink_metadata(path)
                .is_ok_and(|metadata| metadata.is_file() && !fs_safety::linked(&metadata))
        }),
    }
}

pub fn preview() -> Result<Preview> {
    require_closed()?;
    let source = bookmarks_path()?;
    let identity = snapshot(&source, source_version()?)?;
    let token = put(Plan::Create {
        source,
        identity: identity.clone(),
    });
    Ok(Preview {
        token,
        source_app_version: identity.version,
        bookmark_count: identity.paths.len(),
        repository_paths: identity.paths,
        bytes: identity.bytes,
        sensitive: true,
    })
}
pub fn create(token: &str) -> Result<Created> {
    require_closed()?;
    let Plan::Create { source, identity } = get(token)? else {
        return Err("Preview SourceTree bookmarks first.".into());
    };
    if source_version()? != identity.version {
        return Err("SourceTree bookmarks or installed version changed; preview again.".into());
    }
    create_revalidated_at(&source, &identity, &platform::personal_bundle_dir())
}
fn create_revalidated_at(source: &Path, identity: &Identity, output: &Path) -> Result<Created> {
    if snapshot(source, identity.version.clone())? != *identity {
        return Err("SourceTree bookmarks or installed version changed; preview again.".into());
    }
    create_at(source, identity, output)
}
fn create_at(source: &Path, identity: &Identity, output: &Path) -> Result<Created> {
    fs_safety::check(output)?;
    fs::create_dir_all(output)
        .map_err(|_| "Cannot create the Companion bundle folder.".to_string())?;
    fs_safety::check(output)?;
    let path = output.join(format!(
        "sourcetree-bundle-{}.zip",
        time::OffsetDateTime::now_utc().unix_timestamp_nanos()
    ));
    let mut created = false;
    let result = (|| -> Result<()> {
        let output = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .map_err(|_| "Cannot create the SourceTree bundle.")?;
        created = true;
        let manifest = Manifest {
            format_version: 1,
            kind: "codex-companion-personal-bundle".into(),
            created_at: timestamp()?,
            platform: "windows".into(),
            sensitive: true,
            artifacts: vec![Artifact {
                app_id: APP.into(),
                adapter_format_version: 1,
                source_app_version: identity.version.clone(),
                kind: "bookmarks".into(),
                archive_path: ARTIFACT.into(),
                files: 1,
                bytes: identity.bytes,
                sha256: identity.sha256.clone(),
                restore_mode: "manual".into(),
                repository_paths: identity.paths.clone(),
            }],
        };
        let mut zip = ZipWriter::new(output);
        let options = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);
        zip.start_file(MANIFEST, options)
            .map_err(|_| "Cannot create the SourceTree bundle.")?;
        zip.write_all(
            &serde_json::to_vec(&manifest).map_err(|_| "Cannot create the SourceTree bundle.")?,
        )
        .map_err(|_| "Cannot create the SourceTree bundle.")?;
        zip.start_file(ARTIFACT, options)
            .map_err(|_| "Cannot create the SourceTree bundle.")?;
        let mut input = open_regular(source)?;
        let copied = std::io::copy(&mut input, &mut zip)
            .map_err(|_| "Cannot create the SourceTree bundle.")?;
        if copied != identity.bytes {
            return Err("SourceTree bookmarks changed; preview again.".into());
        }
        zip.finish()
            .map_err(|_| "Cannot finalize the SourceTree bundle.")?
            .sync_all()
            .map_err(|_| "Cannot finalize the SourceTree bundle.")?;
        inspect_at(&path).map(|_| ())
    })();
    if let Err(error) = result {
        if created && fs::remove_file(&path).is_err() {
            return Err(
                "SourceTree bundle creation failed and the partial file could not be removed."
                    .into(),
            );
        }
        return Err(error);
    }
    Ok(Created {
        bundle_name: bundle_name(&path),
        bytes: fs::metadata(path)
            .map_err(|_| "Cannot verify the SourceTree bundle.")?
            .len(),
    })
}
fn valid_hash(hash: &str) -> bool {
    hash.len() == 64
        && hash
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}
fn validate_manifest(manifest: &Manifest) -> Result<&Artifact> {
    if manifest.format_version != 1
        || manifest.kind != "codex-companion-personal-bundle"
        || manifest.platform != "windows"
        || !manifest.sensitive
        || manifest.artifacts.len() != 1
    {
        return Err("Unsupported SourceTree bundle manifest.".into());
    }
    time::OffsetDateTime::parse(
        &manifest.created_at,
        &time::format_description::well_known::Rfc3339,
    )
    .map_err(|_| "Invalid SourceTree bundle timestamp.")?;
    let artifact = &manifest.artifacts[0];
    if artifact.app_id != APP
        || artifact.adapter_format_version != 1
        || artifact.source_app_version.is_empty()
        || artifact.kind != "bookmarks"
        || artifact.archive_path != ARTIFACT
        || artifact.files != 1
        || artifact.restore_mode != "manual"
        || artifact.bytes > MAX_BOOKMARKS
        || !valid_hash(&artifact.sha256)
        || artifact.repository_paths.is_empty()
        || artifact
            .repository_paths
            .iter()
            .any(|path| safe_repository_path(path).is_err())
    {
        return Err("Unsupported SourceTree bundle artifact.".into());
    }
    Ok(artifact)
}
fn inspect_archive(archive: &mut ZipArchive<File>) -> Result<(Manifest, u64)> {
    if archive.len() != 2 {
        return Err("SourceTree bundle has an invalid ZIP inventory.".into());
    }
    let mut names = HashSet::new();
    let mut total = 0u64;
    for index in 0..archive.len() {
        let entry = archive
            .by_index(index)
            .map_err(|_| "Invalid SourceTree bundle.")?;
        fs_safety::relative(entry.name())
            .map_err(|_| "SourceTree bundle has an unsafe ZIP path.")?;
        if entry.is_dir()
            || entry
                .unix_mode()
                .is_some_and(|mode| mode & 0o170000 == 0o120000)
            || !names.insert(entry.name().to_ascii_lowercase())
        {
            return Err("SourceTree bundle has an invalid ZIP inventory.".into());
        }
        let limit = if entry.name() == MANIFEST {
            MAX_MANIFEST
        } else if entry.name() == ARTIFACT {
            MAX_BOOKMARKS
        } else {
            return Err("SourceTree bundle has unexpected ZIP entries.".into());
        };
        if entry.size() > limit {
            return Err("SourceTree bundle exceeds the size limit.".into());
        }
        total = total
            .checked_add(entry.size())
            .ok_or("SourceTree bundle exceeds the size limit.")?;
    }
    if !names.contains(MANIFEST) || !names.contains(ARTIFACT) {
        return Err("SourceTree bundle has an incomplete ZIP inventory.".into());
    }
    let mut raw = Vec::new();
    archive
        .by_name(MANIFEST)
        .map_err(|_| "SourceTree bundle has an incomplete ZIP inventory.")?
        .take(MAX_MANIFEST + 1)
        .read_to_end(&mut raw)
        .map_err(|_| "Cannot read the SourceTree bundle manifest.")?;
    if raw.len() as u64 > MAX_MANIFEST {
        return Err("SourceTree bundle manifest exceeds the size limit.".into());
    }
    let manifest: Manifest =
        serde_json::from_slice(&raw).map_err(|_| "Invalid SourceTree bundle manifest.")?;
    let artifact = validate_manifest(&manifest)?.clone();
    let mut entry = archive
        .by_name(ARTIFACT)
        .map_err(|_| "SourceTree bundle has an incomplete ZIP inventory.")?;
    if entry.size() != artifact.bytes
        || hash_reader(&mut entry, MAX_BOOKMARKS)? != (artifact.sha256, artifact.bytes)
    {
        return Err("SourceTree bundle verification failed.".into());
    }
    drop(entry);
    let mut xml = Vec::new();
    archive
        .by_name(ARTIFACT)
        .map_err(|_| "SourceTree bundle has an incomplete ZIP inventory.")?
        .read_to_end(&mut xml)
        .map_err(|_| "Cannot read SourceTree bookmarks.")?;
    if parse_bookmarks(&xml)? != artifact.repository_paths {
        return Err("SourceTree bundle inventory does not match bookmarks.".into());
    }
    Ok((manifest, total))
}
fn open_bundle(path: &Path) -> Result<File> {
    fs_safety::check(path)?;
    let metadata = fs::symlink_metadata(path).map_err(|_| "Cannot read the SourceTree bundle.")?;
    if fs_safety::linked(&metadata) || !metadata.is_file() {
        return Err("Cannot read the SourceTree bundle.".into());
    }
    File::open(path).map_err(|_| "Cannot read the SourceTree bundle.".into())
}
fn inspect_at(path: &Path) -> Result<(Manifest, u64)> {
    let mut archive =
        ZipArchive::new(open_bundle(path)?).map_err(|_| "Invalid SourceTree bundle.")?;
    inspect_archive(&mut archive)
}
fn inspect_hash(path: &Path) -> Result<(Manifest, u64, String)> {
    let mut file = open_bundle(path)?;
    let hash = hash_reader(&mut file, MAX_MANIFEST + MAX_BOOKMARKS + 65536)?.0;
    file.seek(SeekFrom::Start(0))
        .map_err(|_| "Cannot read the SourceTree bundle.")?;
    let mut archive = ZipArchive::new(file).map_err(|_| "Invalid SourceTree bundle.")?;
    let (manifest, bytes) = inspect_archive(&mut archive)?;
    Ok((manifest, bytes, hash))
}
pub fn inspect(path: PathBuf) -> Result<Inspection> {
    let (manifest, bytes, hash) = inspect_hash(&path)?;
    let artifact = validate_manifest(&manifest)?;
    let source_app_version = artifact.source_app_version.clone();
    let bookmark_count = artifact.repository_paths.len();
    let token = put(Plan::Inspect {
        bundle: path.clone(),
        hash,
    });
    Ok(Inspection {
        token,
        bundle_name: bundle_name(&path),
        created_at: manifest.created_at,
        source_app_version,
        bookmark_count,
        bytes,
        sensitive: true,
    })
}
fn destination_state(path: &Path) -> Result<bool> {
    fs_safety::check(path)?;
    match fs::symlink_metadata(path) {
        Ok(metadata) if fs_safety::linked(&metadata) => {
            Err("SourceTree destination is a symbolic link or junction.".into())
        }
        Ok(_) => Ok(true),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(_) => Err("Cannot inspect the SourceTree destination.".into()),
    }
}
pub fn preview_recovery(token: &str) -> Result<RecoveryPreview> {
    require_closed()?;
    let Plan::Inspect { bundle, hash } = get(token)? else {
        return Err("Inspect a SourceTree bundle first.".into());
    };
    let (manifest, _, actual) = inspect_hash(&bundle)?;
    if actual != hash {
        return Err("SourceTree bundle changed; inspect again.".into());
    }
    validate_manifest(&manifest)?;
    let destination = bookmarks_path()?;
    let destination_conflict = destination_state(&destination)?;
    let target = platform::personal_staging_dir()
        .join("sourcetree")
        .join(format!("recovery-{}", NEXT.fetch_add(1, Ordering::Relaxed)))
        .join("bookmarks.xml");
    fs_safety::check(&target)?;
    if target.exists() {
        return Err("Companion staging destination already exists; inspect again.".into());
    }
    let staging_path = target.to_string_lossy().into_owned();
    let destination_path = destination.to_string_lossy().into_owned();
    let recovery = put(Plan::Recover {
        bundle,
        hash,
        target,
        destination,
        destination_conflict,
    });
    Ok(RecoveryPreview {
        token: recovery,
        staging_path,
        destination_path,
        destination_conflict,
        manual_only: true,
    })
}
fn restore_at(
    bundle: &Path,
    hash: &str,
    target: &Path,
    destination: &Path,
    expected_destination_conflict: bool,
    fail_after_write: bool,
) -> Result<()> {
    let mut file = open_bundle(bundle)?;
    if hash_reader(&mut file, MAX_MANIFEST + MAX_BOOKMARKS + 65536)?.0 != hash {
        return Err("SourceTree bundle changed; inspect again.".into());
    }
    file.seek(SeekFrom::Start(0))
        .map_err(|_| "Cannot read the SourceTree bundle.")?;
    let mut archive = ZipArchive::new(file).map_err(|_| "Invalid SourceTree bundle.")?;
    let (manifest, _) = inspect_archive(&mut archive)?;
    let artifact = validate_manifest(&manifest)?;
    if destination_state(destination)? != expected_destination_conflict || target.exists() {
        return Err("SourceTree destination changed; preview again.".into());
    }
    let parent = target
        .parent()
        .ok_or("Invalid Companion staging destination.")?;
    fs_safety::check(parent)?;
    fs::create_dir_all(parent).map_err(|_| "Cannot create Companion staging.")?;
    fs_safety::check(target)?;
    let mut created = false;
    let attempt = (|| -> Result<()> {
        let mut output = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(target)
            .map_err(|_| "Companion staging destination changed; preview again.")?;
        created = true;
        let copied = std::io::copy(
            &mut archive
                .by_name(ARTIFACT)
                .map_err(|_| "SourceTree bundle has an incomplete ZIP inventory.")?
                .take(artifact.bytes + 1),
            &mut output,
        )
        .map_err(|_| "Cannot recover SourceTree bookmarks to Companion staging.")?;
        if copied != artifact.bytes {
            return Err("Recovered SourceTree bookmarks size verification failed.".into());
        }
        output
            .sync_all()
            .map_err(|_| "Cannot recover SourceTree bookmarks to Companion staging.")?;
        drop(output);
        if fail_after_write {
            return Err("Injected recovery failure.".into());
        }
        if hash_file(target)? != (artifact.sha256.clone(), artifact.bytes) {
            return Err("Recovered SourceTree bookmarks verification failed.".into());
        }
        Ok(())
    })();
    if let Err(error) = attempt {
        if created && fs::remove_file(target).is_err() {
            return Err(
                "SourceTree recovery failed and Companion staging could not be rolled back.".into(),
            );
        }
        return Err(error);
    }
    Ok(())
}
pub fn recover(token: &str, confirmation: &str) -> Result<RecoveryResult> {
    if confirmation != "RECOVER" {
        return Err("Type RECOVER to confirm.".into());
    }
    require_closed()?;
    let Plan::Recover {
        bundle,
        hash,
        target,
        destination,
        destination_conflict,
    } = get(token)?
    else {
        return Err("Preview SourceTree recovery first.".into());
    };
    restore_at(
        &bundle,
        &hash,
        &target,
        &destination,
        destination_conflict,
        false,
    )?;
    Ok(RecoveryResult {
        recovered: true,
        staging_path: target.to_string_lossy().into_owned(),
        manual_only: true,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn root() -> PathBuf {
        std::env::temp_dir().join(format!(
            "sourcetree-test-{}",
            time::OffsetDateTime::now_utc().unix_timestamp_nanos()
        ))
    }
    fn xml(path: &Path, value: &str) {
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, format!("<?xml version=\"1.0\"?><ArrayOfBookmark><Bookmark><Name>demo</Name><Path>{value}</Path></Bookmark></ArrayOfBookmark>")).unwrap();
    }
    fn source(root: &Path) -> PathBuf {
        let path = root.join("source/bookmarks.xml");
        xml(&path, "C:\\repos\\demo");
        path
    }
    fn identity(path: &Path) -> Identity {
        snapshot(path, "3.4.0".into()).unwrap()
    }
    fn bundle(path: &Path, identity: &Identity) {
        create_at(path, identity, path.parent().unwrap().join("out").as_path()).unwrap();
    }
    #[test]
    fn accepts_only_the_fixture_proven_bookmark_schema() {
        let root = root();
        let path = source(&root);
        assert_eq!(identity(&path).paths, ["C:\\repos\\demo"]);
        fs::write(&path, "<ArrayOfBookmark><Bookmark><Name>x</Name><Path>C:\\repo</Path><Type>Git</Type></Bookmark></ArrayOfBookmark>").unwrap();
        assert!(snapshot(&path, "3.4.0".into()).is_err());
        fs::write(
            &path,
            "<ArrayOfBookmark><Bookmark><Name>x</Name><Path>C:\\repo</Path></Bookmark>",
        )
        .unwrap();
        assert!(snapshot(&path, "3.4.0".into()).is_err());
    }
    #[test]
    fn rejects_credentials_and_unsafe_bookmark_paths() {
        for value in [
            "https://user:password@example.test/repo",
            "https://user&#64;example.test/repo",
            "https://user&#x40;example.test/repo",
            "C:\\repo\\..\\secret",
            "C:\\repo:stream",
            "C:\\NUL\\repo",
        ] {
            let root = root();
            let path = source(&root);
            xml(&path, value);
            assert!(snapshot(&path, "3.4.0".into()).is_err(), "{value}");
        }
    }
    #[test]
    fn rejects_running_process_from_tasklist_output() {
        assert!(has_process("\"SourceTree.exe\",\"123\""));
        assert!(!has_process("\"Code.exe\",\"123\""));
    }
    #[test]
    fn rejects_a_bookmarks_reparse_point() {
        let root = root();
        let source = source(&root);
        let link = root.join("linked");
        #[cfg(windows)]
        let linked = std::process::Command::new("cmd")
            .args(["/c", "mklink", "/J"])
            .arg(&link)
            .arg(source.parent().unwrap())
            .status()
            .is_ok_and(|status| status.success());
        #[cfg(not(windows))]
        let linked = false;
        if linked {
            assert!(snapshot(&link.join("bookmarks.xml"), "3.4.0".into()).is_err());
        }
    }
    #[test]
    fn validates_inventory_hash_size_duplicate_entries_and_source_change() {
        let root = root();
        let source = source(&root);
        let identity = identity(&source);
        bundle(&source, &identity);
        let archive = root
            .join("source/out")
            .read_dir()
            .unwrap()
            .next()
            .unwrap()
            .unwrap()
            .path();
        assert!(inspect_at(&archive).is_ok());
        fs::write(&source, b"changed").unwrap();
        assert!(create_revalidated_at(&source, &identity, &root.join("changed-out")).is_err());
        let bad = root.join("bad.zip");
        let file = File::create(&bad).unwrap();
        let mut zip = ZipWriter::new(file);
        let opt = SimpleFileOptions::default();
        let manifest = Manifest {
            format_version: 1,
            kind: "codex-companion-personal-bundle".into(),
            created_at: "2026-09-07T00:00:00Z".into(),
            platform: "windows".into(),
            sensitive: true,
            artifacts: vec![Artifact {
                app_id: APP.into(),
                adapter_format_version: 1,
                source_app_version: "3.4.0".into(),
                kind: "bookmarks".into(),
                archive_path: ARTIFACT.into(),
                files: 1,
                bytes: 1,
                sha256: "0".repeat(64),
                restore_mode: "manual".into(),
                repository_paths: vec!["C:\\repos\\demo".into()],
            }],
        };
        zip.start_file(MANIFEST, opt).unwrap();
        zip.write_all(&serde_json::to_vec(&manifest).unwrap())
            .unwrap();
        zip.start_file(ARTIFACT, opt).unwrap();
        zip.write_all(b"x").unwrap();
        zip.start_file("APPS/SOURCETREE/BOOKMARKS.XML", opt)
            .unwrap();
        zip.write_all(b"x").unwrap();
        zip.finish().unwrap();
        assert!(inspect_at(&bad).is_err());
    }
    #[test]
    fn recovery_is_create_new_and_rolls_back() {
        let root = root();
        let source = source(&root);
        let identity = identity(&source);
        bundle(&source, &identity);
        let archive = root
            .join("source/out")
            .read_dir()
            .unwrap()
            .next()
            .unwrap()
            .unwrap()
            .path();
        let hash = hash_file(&archive).unwrap().0;
        let target = root.join("stage/bookmarks.xml");
        let destination = root.join("destination/bookmarks.xml");
        assert!(restore_at(&archive, &hash, &target, &destination, false, true).is_err());
        assert!(!target.exists());
        fs::create_dir_all(target.parent().unwrap()).unwrap();
        fs::write(&target, b"keep").unwrap();
        assert!(restore_at(&archive, &hash, &target, &destination, false, false).is_err());
        assert_eq!(fs::read(&target).unwrap(), b"keep");
    }
}
