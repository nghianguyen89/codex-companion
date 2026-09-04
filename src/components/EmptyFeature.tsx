interface EmptyFeatureProps {
  title: string;
  description: string;
  nextStep: string;
}

export function EmptyFeature({ title, description, nextStep }: EmptyFeatureProps) {
  const { t } = useTranslation();
  return (
    <section className="empty-feature" aria-label={t("empty.status", { title })}>
      <span className="status-dot" />
      <p className="eyebrow">{t("empty.milestone")}</p>
      <h2>{title}</h2>
      <p>{description}</p>
      <p className="next-step">{t("empty.next", { step: nextStep })}</p>
    </section>
  );
}
import { useTranslation } from "../i18n";
