import { describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { desktopFilename, handlerID, snippet, validateHandlerSpec, type HandlerSpec } from "../src";

type Fixture = {
  cases: Array<{
    name: string;
    platform: string;
    spec: HandlerSpec;
    expected: { handlerId: string; desktopFilename?: string; renderedContains: string[] };
  }>;
  invalidCases: Array<{ name: string; spec: HandlerSpec }>;
};

const fixture = JSON.parse(readFileSync(join(__dirname, "../../spec/fixtures/handler-contract.json"), "utf8")) as Fixture;

describe("handler proto fixture", () => {
  for (const tc of fixture.cases) {
    it(tc.name, () => {
      expect(handlerID(tc.spec)).toBe(tc.expected.handlerId);
      if (tc.expected.desktopFilename) expect(desktopFilename(tc.spec)).toBe(tc.expected.desktopFilename);
      const rendered = snippet(tc.platform, tc.spec);
      for (const text of tc.expected.renderedContains) {
        expect(rendered).toContain(text);
      }
    });
  }

  for (const tc of fixture.invalidCases) {
    it(`rejects ${tc.name}`, () => {
      expect(() => validateHandlerSpec(tc.spec)).toThrow();
    });
  }
});
