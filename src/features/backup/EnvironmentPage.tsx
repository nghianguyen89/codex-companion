import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { PageHeader } from "../../components/PageHeader";
import { useTranslation } from "../../i18n";

interface Entry { path: string; group: string; bytes: number; manual: boolean }
interface Group { id: string; files: number; bytes: number; reason: string }
interface Preview { token: string; groups: Group[]; excluded: Group[]; entries: Entry[] }
interface RestorePreview { token: string; items: Array<{ path: string; status: string; bytes: number }> }
const size = (bytes: number) => `${(bytes / 1024 / 1024).toFixed(2)} MiB`;

export function EnvironmentPage() {
  const { t } = useTranslation();
  const [groups, setGroups] = useState(["chat", "settings", "skills", "pets"]);
  const [preview, setPreview] = useState<Preview | null>(null);
  const [archive, setArchive] = useState<Preview | null>(null);
  const [restore, setRestore] = useState<RestorePreview | null>(null);
  const [confirmation, setConfirmation] = useState("");
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState("");
  const [result, setResult] = useState("");
  const run = async (action: () => Promise<void>) => { setBusy(true); setError(""); setResult(""); try { await action(); } catch (e) { setError(typeof e === "string" ? e : JSON.stringify(e)); } finally { setBusy(false); } };
  return <>
    <PageHeader title={t("environment.title")} description={t("environment.description")} />
    <p className="notice-card">{t("environment.boundary")}</p>
    <fieldset disabled={busy} className="backup-preview"><legend>{t("environment.components")}</legend>
      {(["chat", "settings", "skills", "pets"] as const).map(g => <label key={g} className="component-option"><input type="checkbox" checked={groups.includes(g)} onChange={() => { setGroups(current => current.includes(g) ? current.filter(v => v !== g) : [...current, g]); setPreview(null); setResult(""); }} />{t(`environment.${g}`)}</label>)}
      <button disabled={!groups.length} onClick={() => void run(async () => { setPreview(null); setPreview(await invoke<Preview>("preview_environment", { groups })); })}>{t("environment.preview")}</button>
    </fieldset>
    {preview && <section className="backup-preview"><h2>{t("environment.selected")}</h2>
      {[...preview.groups, ...preview.excluded].map(g => <p key={g.id}><strong>{g.id}</strong> · {g.files} {t("environment.files")} · {size(g.bytes)}<br />{g.reason}</p>)}
      <details><summary>{t("environment.fileList")}</summary><ul className="safe-session-list">{preview.entries.map(e => <li key={e.path}><code>{e.path}</code> · {size(e.bytes)}</li>)}</ul></details>
      <button disabled={busy || !preview.entries.length} onClick={() => void run(async () => { const r = await invoke<{ archivePath: string; archiveBytes: number; files: number }>("create_environment", { token: preview.token }); setResult(`${t("environment.verified")}: ${r.archivePath} · ZIP ${size(r.archiveBytes)} · ${r.files} ${t("environment.files")}`); setPreview(null); })}>{t("environment.create")}</button>
    </section>}
    <section className="backup-preview"><h2>{t("environment.recovery")}</h2>
      <button disabled={busy} onClick={() => void run(async () => { setRestore(null); setConfirmation(""); setArchive(null); setArchive(await invoke<Preview | null>("inspect_environment")); })}>{t("environment.inspect")}</button>
      {archive && <><p>{archive.entries.length} {t("environment.files")} · SHA-256 ✓</p><button disabled={busy} onClick={() => void run(async () => { setRestore(null); setConfirmation(""); setRestore(await invoke<RestorePreview>("preview_environment_restore", { token: archive.token })); })}>{t("environment.previewRestore")}</button></>}
      {restore && <><div className="conversation-table-wrap"><table className="conversation-table"><thead><tr><th>{t("environment.fileList")}</th><th>{t("environment.status")}</th><th>MiB</th></tr></thead><tbody>{restore.items.map(i => <tr key={i.path}><td><code>{i.path}</code></td><td>{i.status}</td><td>{size(i.bytes)}</td></tr>)}</tbody></table></div>
        <p>{t("environment.statusHelp")}</p><label>{t("environment.confirm")}<input disabled={busy} value={confirmation} onChange={e => setConfirmation(e.target.value)} /></label>
        <button disabled={busy || confirmation !== "RESTORE"} onClick={() => void run(async () => { const r = await invoke<{ restored: number; skipped: number; errors: string[]; rollbackRemaining: number }>("restore_environment", { token: restore.token, confirmation }); setResult(`${t("environment.result")}: ${r.restored} / ${r.skipped}; ${t("environment.rollback")}: ${r.rollbackRemaining}`); setError(r.errors.join("\n")); setRestore(null); setConfirmation(""); })}>{t("environment.restore")}</button></>}
    </section>
    {busy && <p role="status">{t("common.working")}</p>}{error && <p role="alert" className="error-banner">{error}</p>}{result && <p role="status" className="notice-card">{result}</p>}
  </>;
}
