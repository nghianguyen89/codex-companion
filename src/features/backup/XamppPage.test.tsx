import { expect, it } from "vitest";
import { renderToStaticMarkup } from "react-dom/server";
import { XamppPage } from "./XamppPage";

it("shows XAMPP exclusions, stopped-process requirement, and manual-only staging", () => {
  const html = renderToStaticMarkup(<XamppPage />);
  expect(html).toContain("Apache, MariaDB, and XAMPP-related processes must be stopped");
  expect(html).toContain("binaries, MariaDB data directories, credentials, keys, logs, caches, repository metadata, symbolic links");
  expect(html).toContain("trusted encrypted transport");
  expect(html).toContain("manual placement");
  expect(html).not.toContain("Import XAMPP automatically");
});
