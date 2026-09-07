import { expect, it } from "vitest";
import { renderToStaticMarkup } from "react-dom/server";
import { SourceTreePage } from "./SourceTreePage";

it("shows the explicit SourceTree exclusions and manual-only recovery", () => {
  const html = renderToStaticMarkup(<SourceTreePage />);
  expect(html).toContain("SourceTree");
  expect(html).toContain("bookmarks.xml");
  expect(html).toContain("Repositories, tabs, custom actions, user.config, hosted accounts, credential files, license keys, and secrets");
  expect(html).toContain("trusted encrypted transport");
  expect(html).toContain("manual only");
  expect(html).not.toContain("Apply SourceTree settings automatically");
});
