import { describe, expect, it } from "vitest";
import { isWithinApprovedRoot, selectionMatches } from "./safety";

it("requires the exact preview selection, including IDs containing delimiters", () => {
  expect(selectionMatches(["a"], ["a"])).toBe(true);
  expect(selectionMatches(["a"], ["b"])).toBe(false);
  expect(selectionMatches(["a"], ["a", "b"])).toBe(false);
  expect(selectionMatches(["a\0b"], ["a", "b"])).toBe(false);
});

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
