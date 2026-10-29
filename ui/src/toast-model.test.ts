import assert from "node:assert/strict";
import test from "node:test";

import { selectNewestHint } from "./toast-model.ts";

const hint = (at: number, title: string) => ({ at, title });

test("toast startup recovers a native hint that was emitted before the listener existed", () => {
  assert.deepEqual(selectNewestHint(null, hint(10, "Augen kurz raus")), hint(10, "Augen kurz raus"));
});

test("a late bootstrap response cannot overwrite a newer live hint", () => {
  assert.deepEqual(selectNewestHint(hint(20, "Neu"), hint(10, "Alt")), hint(20, "Neu"));
});

