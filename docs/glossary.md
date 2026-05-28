---
doc_type: kb
subtype: glossary
status: draft
title: cite glossary
summary: Controlled terms for URI scheme parsing, handler identity, and polyglot parity.
owner: Idea Crafters Labs
created: 2026-05-13
updated: 2026-05-13
audience:
  - developers
  - tool_authors
  - oss_contributors
  - ai_assistant
confidentiality: public
tags:
  - cite
  - glossary
  - terminology
---

# cite Glossary

## Scheme

The URI protocol label before `://`. It selects the URI type and OS routing
family, for example `task`, `doc`, or `repo`.

## Namespace

The collision domain inside a scheme. For `task://hop-top/cite/T-0001`,
the namespace is `hop-top/cite`.

## Namespace Policy

The rule that says how many leading path segments belong to the namespace for a
scheme.

## Namespace Segments

The number of leading URI path segments assigned to the namespace.

## ID

The object identifier inside a namespace. It may contain slashes when the
scheme's namespace policy leaves multiple remaining path segments.

## Query Metadata

The URI query string without `?`. Used for schema, version, mode, channel, or
handler hints when they should not change OS routing.

## Fragment

The URI fragment without `#`. Used for intra-object anchors such as a document
section.

## Handler

The app or command registered to receive a custom URI scheme from an operating
system or framework dispatcher.

## HandlerSpec

The structured input used to generate or register handler artifacts.

## Handler ID

The stable artifact identity:

```text
<vendor>.<app>[.<instance>].<language>.<scheme>
```

## Instance

An optional handler identity segment used for local dev, staging, side-by-side
version installs, or app profiles.

## Parity

The requirement that Go, TypeScript, Python, Rust, and PHP implementations
produce the same behavior for the shared contract fixtures.

## Facade

The stable public API that hides parser, proto, and OS registration
dependencies.

## Backend

The replaceable implementation dependency behind a facade.

