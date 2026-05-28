import type { Registry } from "./registry";

export type CompletionResult = {
  suggestions: string[];
};

export async function completeWithScheme(reg: Registry, typeName: string, toComplete: string): Promise<CompletionResult> {
  let prefix = toComplete;
  let scheme = "";

  const idx = toComplete.indexOf("://");
  if (idx >= 0) {
    scheme = toComplete.slice(0, idx);
    prefix = toComplete.slice(idx + 3);
  }

  if (scheme !== "") {
    const candidates = reg.completeVanity(toComplete);
    if (candidates.length > 1) {
      return { suggestions: candidates.map((candidate) => `${candidate.from}\tcanonical: ${candidate.to}`) };
    }
  }

  const suggestions = await reg.complete(typeName, prefix);
  if (!scheme) return { suggestions };
  return { suggestions: suggestions.map((s) => `${scheme}://${s}`) };
}
