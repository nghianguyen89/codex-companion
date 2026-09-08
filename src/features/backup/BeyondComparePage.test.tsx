import { expect, it } from "vitest";
import { renderToStaticMarkup } from "react-dom/server";
import { BeyondComparePage } from "./BeyondComparePage";

it("shows the explicit sensitive Beyond Compare workflow without automatic import", () => {
  const html = renderToStaticMarkup(<BeyondComparePage />);
  expect(html).toContain("Beyond Compare");
  expect(html).toContain("saved passwords and FTP/SSH connection credentials");
  expect(html).toContain("not password-protected");
  expect(html).toContain("trusted encrypted transport");
  expect(html).toContain("Tools &gt; Import Settings");
  expect(html).not.toContain("Import automatically");
});
