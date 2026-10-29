export function selectNewestHint<T extends { at: number }>(current: T | null, candidate: T | null): T | null {
  if (!candidate) return current;
  if (!current || candidate.at >= current.at) return candidate;
  return current;
}

