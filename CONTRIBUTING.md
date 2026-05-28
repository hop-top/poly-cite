# Contributing to cite

`cite` is a polyglot package. Changes that affect public behavior must
keep Go, TypeScript, Python, Rust, and PHP aligned.

## Getting Started

1. Fork the repository.
2. Clone your fork locally.
3. Create a feature branch: `git checkout -b feat/my-change`.
4. Make focused changes.
5. Run the relevant language tests.
6. Commit using Conventional Commits.
7. Push and open a pull request.

## Development Areas

| Area | Path |
| --- | --- |
| Go | `go/` |
| TypeScript | `ts/` |
| Python | `py/` |
| Rust | `rs/` |
| PHP | `php/` |
| Shared contracts | `spec/` |
| Product and engineering docs | `docs/` |

## Contract Changes

If a change affects parsing, canonicalization, handler identity, completion
behavior, or public API semantics:

- Update the shared fixture or spec first.
- Implement the behavior in every supported language.
- Add or update tests in each affected language.
- Update docs under `docs/`.

Do not let one language define behavior that the others cannot reproduce.

## Documentation Rules

All markdown files under `docs/` must use the required `.ops` frontmatter keys:

- `doc_type`
- `subtype`
- `status`
- `title`
- `summary`
- `owner`
- `created`
- `updated`
- `audience`
- `confidentiality`
- `tags`

Frontmatter formatting is strict:

- The opening `---` must be the first line of the file.
- There must be no blank line immediately after the opening `---`.
- There must be a blank line after the closing `---` before any text or header.

Use controlled values already present in the docs unless a new option is
explicitly added to the business/project documentation configuration.

## Code Style

- Follow existing conventions in each language directory.
- Keep files small and split code when behavior boundaries become unclear.
- Keep public facades stable and hide parser/handler dependencies behind
  replaceable adapters.
- Do not commit generated build artifacts, dependency caches, or local tool
  output.

## Tests

Run the relevant test suite before opening a pull request:

```sh
(cd go && go test ./...)
(cd ts && pnpm test)
(cd py && PYTHONPATH=src python3 -m pytest -q)
(cd rs && cargo test)
(cd php && vendor/bin/phpunit)
```

If a dependency is not installed locally, document what could not be run in the
pull request.

## Commit Messages

Use Conventional Commits:

```text
feat(scope): add new behavior
fix(scope): correct a bug
docs: update documentation
test(scope): add coverage
ci: update workflow
```

Use `feat:` for public behavior additions. Do not downgrade a user-visible
feature to `chore:` to avoid a release bump.

## Pull Requests

- Reference related issues, specs, stories, or ADRs.
- Keep PRs small and reviewable.
- Ensure CI passes before requesting review.
- Update docs when behavior, API, or workflow changes.

## Code of Conduct

Be respectful, specific, and constructive. Focus reviews on correctness,
maintainability, and cross-language parity.
