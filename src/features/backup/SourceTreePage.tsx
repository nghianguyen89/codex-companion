import { useState } from "react";
import { PageHeader } from "../../components/PageHeader";
import { useTranslation } from "../../i18n";
import { createSourceTreeBundle, getSourceTreeReadiness, inspectSourceTreeBundle, previewSourceTree, previewSourceTreeRecovery, recoverSourceTree } from "../../services/tauri";
import type { SourceTreeInspection, SourceTreePreview, SourceTreeReadiness, SourceTreeRecoveryPreview } from "../../types/codex";

export function SourceTreePage() {
  const { t } = useTranslation();
  const [ready, setReady] = useState<SourceTreeReadiness | null>(null); const [preview, setPreview] = useState<SourceTreePreview | null>(null); const [inspection, setInspection] = useState<SourceTreeInspection | null>(null); const [recovery, setRecovery] = useState<SourceTreeRecoveryPreview | null>(null); const [confirmation, setConfirmation] = useState(""); const [busy, setBusy] = useState(false); const [result, setResult] = useState(""); const [error, setError] = useState("");
  const run = async (action: () => Promise<void>) => { setBusy(true); setError(""); try { await action(); } catch { setError(t("sourceTree.error")); } finally { setBusy(false); } };
  return <>
    <PageHeader title={t("sourceTree.title")} description={t("sourceTree.description")} />
    <section className="notice-card"><h2>{t("sourceTree.boundaryTitle")}</h2><p>{t("sourceTree.boundary")}</p></section>
    <section className="backup-preview"><h2>{t("sourceTree.readiness")}</h2><p>{ready ? (ready.bookmarksFound ? t("sourceTree.ready") : t("sourceTree.notFound")) : t("sourceTree.readinessDescription")}</p><button disabled={busy} onClick={() => void run(async () => setReady(await getSourceTreeReadiness()))}>{t("sourceTree.check")}</button></section>
    <section className="backup-preview"><h2>{t("sourceTree.export")}</h2><p>{t("sourceTree.exportDescription")}</p><button disabled={busy} onClick={() => void run(async () => setPreview(await previewSourceTree()))}>{t("sourceTree.preview")}</button>
      {preview && <div><p>{t("sourceTree.version")} {preview.sourceAppVersion} · {preview.bookmarkCount} {t("sourceTree.bookmarks")} · {preview.bytes} {t("sourceTree.bytes")}</p><p>{t("sourceTree.machinePaths")}</p><button disabled={busy} onClick={() => void run(async () => { setResult((await createSourceTreeBundle(preview.token)).bundleName); setPreview(null); })}>{t("sourceTree.create")}</button></div>}
    </section>
    <section className="backup-preview"><h2>{t("sourceTree.recover")}</h2><p>{t("sourceTree.recoverDescription")}</p><button disabled={busy} onClick={() => void run(async () => { setInspection(await inspectSourceTreeBundle()); setRecovery(null); setConfirmation(""); })}>{t("sourceTree.inspect")}</button>
      {inspection && <div><p>{inspection.bundleName} · {inspection.bookmarkCount} {t("sourceTree.bookmarks")} · SHA-256 ✓</p><button disabled={busy} onClick={() => void run(async () => { setRecovery(await previewSourceTreeRecovery(inspection.token)); setConfirmation(""); })}>{t("sourceTree.previewRecovery")}</button></div>}
      {recovery && <div><p>{recovery.destinationConflict ? t("sourceTree.destinationConflict") : t("sourceTree.destinationAbsent")}</p><p><code>{recovery.stagingPath}</code></p><label className="delete-confirmation">{t("sourceTree.confirm")}<input disabled={busy} value={confirmation} onChange={(event) => setConfirmation(event.target.value)} /></label><button disabled={busy || confirmation !== "RECOVER"} onClick={() => void run(async () => { const output = await recoverSourceTree(recovery.token, confirmation); setResult(output.stagingPath); setRecovery(null); setConfirmation(""); })}>{t("sourceTree.recover")}</button></div>}
    </section>
    {result && <section className="success-card"><h2>{t("sourceTree.completed")}</h2><p><code>{result}</code></p><p>{t("sourceTree.manual")}</p></section>}
    {error && <p className="error-banner">{error}</p>}
  </>;
}
