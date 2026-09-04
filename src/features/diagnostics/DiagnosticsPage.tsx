import { PageHeader } from "../../components/PageHeader";
import { useTranslation } from "../../i18n";
import type { DiagnosticsSnapshot } from "../../types/codex";

interface DiagnosticsPageProps { diagnostics: DiagnosticsSnapshot | null; loading: boolean; error: string | null; onRefresh: () => Promise<void>; }

export function DiagnosticsPage({ diagnostics, loading, error, onRefresh }: DiagnosticsPageProps) {
  const { t } = useTranslation();
  const rows = diagnostics ? [
    [t("diagnostics.operatingSystem"), diagnostics.operatingSystem], [t("diagnostics.architecture"), diagnostics.architecture], [t("diagnostics.codexHome"), diagnostics.codexHome],
    [t("diagnostics.codexHomeStatus"), diagnostics.codexHomeExists ? t("common.available") : t("common.notFound")], [t("diagnostics.cli"), diagnostics.codexCliVersion ?? t("common.notFound")],
    [t("diagnostics.config"), diagnostics.configDir], [t("diagnostics.backupDir"), diagnostics.backupDir], [t("diagnostics.skills"), String(diagnostics.skillsCount)], [t("diagnostics.pets"), String(diagnostics.petsCount)]
  ] : [];
  return <>
    <PageHeader title={t("diagnostics.title")} description={t("diagnostics.description")} action={<button type="button" onClick={() => void onRefresh()} disabled={loading}>{loading ? t("common.refreshing") : t("common.refresh")}</button>} />
    {error && <p className="error-banner">{error}</p>}
    <section className="diagnostics-list" aria-busy={loading}>
      {rows.map(([label, value]) => <div key={label}><span>{label}</span><code>{value}</code></div>)}
      {!loading && !diagnostics && !error && <p>{t("diagnostics.none")}</p>}
    </section>
  </>;
}
