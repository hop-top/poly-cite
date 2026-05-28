import { defaultPolicy, parse, type Policy, type URI, type VanityCandidate, vanityCandidates } from "./scheme";

export type Parser = (input: string) => URI;
export type Completer = (prefix: string) => string[] | Promise<string[]>;

export type TypeRegistration = {
  name: string;
  parser?: Parser;
  completer?: Completer;
};

export class Registry {
  private readonly typesByName = new Map<string, TypeRegistration>();

  constructor(private readonly policy: Policy = defaultPolicy) {}

  register(reg: TypeRegistration): void {
    if (!reg.name) throw new Error("cite: registration name is required");
    if (this.typesByName.has(reg.name)) throw new Error(`cite: type ${JSON.stringify(reg.name)} already registered`);
    this.typesByName.set(reg.name, reg);
  }

  parse(input: string): URI {
    const parsed = parse(input, this.policy);
    const reg = this.typesByName.get(parsed.scheme);
    if (!reg) throw new Error(`cite: unknown type ${JSON.stringify(parsed.scheme)}`);
    return reg.parser ? reg.parser(input) : parsed;
  }

  completeVanity(input: string): VanityCandidate[] {
    return vanityCandidates(input, this.policy);
  }

  async complete(typeName: string, prefix: string): Promise<string[]> {
    const reg = this.typesByName.get(typeName);
    if (!reg) throw new Error(`cite: unknown type ${JSON.stringify(typeName)}`);
    if (!reg.completer) return [];
    return await reg.completer(prefix);
  }

  types(): string[] {
    return [...this.typesByName.keys()].sort();
  }
}

export function newRegistry(policy: Policy = defaultPolicy): Registry {
  return new Registry(policy);
}
