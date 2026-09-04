import { useCallback, useState } from "react";
import { BackupPage } from "../features/backup/BackupPage";
import { ConversationsPage } from "../features/conversations/ConversationsPage";
import { DashboardPage } from "../features/dashboard/DashboardPage";
import { DiagnosticsPage } from "../features/diagnostics/DiagnosticsPage";
import { PetsPage } from "../features/pets/PetsPage";
import { SettingsPage } from "../features/settings/SettingsPage";
import { SkillsPage } from "../features/skills/SkillsPage";
import { useAsyncValue } from "../hooks/useAsyncValue";
import { getConfiguration, getDiagnostics, saveConfiguration } from "../services/tauri";
import type { AppConfiguration } from "../types/codex";
import { I18nProvider, translate, type TranslationKey } from "../i18n";

type Page = "dashboard" | "conversations" | "backup" | "skills" | "pets" | "diagnostics" | "settings";
const navigation: Array<{ id: Page; label: TranslationKey; group: TranslationKey }> = [
  { id: "dashboard", label: "nav.dashboard", group: "nav.overview" }, { id: "conversations", label: "nav.conversations", group: "nav.manage" },
  { id: "backup", label: "nav.backup", group: "nav.manage" }, { id: "skills", label: "nav.skills", group: "nav.codex" },
  { id: "pets", label: "nav.pets", group: "nav.codex" }, { id: "diagnostics", label: "nav.diagnostics", group: "nav.tools" }, { id: "settings", label: "nav.settings", group: "nav.preferences" }
];

export function App() {
  const [page, setPage] = useState<Page>("dashboard");
  const diagnostics = useAsyncValue(getDiagnostics);
  const configuration = useAsyncValue(getConfiguration);
  const { setValue: setConfiguration } = configuration;
  const save = useCallback(async (next: AppConfiguration) => { await saveConfiguration(next); setConfiguration(next); }, [setConfiguration]);
  const language = configuration.value?.language ?? "en";
  const t = (key: TranslationKey) => translate(language, key);
  const groups = [...new Set(navigation.map((item) => item.group))];

  return <I18nProvider value={{ language, t }}><main className="app-shell">
    <aside className="sidebar">
      <div className="brand"><span className="brand-mark">C</span><div><strong>Codex</strong><span>Companion</span></div></div>
      <nav aria-label={t("nav.primary")}>{groups.map((group) => <section key={group}><p>{t(group)}</p>{navigation.filter((item) => item.group === group).map((item) => <button key={item.id} className={page === item.id ? "active" : ""} type="button" onClick={() => setPage(item.id)}>{t(item.label)}</button>)}</section>)}</nav>
      <footer><span className="status-dot" /> {t("nav.localFoundation")}</footer>
    </aside>
    <section className="content">
      {page === "dashboard" && <DashboardPage diagnostics={diagnostics.value} />}
      {page === "conversations" && <ConversationsPage />}
      {page === "backup" && <BackupPage />}
      {page === "skills" && <SkillsPage />}
      {page === "pets" && <PetsPage />}
      {page === "diagnostics" && <DiagnosticsPage diagnostics={diagnostics.value} loading={diagnostics.loading} error={diagnostics.error} onRefresh={diagnostics.refresh} />}
      {page === "settings" && <SettingsPage configuration={configuration.value} onSave={save} />}
    </section>
  </main></I18nProvider>;
}
