import assert from "node:assert/strict";
import test from "node:test";

import { updatePresentation, type UpdateViewState } from "./updater-model.ts";

test("idle and current states offer an explicit update check", () => {
  assert.deepEqual(updatePresentation({ phase: "idle", currentVersion: "0.1.0" }), {
    status: "Installiert: Version 0.1.0",
    action: "check",
    actionLabel: "Nach Updates suchen",
    disabled: false,
  });
  assert.equal(
    updatePresentation({ phase: "current", currentVersion: "0.2.0" }).status,
    "Version 0.2.0 ist aktuell.",
  );
});

test("an available release is installed only by a separate explicit action", () => {
  const state: UpdateViewState = {
    phase: "available",
    currentVersion: "0.1.0",
    version: "0.2.0",
    notes: "Sicherheits- und Stabilitätsupdate",
  };
  assert.deepEqual(updatePresentation(state), {
    status: "Version 0.2.0 ist verfügbar. Sicherheits- und Stabilitätsupdate",
    action: "install",
    actionLabel: "Version 0.2.0 installieren",
    disabled: false,
  });
});

test("download progress is bounded and raw updater errors are not rendered", () => {
  assert.equal(
    updatePresentation({
      phase: "downloading",
      currentVersion: "0.1.0",
      version: "0.2.0",
      progress: 140,
    }).status,
    "Update wird geladen … 100 %",
  );
  assert.deepEqual(
    updatePresentation({ phase: "error", currentVersion: "0.1.0" }),
    {
      status: "Update konnte nicht geprüft oder installiert werden.",
      action: "check",
      actionLabel: "Erneut versuchen",
      disabled: false,
    },
  );
});
