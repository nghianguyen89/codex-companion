/**
 * Ensures future filesystem operations can only target a path below an approved root.
 * Paths are normalized as slash-separated strings because the Rust implementation will
 * perform the authoritative filesystem validation before every native operation.
 */
export function isWithinApprovedRoot(target: string, root: string): boolean {
  const normalize = (path: string) => path.replace(/\\/g, "/").replace(/\/+$/, "").toLowerCase();
  const normalizedTarget = normalize(target);
  const normalizedRoot = normalize(root);
  if (normalizedTarget.split("/").includes("..") || normalizedRoot.split("/").includes("..")) return false;
  return normalizedTarget === normalizedRoot || normalizedTarget.startsWith(`${normalizedRoot}/`);
}

export function selectionMatches(preview: readonly string[], selected: readonly string[]): boolean {
  return preview.length === selected.length && preview.every((id, index) => id === selected[index]);
}
