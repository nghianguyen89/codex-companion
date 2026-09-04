import { EmptyFeature } from "../../components/EmptyFeature";
import { PageHeader } from "../../components/PageHeader";
import { useTranslation } from "../../i18n";

export function SkillsPage() {
  const { t } = useTranslation();
  return <><PageHeader title={t("skills.title")} description={t("skills.description")} /><EmptyFeature title={t("skills.planned")} description={t("skills.detail")} nextStep={t("skills.next")} /></>;
}
