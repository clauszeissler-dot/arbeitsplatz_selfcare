import assert from "node:assert/strict";
import test from "node:test";

import { daySummaryText, focusVisual, formatElapsed, liveSeconds, loadDashboardData, runningClock, versionLabel } from "./dashboard-model.ts";

test("pause uses one calm cyan semantic instead of a focus warning color", () => {
  assert.deepEqual(focusVisual(true, 95, "red"), {
    mode: "pause",
    tone: "pause",
    label: "Pause",
    value: 0,
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
