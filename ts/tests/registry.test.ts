import { describe, expect, it } from "vitest";
import { newRegistry } from "../src/registry";
import { completeWithScheme } from "../src/completions";

describe("Registry.complete", () => {
  it("returns suggestions for prefix", async () => {
    const r = newRegistry();
    r.register({
      name: "task",
      completer: (prefix) => ["T-0001", "T-0002", "T-0099"].filter((x) => x.startsWith(prefix)),
    });

    const out = await r.complete("task", "T-000");
    expect(out).toEqual(["T-0001", "T-0002"]);
  });

  it("preserves scheme", async () => {
    const r = newRegistry();
    r.register({ name: "task", completer: () => ["T-0001"] });

    const out = await completeWithScheme(r, "task", "task://T-");
    expect(out.suggestions).toEqual(["task://T-0001"]);
  });
});
