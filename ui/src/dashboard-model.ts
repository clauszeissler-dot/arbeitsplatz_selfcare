import type { Level, Snapshot, StorageHealth } from "./api";

export type FocusTone = Level | "pause";

export interface FocusVisual {
  mode: "focus" | "pause";
  tone: FocusTone;
  label: string;
  value: number;
}

export function focusVisual(
  inBreak: boolean,
  blockMinutes: number,
  focusLevel: Level,
): FocusVisual {
  if (inBreak) return { mode: "pause", tone: "pause", label: "Pause", value: 0 };
  return { mode: "focus", tone: focusLevel, label: `${blockMinutes}′`, value: blockMinutes };
}

type DashboardSource = {
  snapshot: () => Promise<Snapshot>;
  storageHealth: () => Promise<StorageHealth>;
};

export type DashboardLoadResult =
  | { ok: true; snapshot: Snapshot; storageHealth: StorageHealth }
  | { ok: false; code: "dashboard-load" };

export async function loadDashboardData(source: DashboardSource): Promise<DashboardLoadResult> {
  try {
    const [snapshot, storageHealth] = await Promise.all([source.snapshot(), source.storageHealth()]);
    return { ok: true, snapshot, storageHealth };
  } catch {
    return { ok: false, code: "dashboard-load" };
  }
}

