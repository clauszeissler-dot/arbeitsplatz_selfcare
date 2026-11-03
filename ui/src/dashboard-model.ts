import type { Level, PostureDefinition, Snapshot, StorageHealth } from "./api";

export type FocusTone = Level | "pause";

export interface FocusVisual {
  mode: "focus" | "pause";
  tone: FocusTone;
  label: string;
  value: number;
  /** In der Pause: Startzeit der sekündlich laufenden Pausenuhr im Ring. */
  clockFrom: number | null;
}

export function focusVisual(
  inBreak: boolean,
  blockMinutes: number,
  focusLevel: Level,
  pauseStartedAt: number | null = null,
): FocusVisual {
  if (inBreak) return { mode: "pause", tone: "pause", label: "Pause", value: 0, clockFrom: pauseStartedAt };
  return { mode: "focus", tone: focusLevel, label: `${blockMinutes}′`, value: blockMinutes, clockFrom: null };
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

/** Zeile in der Fokus-Karte: seit wann Fokus bzw. Pause läuft. */
export function sinceFact(running: RunningClock): { label: string; since: number } {
  return { label: running.mode === "pause" ? "Pause seit" : "Fokus seit", since: running.since };
}

// ---------- Haltungsampel (A-033) ----------

/** Anteil Haltungswechsel, ab dem der Tag als ausreichend gilt (Sitz-Steh-Rhythmus 30/15 ≈ ein Drittel). */
export const POSTURE_CHANGE_TARGET = 33;
/** Darunter gilt der Tag als „nur Bürostuhl“ – rot, sobald das Sitzintervall überschritten ist. */
export const POSTURE_ONLY_CHAIR_BELOW = 10;

export interface PostureLight {
  id: string;
  label: string;
  minutes: number;
  percent: number;
  used: boolean;
  level: Level;
}

/** Ampel zur Haltungsverteilung des Tages: Sitzanteil gegen Wechsel, je Haltung genutzt/ausreichend. */
export function postureTraffic(
  share: Record<string, number>,
  postures: PostureDefinition[],
  chairId = "office_chair",
): { level: Level; changePercent: number; items: PostureLight[] } {
  const total = postures.reduce((sum, posture) => sum + (share[posture.id] ?? 0), 0);
  const chair = postures.find((posture) => posture.id === chairId) ?? postures[0];
  const chairMin = chair ? share[chair.id] ?? 0 : 0;
  const percent = (minutes: number) => (total > 0 ? Math.round((minutes / total) * 100) : 0);
  const changePercent = percent(total - chairMin);
  const level: Level =
    total > 0 && changePercent >= POSTURE_CHANGE_TARGET
      ? "green"
      : chair && chairMin > chair.duration_min && changePercent < POSTURE_ONLY_CHAIR_BELOW
        ? "red"
        : "yellow";
  const items = postures.map((posture) => {
    const minutes = share[posture.id] ?? 0;
    const used = minutes > 0;
    const own: Level = !used ? "yellow" : minutes >= posture.duration_min ? "green" : "yellow";
    return { id: posture.id, label: posture.label, minutes, percent: percent(minutes), used, level: posture === chair ? level : own };
  });
  return { level, changePercent, items };
}

// ---------- Entwicklung über die letzten Tage (A-034) ----------

export interface DevelopmentDay {
  day: string;
  active_min: number;
  long_breaks: number;
  eyes_offered?: number | null;
  eyes_done?: number | null;
}

export type Trend = "up" | "down" | "flat";

export interface DevelopmentRow {
  key: "eyes" | "long_breaks" | "workday";
  label: string;
  rule: string;
  met: number;
  days: number;
  level: Level | null;
  trend: Trend | null;
}

/** Tage unter dieser Aktivität zählen weder für noch gegen eine Gewohnheit. */
export const DEVELOPMENT_MIN_ACTIVE = 60;

function rate(results: boolean[]): number {
  return results.length ? results.filter(Boolean).length / results.length : 0;
}

/** Konsequenz je Gewohnheit über abgeschlossene Tage (ohne heute), mit Trend ältere gegen jüngere Hälfte. */
export function developmentRows(history: DevelopmentDay[], today: string, workdayMin: number, window = 7): DevelopmentRow[] {
  const days = history
    .filter((d) => d.day !== today && d.active_min >= DEVELOPMENT_MIN_ACTIVE)
    .sort((a, b) => a.day.localeCompare(b.day))
    .slice(-window);
  const build = (key: DevelopmentRow["key"], label: string, rule: string, check: (d: DevelopmentDay) => boolean | null): DevelopmentRow => {
    const results = days.map(check).filter((r): r is boolean => r !== null);
    const half = Math.floor(results.length / 2);
    const older = rate(results.slice(0, half));
    const newer = rate(results.slice(results.length - half));
    const r = rate(results);
    return {
      key,
      label,
      rule,
      met: results.filter(Boolean).length,
      days: results.length,
      level: results.length ? (r >= 0.8 ? "green" : r >= 0.5 ? "yellow" : "red") : null,
      trend: half ? (newer > older ? "up" : newer < older ? "down" : "flat") : null,
    };
  };
  return [
    build("eyes", "Augenpausen", "mind. die Hälfte der 20-Sekunden-Pausen gemacht", (d) =>
      d.eyes_offered ? (d.eyes_done ?? 0) * 2 >= d.eyes_offered : null),
    build("long_breaks", "Lotsenregel", "zwischen je 2 h Arbeit eine lange Pause", (d) => d.long_breaks >= Math.floor(Math.max(0, d.active_min - 1) / 120)),
    build("workday", "Feierabend", "aktive Zeit im Rahmen des Arbeitstags", (d) => d.active_min <= workdayMin),
  ];
}
