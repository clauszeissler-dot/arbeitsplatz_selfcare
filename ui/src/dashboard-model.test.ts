import assert from "node:assert/strict";
import test from "node:test";

import { daySummaryText, developmentRows, focusVisual, formatElapsed, liveSeconds, loadDashboardData, postureTraffic, runningClock, sinceFact, versionLabel } from "./dashboard-model.ts";

test("pause uses one calm cyan semantic instead of a focus warning color", () => {
  assert.deepEqual(focusVisual(true, 95, "red", 1000), {
    mode: "pause",
    tone: "pause",
    label: "Pause",
    value: 0,
    clockFrom: 1000,
  });
});

test("focus color uses the engine level derived from configured thresholds", () => {
  assert.equal(focusVisual(false, 45, "green").tone, "green");
  assert.equal(focusVisual(false, 60, "yellow").tone, "yellow");
  assert.equal(focusVisual(false, 90, "red").tone, "red");
});

test("a failed initial invoke becomes a recoverable state instead of an empty dashboard", async () => {
  const result = await loadDashboardData({
    snapshot: async () => { throw new Error("transient IPC failure"); },
    storageHealth: async () => ({ ok: true, message: null }),
  });

  assert.equal(result.ok, false);
  if (!result.ok) assert.equal(result.code, "dashboard-load");
});


test("elapsed time reads m:ss below an hour and h:mm:ss from one hour on", () => {
  assert.equal(formatElapsed(0), "0:00");
  assert.equal(formatElapsed(65), "1:05");
  assert.equal(formatElapsed(3599), "59:59");
  assert.equal(formatElapsed(3600), "1:00:00");
  assert.equal(formatElapsed(7385), "2:03:05");
});

test("elapsed time never goes negative when the clock is slightly behind the engine", () => {
  assert.equal(formatElapsed(-4), "0:00");
  assert.equal(formatElapsed(Number.NaN), "0:00");
});

test("the running clock follows focus, and switches to the pause start during a pause", () => {
  const base = { in_break: false, block_started_at: 1000, pause_started_at: null as number | null };
  assert.deepEqual(runningClock(base), { mode: "focus", since: 1000 });
  assert.deepEqual(runningClock({ ...base, in_break: true, pause_started_at: 1900 }), { mode: "pause", since: 1900 });
});

test("a pause without a known start falls back to the focus start instead of a bogus clock", () => {
  assert.deepEqual(runningClock({ in_break: true, block_started_at: 1000, pause_started_at: null }), {
    mode: "pause",
    since: 1000,
  });
});

test("live durations add the time since the last snapshot to the reported seconds", () => {
  assert.equal(liveSeconds({ reported: 600, reportedAt: 1000 }, 1090), 690);
  assert.equal(liveSeconds({ reported: 600, reportedAt: 1000 }, 900), 600);
});

test("a day log summary names time span, tasks and pauses", () => {
  const text = daySummaryText({ first_active: 1000, ended_at: 4600, tasks: [1, 2], pauses: [1] }, (t) => `t${t}`);
  assert.equal(text, "t1000–t4600 · 2 Aufgaben · 1 Pause");
  assert.equal(daySummaryText({ first_active: null, ended_at: null, tasks: [], pauses: [] }, (t) => `t${t}`), "keine Details");
});

test("a release shows only its version number", () => {
  assert.equal(versionLabel({ version: "0.3.0", build: "release" }), "0.3.0");
});

test("local, CI and development builds are never mistaken for a release", () => {
  assert.equal(versionLabel({ version: "0.3.0", build: "local-ce807fb" }), "0.3.0 · lokaler Build (ce807fb)");
  assert.equal(versionLabel({ version: "0.3.0", build: "local-ce807fb-dirty" }), "0.3.0 · lokaler Build (ce807fb, mit Änderungen)");
  assert.equal(versionLabel({ version: "0.3.0", build: "ci" }), "0.3.0 · Test-Build (CI)");
  assert.equal(versionLabel({ version: "0.3.0", build: "dev" }), "0.3.0 · Entwicklungsbuild");
  assert.equal(versionLabel({ version: "0.3.0", build: "irgendwas" }), "0.3.0 · Entwicklungsbuild");
});

// ---------- A-031/A-032: Fokus-Karte ----------

test("focus mode has no running pause clock in the ring", () => {
  assert.equal(focusVisual(false, 30, "green", 1000).clockFrom, null);
});

test("the facts list names the start time of the running focus or pause", () => {
  assert.deepEqual(sinceFact({ mode: "focus", since: 10 }), { label: "Fokus seit", since: 10 });
  assert.deepEqual(sinceFact({ mode: "pause", since: 20 }), { label: "Pause seit", since: 20 });
});

