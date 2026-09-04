/* eslint-disable react-refresh/only-export-components -- this module intentionally exports the provider and typed translation API together. */
import { createContext, useContext } from "react";
import { en } from "./en";
import { vi } from "./vi";
export type Language = "en" | "vi";
export type TranslationKey = { [S in keyof typeof en]: `${S & string}.${keyof (typeof en)[S] & string}` }[keyof typeof en];
const dictionaries = { en, vi } as const;
export type TranslationValues = Record<string, string | number>;
export function translate(language: Language | undefined, key: TranslationKey, values?: TranslationValues): string {
  const [section, item] = key.split(".") as [keyof typeof en, string];
  const selected = dictionaries[language ?? "en"] ?? en;
  const localized = selected[section] as Record<string, string> | undefined;
  const fallback = en[section] as Record<string, string> | undefined;
  const message = localized?.[item] ?? fallback?.[item] ?? key;
  return message.replace(/\{(\w+)\}/g, (token, name) => values?.[name]?.toString() ?? token);
}
const I18nContext = createContext({ language: "en" as Language, t: (key: TranslationKey, values?: TranslationValues) => translate("en", key, values) });
export const I18nProvider = I18nContext.Provider;
export const useTranslation = () => useContext(I18nContext);
