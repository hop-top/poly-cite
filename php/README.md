# cite (php)

PHP implementation of `cite`, the polyglot custom URI parsing and
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
Install this PHP library with Composer when published, or use the local php/
package while working inside poly-cite. Import classes from the HopTop\Cite
namespace. For usage docs and contract details, read
https://github.com/hop-top/poly-cite/tree/main/docs and
https://github.com/hop-top/poly-cite/tree/main/spec.
```

### App Authors

Use this package when your app needs stable custom URI identity, vanity links,
completion candidates, or handler artifacts.

Basic parsing:

```php
<?php

use HopTop\Cite\Scheme;

$parsed = Scheme::parse("task://hop-top/cite/T-0001");
echo $parsed->namespace; // hop-top/cite
echo $parsed->id; // T-0001
```

Action routing:

```php
<?php

use HopTop\Cite\ActionRoute;
use HopTop\Cite\Policy;
use HopTop\Cite\Scheme;

$policy = new Policy(
    defaultNamespaceSegments: 1,
    schemeNamespaceSegments: ["tlc" => 2],
    actionRoutes: [
        "task.claim" => new ActionRoute(
            command: "tlc",
            args: ["-C", "{namespace}", "task", "claim", "{id}"],
        ),
    ],
);

$parsed = Scheme::parse("tlc://org/repo/T-0001?name=task&action=claim", $policy);
$plan = $policy->resolveAction($parsed);
print_r($plan->args); // ["-C", "org/repo", "task", "claim", "T-0001"]
```

Advanced vanity alias:

```php
<?php

use HopTop\Cite\Policy;
use HopTop\Cite\Scheme;
use HopTop\Cite\VanityAlias;

$policy = new Policy(
    defaultNamespaceSegments: 1,
    schemeNamespaceSegments: ["task" => 2],
    vanityAliases: [
        new VanityAlias(
            from: "task://shortcut",
            to: "task://hop-top/cite/T-0001",
            prefix: true,
            preserveSuffix: true,
        ),
    ],
);

$parsed = Scheme::parse("task://shortcut/child", $policy);
echo $parsed->canonical(); // task://hop-top/cite/T-0001/child
```

API docs: [`docs/specs`](https://github.com/hop-top/poly-cite/tree/main/docs/specs)
and [`spec/fixtures`](https://github.com/hop-top/poly-cite/tree/main/spec/fixtures).

## License

MIT. See the [`hop-top/poly-cite` LICENSE](https://github.com/hop-top/poly-cite/blob/main/LICENSE).
