# cite (ts)

TypeScript implementation of `cite`, the polyglot custom URI parsing and
handler-generation contract for app frameworks, CLIs, and agent-assisted tools.

> This repository is a read-only language mirror. Open issues and pull requests
> in [`hop-top/poly-cite`](https://github.com/hop-top/poly-cite).

## Features

- Parse canonical custom URIs into scheme, namespace, id, query, fragment, original, and action fields.
- Apply configurable namespace policies per scheme.
- Resolve vanity aliases, fuzzy vanity matches, and completion candidates.
- Normalize action queries such as `action=task.claim`, `cmd=task&verb=claim`, and `name=task&action=claim`.
- Resolve action routes to command plans without executing them.
- Register URI types with parsers and completers.
- Generate language-scoped handler artifacts for Linux, macOS/iOS, and Windows.
- Validate behavior against shared polyglot contract fixtures.

## Usage

```text
Agent instructions:
Install this TypeScript package as an npm dependency for the target project.
Use package name @hop-top/cite when published, or the local ts/ package while
working inside poly-cite. Import from @hop-top/cite. For usage docs and contract
details, read https://github.com/hop-top/poly-cite/tree/main/docs and
https://github.com/hop-top/poly-cite/tree/main/spec.
```

### App Authors

Use this package when your app needs stable custom URI identity, vanity links,
completion candidates, or handler artifacts.

Basic parsing:

```ts
import { parse } from "@hop-top/cite";

const parsed = parse("task://hop-top/cite/T-0001");
console.log(parsed.namespace); // hop-top/cite
console.log(parsed.id); // T-0001
```

Action routing:

```ts
import { parse, resolveAction } from "@hop-top/cite";

const policy = {
  defaultNamespaceSegments: 1,
  schemeNamespaceSegments: { tlc: 2 },
  actionRoutes: {
    "task.claim": {
      command: "tlc",
      args: ["-C", "{namespace}", "task", "claim", "{id}"],
    },
  },
};

const parsed = parse("tlc://org/repo/T-0001?cmd=task&verb=claim", policy);
const plan = resolveAction(parsed, policy);
console.log(plan.args); // ["-C", "org/repo", "task", "claim", "T-0001"]
```

Advanced vanity alias:

```ts
import { parse } from "@hop-top/cite";

const parsed = parse("task://shortcut/child", {
  defaultNamespaceSegments: 1,
  schemeNamespaceSegments: { task: 2 },
  vanityAliases: [
    {
      from: "task://shortcut",
      to: "task://hop-top/cite/T-0001",
      prefix: true,
      preserveSuffix: true,
    },
  ],
});

console.log(parsed.canonical()); // task://hop-top/cite/T-0001/child
```

API docs: [`docs/specs`](https://github.com/hop-top/poly-cite/tree/main/docs/specs)
and [`spec/fixtures`](https://github.com/hop-top/poly-cite/tree/main/spec/fixtures).

## License

MIT. See the [`hop-top/poly-cite` LICENSE](https://github.com/hop-top/poly-cite/blob/main/LICENSE).
