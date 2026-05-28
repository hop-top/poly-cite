import { describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { AmbiguousVanityError, parse, resolveAction, type Policy, type URI } from "../src";

type Fixture = {
  namespacePolicy: {
    defaultNamespaceSegments: number;
    schemeNamespaceSegments: Record<string, number>;
  };
  validCases: Array<{ name: string; input: string; expected: Partial<URI>; canonical: string }>;
  invalidCases: Array<{ name: string; input: string }>;
  vanityCases: Array<{
    name: string;
    input: string;
    aliases: Array<{ from: string; to: string; prefix?: boolean; preserveSuffix?: boolean }>;
    options?: { strict?: boolean; jsonAmbiguity?: boolean };
    valid: boolean;
    expected?: Partial<URI>;
    canonical?: string;
  }>;
  actionCases: Array<{
    name: string;
    input: string;
    actionRoutes: Policy["actionRoutes"];
    expected: Partial<URI>;
    canonical: string;
    command: string;
    args: string[];
  }>;
};

const fixture = JSON.parse(readFileSync(join(__dirname, "../../spec/fixtures/cite-contract.json"), "utf8")) as Fixture;
const basePolicy: Policy = {
  defaultNamespaceSegments: fixture.namespacePolicy.defaultNamespaceSegments,
  schemeNamespaceSegments: fixture.namespacePolicy.schemeNamespaceSegments,
};

function expectURI(actual: URI, expected: Partial<URI>) {
  expect(actual).toMatchObject(expected);
}

describe("URI proto fixture", () => {
  for (const tc of fixture.validCases) {
    it(tc.name, () => {
      const got = parse(tc.input, basePolicy);
      expectURI(got, tc.expected);
      expect(got.canonical()).toBe(tc.canonical);
    });
  }

  for (const tc of fixture.invalidCases) {
    it(`rejects ${tc.name}`, () => {
      expect(() => parse(tc.input, basePolicy)).toThrow();
    });
  }

  for (const tc of fixture.vanityCases) {
    it(tc.name, () => {
      const policy = { ...basePolicy, vanityAliases: tc.aliases };
      if (!tc.valid) {
        expect(() => parse(tc.input, policy, tc.options)).toThrow();
        return;
      }
      const got = parse(tc.input, policy, tc.options);
      expectURI(got, tc.expected ?? {});
      expect(got.canonical()).toBe(tc.canonical);
    });
  }

  for (const tc of fixture.actionCases) {
    it(tc.name, () => {
      const policy = { ...basePolicy, actionRoutes: tc.actionRoutes };
      const got = parse(tc.input, policy);
      expectURI(got, tc.expected);
      expect(got.canonical()).toBe(tc.canonical);
      expect(resolveAction(got, policy)).toEqual({ action: got.action, command: tc.command, args: tc.args });
    });
  }

  it("rejects conflicting action query params", () => {
    expect(() => parse("tlc://org/repo/T-0001?action=task.claim&cmd=task&verb=close", basePolicy)).toThrow(
      "cite: conflicting action query parameters"
    );
  });

  it("can report ambiguous fuzzy vanity matches as JSON", () => {
    const policy = {
      ...basePolicy,
      vanityAliases: [
        { from: "task://shortcut-a", to: "task://hop-top/cite/T-0001" },
        { from: "task://shortcut-b", to: "task://hop-top/cite/T-0002" },
      ],
    };

    expect(() => parse("task://shortcut-c", policy, { jsonAmbiguity: true })).toThrow(AmbiguousVanityError);
    try {
      parse("task://shortcut-c", policy, { jsonAmbiguity: true });
    } catch (err) {
      const body = JSON.parse((err as Error).message) as { input: string; candidates: unknown[] };
      expect(body.input).toBe("task://shortcut-c");
      expect(body.candidates).toHaveLength(2);
    }
  });
});
