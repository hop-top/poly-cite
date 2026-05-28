export class URI {
  constructor(
    readonly scheme: string,
    readonly namespace: string,
    readonly id: string,
    readonly query = "",
    readonly fragment = "",
    readonly original = "",
    readonly action = ""
  ) {}

  canonical(): string {
    const query = this.query ? `?${this.query}` : "";
    const fragment = this.fragment ? `#${this.fragment}` : "";
    return `${this.scheme}://${this.namespace}/${this.id}${query}${fragment}`;
  }

  vanity(): string {
    return this.original || this.canonical();
  }

  toString(): string {
    return this.canonical();
  }
}

export type VanityAlias = {
  from: string;
  to: string;
  prefix?: boolean;
  preserveSuffix?: boolean;
};

export type ActionRoute = {
  command: string;
  args?: string[];
};

export type ResolvedAction = {
  action: string;
  command: string;
  args: string[];
};

export type Policy = {
  defaultNamespaceSegments?: number;
  schemeNamespaceSegments?: Record<string, number>;
  vanityAliases?: VanityAlias[];
  actionRoutes?: Record<string, ActionRoute>;
};

export type ParseOptions = {
  strict?: boolean;
  jsonAmbiguity?: boolean;
};

export type VanityCandidate = {
  from: string;
  to: string;
  distance: number;
};

export class AmbiguousVanityError extends Error {
  readonly input: string;
  readonly candidates: VanityCandidate[];

  constructor(input: string, candidates: VanityCandidate[], asJSON = false) {
    super(asJSON ? JSON.stringify({ input, candidates }) : `cite: ambiguous vanity alias ${JSON.stringify(input)}: ${candidates.map((c) => c.from).join(", ")}`);
    this.name = "AmbiguousVanityError";
    this.input = input;
    this.candidates = candidates;
  }

  toJSON(): { input: string; candidates: VanityCandidate[] } {
    return { input: this.input, candidates: this.candidates };
  }
}

export const defaultPolicy: Policy = {
  defaultNamespaceSegments: 1,
  schemeNamespaceSegments: {
    task: 2,
    doc: 2,
    repo: 1,
    tlc: 2,
    "task-dev": 2,
    "task-stress": 2,
  },
};

export const DefaultPolicy = defaultPolicy;

export function parse(input: string, policy: Policy = defaultPolicy, options: ParseOptions = {}): URI {
  if (input === "") throw new Error("cite: empty input");

  const resolved = resolveVanity(input, policy, options);
  const parsed = parseURL(resolved.parseInput);
  const namespaceSegmentCount = namespaceSegments(policy, parsed.scheme);
  if (namespaceSegmentCount <= 0) throw new Error("cite: namespace segment count must be positive");
  if (parsed.segments.length <= namespaceSegmentCount) throw new Error("cite: id is required");

  const namespace = parsed.segments.slice(0, namespaceSegmentCount).join("/");
  const id = parsed.segments.slice(namespaceSegmentCount).join("/");
  if (namespace === "") throw new Error("cite: namespace is required");
  if (id === "") throw new Error("cite: id is required");

  return new URI(parsed.scheme, namespace, id, parsed.query, parsed.fragment, resolved.vanity, actionFromQuery(parsed.query));
}

export function resolveAction(uri: URI, policy: Policy = defaultPolicy): ResolvedAction {
  if (!uri) throw new Error("cite: nil URI");
  if (!uri.action) throw new Error("cite: action is required");
  const route = policy.actionRoutes?.[uri.action];
  if (!route) throw new Error(`cite: unknown action ${JSON.stringify(uri.action)}`);
  if (!route.command) throw new Error("cite: action route command is required");

  return {
    action: uri.action,
    command: expandActionTemplate(route.command, uri),
    args: (route.args ?? []).map((arg) => expandActionTemplate(arg, uri)),
  };
}

export function vanityCandidates(input: string, policy: Policy = defaultPolicy): VanityCandidate[] {
  return (policy.vanityAliases ?? [])
    .map((alias) => ({ from: alias.from, to: alias.to, distance: levenshtein(input, alias.from) }))
    .filter((candidate) => withinFuzzyThreshold(input, candidate.from, candidate.distance))
    .sort((a, b) => (a.distance === b.distance ? a.from.localeCompare(b.from) : a.distance - b.distance));
}

