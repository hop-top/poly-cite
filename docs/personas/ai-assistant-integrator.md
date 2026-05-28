---
doc_type: product
subtype: persona
status: draft
title: AI assistant integrator persona
summary: Persona for AI assistants and agent runtimes that classify, generate, and follow custom URI references.
owner: Idea Crafters Labs
created: 2026-05-13
updated: 2026-05-13
audience:
  - ai_assistant
  - developers
  - tool_authors
confidentiality: public
tags:
  - cite
  - persona
  - ai-assistant
---

# AI Assistant Integrator

## Responsibilities

- Generate URI references in responses and artifacts.
- Classify pasted URIs into known objects.
- Use completions to reduce hallucinated IDs.
- Route URI handling to the correct app or tool.

## Goals

- Stable references that tools can resolve.
- Clear namespace and id semantics.
- Low ambiguity when multiple apps expose similar object types.

## Pain Points

- Free-form links are hard to validate.
- Scheme-only parsing hides app and namespace identity.
- Generated handler files can collide when assistants scaffold multiple apps.

## Success Criteria

- Assistant-generated URIs pass fixture validation.
- Assistant instructions can reference the glossary and specs.
- Completion providers can be used as grounding sources.

