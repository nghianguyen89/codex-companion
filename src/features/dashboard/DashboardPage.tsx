import { PageHeader } from "../../components/PageHeader";
import { useTranslation } from "../../i18n";
import type { DiagnosticsSnapshot } from "../../types/codex";

interface DashboardPageProps { diagnostics: DiagnosticsSnapshot | null; }

export function DashboardPage({ diagnostics }: DashboardPageProps) {
  const { t } = useTranslation();
  const cards = [
    [t("dashboard.environment"), diagnostics?.codexHomeExists ? t("dashboard.detected") : t("dashboard.notDetected")],
    [t("dashboard.skills"), diagnostics ? String(diagnostics.skillsCount) : t("common.dash")],
    [t("dashboard.pets"), diagnostics ? String(diagnostics.petsCount) : t("common.dash")],
    [t("dashboard.backups"), t("dashboard.comingSoon")]
  ];

  return (
    <>
      <PageHeader title={t("dashboard.title")} description={t("dashboard.description")} />
      <section className="summary-grid" aria-label={t("dashboard.summary")}>
        {cards.map(([label, value]) => <article className="summary-card" key={label}><span>{label}</span><strong>{value}</strong></article>)}
      </section>
      <section className="notice-card">
        <h2>{t("dashboard.safe")}</h2>
        <p>{t("dashboard.safeText")}</p>
      </section>
    </>
  );
}
