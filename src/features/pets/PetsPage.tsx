import { EmptyFeature } from "../../components/EmptyFeature";
import { PageHeader } from "../../components/PageHeader";
import { useTranslation } from "../../i18n";

export function PetsPage() {
  const { t } = useTranslation();
  return <><PageHeader title={t("pets.title")} description={t("pets.description")} /><EmptyFeature title={t("pets.planned")} description={t("pets.detail")} nextStep={t("pets.next")} /></>;
}