function parseURL(input: string): { scheme: string; segments: string[]; query: string; fragment: string } {
  const schemeSep = input.indexOf("://");
  if (schemeSep <= 0) throw new Error("cite: scheme is required");

  const scheme = input.slice(0, schemeSep);
  let rest = input.slice(schemeSep + 3);
  let fragment = "";
  const hash = rest.indexOf("#");
  if (hash >= 0) {
    fragment = rest.slice(hash + 1);
    rest = rest.slice(0, hash);
  }

  let query = "";
  const queryIdx = rest.indexOf("?");
  if (queryIdx >= 0) {
    query = rest.slice(queryIdx + 1);
    rest = rest.slice(0, queryIdx);
  }

  const rawSegments = rest.split("/");
  const host = rawSegments.shift() ?? "";
  if (host === "") throw new Error("cite: namespace is required");

  const segments = [host];
  for (const segment of rawSegments) {
    if (segment === "") continue;
    try {
      segments.push(decodeURIComponent(segment));
    } catch (cause) {
      throw new Error(`cite: invalid path segment: ${(cause as Error).message}`);
    }
  }

  return { scheme, segments, query, fragment };
}

function actionFromQuery(rawQuery: string): string {
  let values: URLSearchParams;
  try {
    values = new URLSearchParams(rawQuery);
  } catch (cause) {
    throw new Error(`cite: invalid query: ${(cause as Error).message}`);
  }

  const candidates: string[] = [];
  const action = values.get("action") ?? "";
  const name = values.get("name") ?? "";
  if (action !== "" && name === "") candidates.push(action);

  const cmd = values.get("cmd") ?? "";
  const verb = values.get("verb") ?? "";
  if (cmd !== "" || verb !== "") {
    if (cmd === "" || verb === "") throw new Error("cite: cmd and verb must be provided together");
    candidates.push(`${cmd}.${verb}`);
  }

  if (name !== "") {
    if (action === "") throw new Error("cite: name and action must be provided together");
    candidates.push(`${name}.${action}`);
  }

  for (const candidate of candidates.slice(1)) {
    if (candidate !== candidates[0]) throw new Error("cite: conflicting action query parameters");
  }
  return candidates[0] ?? "";
}

function namespaceSegments(policy: Policy, scheme: string): number {
  const specific = policy.schemeNamespaceSegments?.[scheme];
  if (specific !== undefined) return specific;
  return policy.defaultNamespaceSegments && policy.defaultNamespaceSegments !== 0 ? policy.defaultNamespaceSegments : 1;
}

function resolveVanity(input: string, policy: Policy, options: ParseOptions): { parseInput: string; vanity: string } {
  let best: VanityAlias | undefined;
  let bestLen = -1;
  for (const candidate of policy.vanityAliases ?? []) {
    if (!candidate.from || !candidate.to) throw new Error("cite: vanity alias from and to are required");
    const matched = input === candidate.from || (candidate.prefix === true && input.startsWith(`${candidate.from}/`));
    if (matched && candidate.from.length > bestLen) {
      best = candidate;
      bestLen = candidate.from.length;
    }
  }

  if (!best) {
    if (!options.strict) {
      const fuzzy = closestVanity(input, policy, options);
      if (fuzzy) return fuzzy;
    }
    return { parseInput: input, vanity: "" };
  }

  let target = best.to;
  if (best.prefix && best.preserveSuffix && input.length > best.from.length) {
    target = best.to.replace(/\/+$/, "") + input.slice(best.from.length);
  }
  return { parseInput: target, vanity: input };
}

function closestVanity(input: string, policy: Policy, options: ParseOptions): { parseInput: string; vanity: string } | undefined {
  const candidates = vanityCandidates(input, policy);
  if (candidates.length === 0) return undefined;

  const bestDistance = candidates[0].distance;
  const best = candidates.filter((candidate) => candidate.distance === bestDistance);
  if (best.length > 1) throw new AmbiguousVanityError(input, best, options.jsonAmbiguity);
  return { parseInput: best[0].to, vanity: input };
}

function withinFuzzyThreshold(input: string, candidate: string, distance: number): boolean {
  const longest = Math.max([...input].length, [...candidate].length);
  const threshold = Math.min(8, Math.max(2, Math.floor(longest / 5)));
  return distance <= threshold;
}

function levenshtein(a: string, b: string): number {
  const ar = [...a];
  const br = [...b];
  if (ar.length === 0) return br.length;
  if (br.length === 0) return ar.length;

  let prev = Array.from({ length: br.length + 1 }, (_, i) => i);
  let curr = new Array<number>(br.length + 1).fill(0);
  for (let i = 0; i < ar.length; i++) {
    curr[0] = i + 1;
    for (let j = 0; j < br.length; j++) {
      const cost = ar[i] === br[j] ? 0 : 1;
      curr[j + 1] = Math.min(curr[j] + 1, prev[j + 1] + 1, prev[j] + cost);
    }
    [prev, curr] = [curr, prev];
  }
  return prev[br.length];
}

function expandActionTemplate(value: string, uri: URI): string {
  return value
    .replaceAll("{scheme}", uri.scheme)
    .replaceAll("{namespace}", uri.namespace)
    .replaceAll("{id}", uri.id)
    .replaceAll("{query}", uri.query)
    .replaceAll("{fragment}", uri.fragment);
}
