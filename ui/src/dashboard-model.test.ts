import assert from "node:assert/strict";
import test from "node:test";

import { focusVisual, loadDashboardData } from "./dashboard-model.ts";

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

