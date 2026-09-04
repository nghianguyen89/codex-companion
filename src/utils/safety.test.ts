import { describe, expect, it } from "vitest";
import { isWithinApprovedRoot } from "./safety";

describe("isWithinApprovedRoot", () => {
  it("accepts a child path under its approved root", () => {
    expect(isWithinApprovedRoot("C:\\Users\\Nghia\\.codex\\skills\\sample", "C:\\Users\\Nghia\\.codex")).toBe(true);
  });

  it("rejects a sibling path with a matching prefix", () => {
    expect(isWithinApprovedRoot("C:\\Users\\Nghia\\.codex-other", "C:\\Users\\Nghia\\.codex")).toBe(false);
  });

  it("rejects a traversal-looking path until native canonicalization approves it", () => {
    expect(isWithinApprovedRoot("C:\\Users\\Nghia\\.codex\\skills\\..\\..\\outside", "C:\\Users\\Nghia\\.codex")).toBe(false);
  });
});
