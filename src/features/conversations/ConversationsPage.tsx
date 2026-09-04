import { useCallback, useMemo, useState } from "react";
import { PageHeader } from "../../components/PageHeader";
import { useTranslation, type TranslationKey } from "../../i18n";
import { useAsyncValue } from "../../hooks/useAsyncValue";
import { discoverConversations } from "../../services/tauri";
import type { ConversationDiscoveryStatus, ConversationSummary } from "../../types/codex";

type SortOrder = "updated" | "created" | "title" | "project";
const statusKeys: Record<Exclude<ConversationDiscoveryStatus, "ready">, TranslationKey> = { codexHomeMissing: "conversations.unavailableHome", sessionDirectoryMissing: "conversations.unavailableDirectory", permissionDenied: "conversations.unavailablePermission", filesystemUnavailable: "conversations.unavailableFilesystem" };
const compareText = (left: string | null, right: string | null) => (left ?? "").localeCompare(right ?? "");

export function ConversationsPage() {
  const { t, language } = useTranslation();
  const discovery = useAsyncValue(useCallback(() => discoverConversations(), []));
  const [query, setQuery] = useState("");
  const [project, setProject] = useState("all");
  const [sort, setSort] = useState<SortOrder>("updated");
  const projects = useMemo(() => [...new Set((discovery.value?.conversations ?? []).map((item) => item.projectPath).filter((value): value is string => Boolean(value)))].sort(), [discovery.value]);
  const conversations = useMemo(() => [...(discovery.value?.conversations ?? [])].filter((item) => project === "all" || item.projectPath === project).filter((item) => !query.trim() || [item.title, item.projectName, item.projectPath, item.id].some((value) => value?.toLocaleLowerCase().includes(query.trim().toLocaleLowerCase()))).sort((left, right) => sort === "title" ? compareText(left.title, right.title) : sort === "project" ? compareText(left.projectName, right.projectName) : compareText(right[sort === "updated" ? "updatedAt" : "createdAt"], left[sort === "updated" ? "updatedAt" : "createdAt"])), [discovery.value, project, query, sort]);
  const formatDate = (value: string | null) => !value || Number.isNaN(new Date(value).valueOf()) ? t("common.dash") : new Intl.DateTimeFormat(language, { dateStyle: "medium", timeStyle: "short" }).format(new Date(value));
  return <>
    <PageHeader title={t("conversations.title")} description={t("conversations.description")} action={<button type="button" onClick={() => void discovery.refresh()} disabled={discovery.loading}>{discovery.loading ? t("common.refreshing") : t("common.refresh")}</button>} />
    {discovery.error && <p className="error-banner">{t("conversations.discoveryError", { error: discovery.error })}</p>}
    {discovery.value && discovery.value.status !== "ready" && <section className="notice-card"><h2>{t("conversations.unavailableTitle")}</h2><p>{t(statusKeys[discovery.value.status])}</p></section>}
    {discovery.value?.status === "ready" && <>
      <div className="summary-grid conversation-summary"><article className="summary-card"><span>{t("conversations.found")}</span><strong>{discovery.value.totalDiscovered}</strong></article><article className="summary-card"><span>{t("conversations.ready")}</span><strong>{discovery.value.successfullyParsed}</strong></article><article className="summary-card"><span>{t("conversations.skipped")}</span><strong>{discovery.value.skipped}</strong></article><article className="summary-card"><span>{t("conversations.unsupported")}</span><strong>{discovery.value.unsupported}</strong></article></div>
      <section className="conversation-controls" aria-label={t("conversations.filters")}><label>{t("conversations.search")}<input type="search" value={query} onChange={(event) => setQuery(event.target.value)} placeholder={t("conversations.searchPlaceholder")} /></label><label>{t("conversations.project")}<select value={project} onChange={(event) => setProject(event.target.value)}><option value="all">{t("conversations.allProjects")}</option>{projects.map((path) => <option key={path} value={path}>{path}</option>)}</select></label><label>{t("conversations.sort")}<select value={sort} onChange={(event) => setSort(event.target.value as SortOrder)}><option value="updated">{t("conversations.updated")}</option><option value="created">{t("conversations.created")}</option><option value="title">{t("conversations.titleColumn")}</option><option value="project">{t("conversations.project")}</option></select></label></section>
      {conversations.length === 0 ? <section className="notice-card"><h2>{discovery.value.totalDiscovered === 0 ? t("conversations.noSessions") : t("conversations.noMatching")}</h2><p>{discovery.value.totalDiscovered === 0 ? t("conversations.noSessionsDescription") : t("conversations.noMatchingDescription")}</p></section> : <section className="conversation-table-wrap"><table className="conversation-table"><thead><tr><th>{t("conversations.titleColumn")}</th><th>{t("conversations.project")}</th><th>{t("conversations.updated")}</th><th>{t("conversations.created")}</th><th>{t("conversations.source")}</th></tr></thead><tbody>{conversations.map((conversation) => <ConversationRow key={conversation.id} conversation={conversation} formatDate={formatDate} />)}</tbody></table></section>}
    </>}
  </>;
}

function ConversationRow({ conversation, formatDate }: { conversation: ConversationSummary; formatDate: (value: string | null) => string }) {
  const { t } = useTranslation();
  return <tr><td><strong>{conversation.title || t("conversations.untitled")}</strong><code>{conversation.id}</code></td><td><span>{conversation.projectName || t("common.dash")}</span>{conversation.projectPath && <code>{conversation.projectPath}</code>}</td><td>{formatDate(conversation.updatedAt)}</td><td>{formatDate(conversation.createdAt)}</td><td>{conversation.source || t("common.dash")}</td></tr>;
}
