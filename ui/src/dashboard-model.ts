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


/** Laufzeit als m:ss, ab einer Stunde als h:mm:ss. */
export function formatElapsed(seconds: number): string {
  const total = Number.isFinite(seconds) ? Math.max(0, Math.floor(seconds)) : 0;
  const hours = Math.floor(total / 3600);
  const minutes = Math.floor((total % 3600) / 60);
  const secs = String(total % 60).padStart(2, "0");
  return hours > 0 ? `${hours}:${String(minutes).padStart(2, "0")}:${secs}` : `${minutes}:${secs}`;
}

export interface RunningClock {
  mode: "focus" | "pause";
  since: number;
}

/** Welche Uhr läuft gerade: Fokus seit dem letzten Pausenende, in der Pause seit Pausenbeginn. */
export function runningClock(
  s: Pick<Snapshot, "in_break" | "block_started_at" | "pause_started_at">,
): RunningClock {
  if (s.in_break) return { mode: "pause", since: s.pause_started_at ?? s.block_started_at };
  return { mode: "focus", since: s.block_started_at };
}

/** Gemeldete Sekunden plus die Zeit, die seit dem letzten Snapshot vergangen ist. */
export function liveSeconds(source: { reported: number; reportedAt: number }, nowSeconds: number): number {
  return source.reported + Math.max(0, nowSeconds - source.reportedAt);
}

/** Kurzfassung eines Tagesprotokolls für die Verlaufsliste. */
export function daySummaryText(
  day: { first_active: number | null; ended_at: number | null; tasks: unknown[]; pauses: unknown[] },
  clock: (epochSeconds: number) => string,
): string {
  const parts: string[] = [];
  if (day.first_active !== null && day.ended_at !== null) parts.push(`${clock(day.first_active)}–${clock(day.ended_at)}`);
  if (day.tasks.length) parts.push(`${day.tasks.length} ${day.tasks.length === 1 ? "Aufgabe" : "Aufgaben"}`);
  if (day.pauses.length) parts.push(`${day.pauses.length} ${day.pauses.length === 1 ? "Pause" : "Pausen"}`);
  return parts.length ? parts.join(" · ") : "keine Details";
}

export interface BuildInfo {
  version: string;
  build: string;
}

/** Version samt Build-Art: Release nur als Nummer, alles andere erkennbar als Testbuild. */
export function versionLabel(info: BuildInfo): string {
  if (info.build === "release") return info.version;
  const local = /^local-([0-9a-f]+)(-dirty)?$/.exec(info.build);
  if (local) return `${info.version} · lokaler Build (${local[1]}${local[2] ? ", mit Änderungen" : ""})`;
  if (info.build === "ci") return `${info.version} · Test-Build (CI)`;
  return `${info.version} · Entwicklungsbuild`;
}