// ---------- A-033: Haltungsampel ----------

const POSTURES = [
  { id: "office_chair", label: "Bürostuhl", duration_min: 30 },
  { id: "swopper", label: "Swopper", duration_min: 30 },
  { id: "standing", label: "Stehen", duration_min: 15 },
];

test("only the office chair after its limit is red, unused postures are orange", () => {
  const t = postureTraffic({ office_chair: 163 }, POSTURES);
  assert.equal(t.level, "red");
  assert.deepEqual(t.items.map((i) => [i.id, i.level, i.used]), [
    ["office_chair", "red", true],
    ["swopper", "yellow", false],
    ["standing", "yellow", false],
  ]);
});

test("a fresh day on the chair is not red yet", () => {
  assert.equal(postureTraffic({ office_chair: 10 }, POSTURES).level, "yellow");
  assert.equal(postureTraffic({}, POSTURES).level, "yellow");
});

test("a posture turns green once its configured duration is reached today", () => {
  const t = postureTraffic({ office_chair: 90, swopper: 30, standing: 5 }, POSTURES);
  assert.equal(t.items.find((i) => i.id === "swopper")!.level, "green");
  assert.equal(t.items.find((i) => i.id === "standing")!.level, "yellow");
  assert.equal(t.items.find((i) => i.id === "standing")!.used, true);
});

test("sitting is green when at least a third of the day was spent changing posture", () => {
  const t = postureTraffic({ office_chair: 60, swopper: 20, standing: 15 }, POSTURES);
  assert.equal(t.changePercent, 37);
  assert.equal(t.level, "green");
  assert.equal(t.items[0].percent, 63);
});

// ---------- A-034: Entwicklung über die letzten Tage ----------

const day = (d: string, over: Partial<Parameters<typeof developmentRows>[0][number]> = {}) => ({
  day: d, active_min: 300, long_breaks: 2, eyes_offered: null, eyes_done: null, ...over,
});

test("development rates the last completed days and skips today", () => {
  const rows = developmentRows(
    [
      day("2026-10-28", { active_min: 600, long_breaks: 1 }),
      day("2026-10-29", { active_min: 620, long_breaks: 1 }),
      day("2026-10-30", { active_min: 356, long_breaks: 3 }),
      day("2026-11-02", { active_min: 325, long_breaks: 2 }),
      day("2026-11-03", { active_min: 900, long_breaks: 0 }),
    ],
    "2026-11-03",
    480,
  );
  const lotse = rows.find((r) => r.key === "long_breaks")!;
  assert.deepEqual([lotse.met, lotse.days], [2, 4]);
  assert.equal(lotse.trend, "up");
  const end = rows.find((r) => r.key === "workday")!;
  assert.deepEqual([end.met, end.days, end.trend], [2, 4, "up"]);
  assert.equal(end.level, "yellow");
});

test("eye breaks without recorded days are shown as not yet recorded instead of failed", () => {
  const eyes = developmentRows([day("2026-11-02")], "2026-11-03", 480).find((r) => r.key === "eyes")!;
  assert.equal(eyes.days, 0);
  assert.equal(eyes.level, null);
  assert.equal(eyes.trend, null);
});

test("eye breaks count a day as kept when at least half of the offered breaks were done", () => {
  const eyes = developmentRows(
    [
      day("2026-11-01", { eyes_offered: 10, eyes_done: 2 }),
      day("2026-11-02", { eyes_offered: 8, eyes_done: 4 }),
      day("2026-11-03", { eyes_offered: 0, eyes_done: 0 }),
    ],
    "2026-11-04",
    480,
  ).find((r) => r.key === "eyes")!;
  assert.deepEqual([eyes.met, eyes.days, eyes.level, eyes.trend], [1, 2, "yellow", "up"]);
});

test("very short days do not count for or against a habit", () => {
  const rows = developmentRows([day("2026-11-02", { active_min: 10, long_breaks: 0 })], "2026-11-03", 480);
  assert.equal(rows.find((r) => r.key === "long_breaks")!.days, 0);
});

test("the pilot rule asks for one long break between every two hours of work", () => {
  const rows = (active: number, long: number) =>
    developmentRows([day("2026-11-02", { active_min: active, long_breaks: long })], "2026-11-03", 480).find((r) => r.key === "long_breaks")!.met;
  assert.equal(rows(240, 1), 1, "4 h Arbeit brauchen eine lange Pause dazwischen");
  assert.equal(rows(241, 1), 0);
  assert.equal(rows(110, 0), 1);
});
