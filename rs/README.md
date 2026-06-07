# cite (rs)

Rust implementation of `cite`, the polyglot custom URI parsing and
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
- Generate language-scoped handler artifacts for Linux, macOS/iOS, and Windows
  ([guide](https://github.com/hop-top/poly-cite/blob/main/docs/guides/registering-a-scheme.md)).
- Validate behavior against shared polyglot contract fixtures.

## Usage

```text
Agent instructions:
Install this Rust crate as `hop-top-cite` from crates.io when published, or use
the local rs/ crate while working inside poly-cite. Import it as `hop_top_cite`.
For usage docs and contract details, read
https://github.com/hop-top/poly-cite/tree/main/docs and
https://github.com/hop-top/poly-cite/tree/main/spec.
```

### App Authors

Use this crate when your app needs stable custom URI identity, vanity links,
completion candidates, or handler artifacts.

Basic parsing:

```rust
use hop_top_cite::parse;

let parsed = parse("task://hop-top/cite/T-0001").unwrap();
assert_eq!(parsed.namespace, "hop-top/cite");
assert_eq!(parsed.id, "T-0001");
```

Action routing:

```rust
use std::collections::HashMap;
use hop_top_cite::{parse_with_policy, ActionRoute, Policy};

let mut policy = Policy::default();
policy.scheme_namespace_segments.insert("tlc".into(), 2);
policy.action_routes.insert(
    "task.claim".into(),
    ActionRoute {
        command: "tlc".into(),
        args: vec!["-C".into(), "{namespace}".into(), "task".into(), "claim".into(), "{id}".into()],
    },
);

let parsed = parse_with_policy("tlc://org/repo/T-0001?cmd=task&verb=claim", &policy).unwrap();
let plan = policy.resolve_action(&parsed).unwrap();
assert_eq!(plan.args, vec!["-C", "org/repo", "task", "claim", "T-0001"]);
```

Advanced vanity alias:

```rust
use hop_top_cite::{parse_with_policy, Policy, VanityAlias};

let mut policy = Policy::default();
policy.scheme_namespace_segments.insert("task".into(), 2);
policy.vanity_aliases.push(VanityAlias {
    from: "task://shortcut".into(),
    to: "task://hop-top/cite/T-0001".into(),
    prefix: true,
    preserve_suffix: true,
});

let parsed = parse_with_policy("task://shortcut/child", &policy).unwrap();
assert_eq!(parsed.canonical(), "task://hop-top/cite/T-0001/child");
```

API docs: [`docs/specs`](https://github.com/hop-top/poly-cite/tree/main/docs/specs)
and [`spec/fixtures`](https://github.com/hop-top/poly-cite/tree/main/spec/fixtures).

## License

MIT. See the [`hop-top/poly-cite` LICENSE](https://github.com/hop-top/poly-cite/blob/main/LICENSE).
