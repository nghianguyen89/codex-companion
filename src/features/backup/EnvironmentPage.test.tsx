import { expect, it } from "vitest";
import { renderToStaticMarkup } from "react-dom/server";
import { EnvironmentPage } from "./EnvironmentPage";
import { CleanupPage } from "../cleanup/CleanupPage";

it("shows the migration limitation and offers no restore action before inspection", () => {
  const html = renderToStaticMarkup(<EnvironmentPage />);
  expect(html).toContain("Desktop chat merging is NOT implemented or verified");
  expect(html).toContain("Inspect environment ZIP");
  expect(html).not.toContain("Type RESTORE");
  expect(html).not.toContain("Restore new files</button>");
});
it("does not offer cache deletion before an explicit scan and selection", () => {
  const html = renderToStaticMarkup(<CleanupPage />);
  expect(html).toContain("Scan</button>");
  expect(html).not.toContain("Clean selected cache</button>");
  expect(html).toContain("recovery archives are protected");
});
