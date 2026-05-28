const fs = require("node:fs");
const path = require("node:path");
const cite = require(path.join(__dirname, "../../../ts/dist"));

const fixtureDir = process.argv[2] || "spec/fixtures";
const citeFixture = readJSON("cite-contract.json");
const handlerFixture = readJSON("handler-contract.json");
const basePolicy = {
  defaultNamespaceSegments: citeFixture.namespacePolicy.defaultNamespaceSegments,
  schemeNamespaceSegments: citeFixture.namespacePolicy.schemeNamespaceSegments,
};

const output = {
  version: 1,
  uri: {
    valid: citeFixture.validCases.map((tc) => parseCase(tc.name, tc.input, basePolicy)),
    invalid: citeFixture.invalidCases.map((tc) => invalidParseCase(tc.name, tc.input, basePolicy)),
    vanity: citeFixture.vanityCases.map((tc) => {
      const policy = { ...basePolicy, vanityAliases: tc.aliases || [] };
      return citeResult(tc.name, () => cite.parse(tc.input, policy, tc.options || {}));
    }),
    action: citeFixture.actionCases.map((tc) => {
      const policy = { ...basePolicy, actionRoutes: tc.actionRoutes || {} };
      const result = citeResult(tc.name, () => cite.parse(tc.input, policy));
      if (result.ok) result.action = cite.resolveAction(cite.parse(tc.input, policy), policy);
      return result;
    }),
  },
  handler: {
    valid: handlerFixture.cases.map((tc) => handlerResult(tc)),
    invalid: handlerFixture.invalidCases.map((tc) => invalidHandlerResult(tc)),
  },
};

process.stdout.write(JSON.stringify(output) + "\n");

function readJSON(name) {
  return JSON.parse(fs.readFileSync(path.join(fixtureDir, name), "utf8"));
}

function parseCase(name, input, policy) {
  return citeResult(name, () => cite.parse(input, policy));
}

function invalidParseCase(name, input, policy) {
  try {
    cite.parse(input, policy);
    return { name, ok: false };
  } catch (_) {
    return { name, ok: true };
  }
}

function citeResult(name, fn) {
  try {
    const got = fn();
    return {
      name,
      ok: true,
      uri: {
        scheme: got.scheme,
        namespace: got.namespace,
        id: got.id,
        query: got.query,
        fragment: got.fragment,
        original: got.original,
        action: got.action,
        canonical: got.canonical(),
        vanity: got.vanity(),
      },
    };
  } catch (_) {
    return { name, ok: false };
  }
}

function handlerResult(tc) {
  try {
    return {
      name: tc.name,
      ok: true,
      handlerId: cite.handlerID(tc.spec),
      desktopFilename: tc.platform === "linux" ? cite.desktopFilename(tc.spec) : "",
      rendered: cite.snippet(tc.platform, tc.spec),
    };
  } catch (_) {
    return { name: tc.name, ok: false };
  }
}

function invalidHandlerResult(tc) {
  try {
    cite.validateHandlerSpec(tc.spec);
    return { name: tc.name, ok: false, handlerId: "", desktopFilename: "", rendered: "" };
  } catch (_) {
    return { name: tc.name, ok: true, handlerId: "", desktopFilename: "", rendered: "" };
  }
}
