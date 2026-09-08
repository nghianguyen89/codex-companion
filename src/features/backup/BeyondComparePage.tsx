import { useState } from "react";
import { useTranslation } from "../../i18n";
import { createBeyondCompareBundle, getBeyondCompareReadiness, inspectBeyondCompareBundle, previewBeyondCompare, previewBeyondCompareRecovery, recoverBeyondCompare } from "../../services/tauri";
import type { BeyondCompareBundleInspection, BeyondCompareBundlePreview, BeyondCompareReadiness, BeyondCompareRecoveryPreview } from "../../types/codex";
import { PageHeader } from "../../components/PageHeader";

export function BeyondComparePage() {
  const { t } = useTranslation();
  const [ready, setReady] = useState<BeyondCompareReadiness | null>(null); const [credentialsIncluded, setCredentialsIncluded] = useState(false); const [acknowledged, setAcknowledged] = useState(false); const [busy, setBusy] = useState(false); const [preview, setPreview] = useState<BeyondCompareBundlePreview | null>(null); const [inspection, setInspection] = useState<BeyondCompareBundleInspection | null>(null); const [recovery, setRecovery] = useState<BeyondCompareRecoveryPreview | null>(null); const [confirmation, setConfirmation] = useState(""); const [result, setResult] = useState(""); const [error, setError] = useState("");
  const run = async (action: () => Promise<void>) => { setBusy(true); setError(""); try { await action(); } catch { setError(t("beyondCompare.error")); } finally { setBusy(false); } };
  return <>
    <PageHeader title={t("beyondCompare.title")} description={t("beyondCompare.description")} />
    <section className="notice-card"><h2>{t("beyondCompare.sensitiveTitle")}</h2><p>{t("beyondCompare.sensitive")}</p></section>
    <section className="backup-preview"><h2>{t("beyondCompare.readiness")}</h2><p>{ready ? (ready.supported ? t("beyondCompare.ready") : t("beyondCompare.unsupported")) : t("beyondCompare.readinessDescription")}</p><button disabled={busy} onClick={() => void run(async () => setReady(await getBeyondCompareReadiness()))}>{t("beyondCompare.check")}</button></section>
    <section className="backup-preview"><h2>{t("beyondCompare.export")}</h2><p>{t("beyondCompare.exportDescription")}</p><label className="component-option"><input type="checkbox" disabled={busy} checked={credentialsIncluded} onChange={(event) => { setCredentialsIncluded(event.target.checked); setAcknowledged(false); setPreview(null); }} />{t("beyondCompare.credentialsIncluded")}</label><label className="component-option"><input type="checkbox" disabled={busy} checked={acknowledged} onChange={(event) => { setAcknowledged(event.target.checked); setPreview(null); }} />{t("beyondCompare.acknowledgement")}</label><button disabled={busy || !acknowledged} onClick={() => void run(async () => setPreview(await previewBeyondCompare(credentialsIncluded)))}>{t("beyondCompare.choosePackage")}</button>
      {preview && <div><p>{preview.packageName} · {preview.bytes} {t("beyondCompare.bytes")}</p><button disabled={busy} onClick={() => void run(async () => { setResult((await createBeyondCompareBundle(preview.token)).bundleName); setPreview(null); })}>{t("beyondCompare.create")}</button></div>}
    </section>
    <section className="backup-preview"><h2>{t("beyondCompare.recover")}</h2><p>{t("beyondCompare.recoverDescription")}</p><button disabled={busy} onClick={() => void run(async () => { setInspection(await inspectBeyondCompareBundle()); setRecovery(null); setConfirmation(""); })}>{t("beyondCompare.inspect")}</button>
      {inspection && <div><p>{inspection.bundleName} · {inspection.bytes} {t("beyondCompare.bytes")} · SHA-256 ✓</p><button disabled={busy} onClick={() => void run(async () => { setRecovery(await previewBeyondCompareRecovery(inspection.token)); setConfirmation(""); })}>{t("beyondCompare.previewRecovery")}</button></div>}
      {recovery && <div><p>{recovery.packageName} · {recovery.bytes} {t("beyondCompare.bytes")}</p><p><code>{recovery.stagingPath}</code></p><label className="delete-confirmation">{t("beyondCompare.confirm")}<input disabled={busy} value={confirmation} onChange={(event) => setConfirmation(event.target.value)} /></label><button disabled={busy || confirmation !== "RECOVER"} onClick={() => void run(async () => { const output = await recoverBeyondCompare(recovery.token, confirmation); setResult(`${t("beyondCompare.recovered")} ${output.stagingPath}`); setRecovery(null); setConfirmation(""); })}>{t("beyondCompare.recover")}</button></div>}
    </section>
    {result && <section className="success-card"><h2>{t("beyondCompare.completed")}</h2><p>{result}</p><p>{t("beyondCompare.manualImport")}</p></section>}
    {error && <p className="error-banner">{error}</p>}
  </>;
}
