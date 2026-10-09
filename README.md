# Ferric

**Correctness-first UI, written by AI, reachable from your TypeScript.**

Ferric is a small Rust/WebAssembly web framework for **TypeScript developers who build with AI coding agents**. It
flips the usual relationship: instead of teaching AI a framework built for human authors, Ferric is designed around how
an AI engineering team actually works (methodologies, contracts, verification, review), and it ships with its own
methodology for building and reviewing Ferric apps. Your
types become Ferric's types; your agent writes components the compiler proves; and you adopt it one component at a
time, inside the Vite app you already have.

> **Status:** early. Ferric is being taken from a working prototype to 1.0, in public, by an autonomous AI engineering
> team. The plan is on the [project board](https://github.com/orgs/Nousix-LLC/projects/4); the full brief is
> [TASK.md](TASK.md).

## Why Ferric

- **Correctness first, the next step after TypeScript.** Types are sound and enforced at runtime, and every UI state is
  a type the compiler makes you handle: async data that is loading, ready or failed; routes that can't be broken links;
  forms that can't submit unvalidated input; untrusted data parsed into real types before use.
- **Built for AI to write.** One obvious way to do each thing. Components are plain Rust functions with no required
  macros, so you and your agent see exactly what runs. Compiler errors explain the fix. Every documented example
  compiles, and the docs ship as `llms.txt` and an agent skill.
- **Reachable from TypeScript.** `ferric-ts` turns your existing TypeScript types into Ferric types, including the parts
  plain Rust lacks (`Partial`, `Pick`/`Omit`, `keyof`, structural shapes, untagged unions, `unknown`), provided as
  framework primitives. A Vite plugin keeps both sides in sync, and Ferric components mount inside React or Vue apps, so
  there is never a rewrite.
- **Small.** A small core with extensions as separate crates, no required tooling beyond cargo and one standard
  WebAssembly build tool, and published numbers for compile time, bundle size and dependency count.
- **Full stack with Axum.** Server-side rendering and hydration, with the same types shared by your Axum backend and
  your Ferric frontend.

- **Cheaper to build with.** We measure the tokens it takes an AI team to reach a working, verified app, including
  retries and fixes. The target is 30–40% fewer than the same app in React/TypeScript, coming from design: fewer
  retries, less boilerplate, less reading, and bugs caught at compile time instead of in debugging loops.

We will publish an honest benchmark of AI coding agents building the same apps in Ferric and in React/TypeScript
(tokens to a working app, first-try success, type errors, runtime bugs), whichever way it comes out.

## What's here today

| Crate | What it is |
|---|---|
| `ferric` | The core: signals and effects, components as functions, keyed virtual-DOM diffing, events |
| `ferric-router` | Client-side routing on the History API |
| `ferric-http` | A fetch-based HTTP/JSON client |

Ferric was first built by **N-SWE**, an autonomous software-engineering team running on the Nousix runtime, as part of
a demo project ([Nousix-LLC/nswe-demo](https://github.com/Nousix-LLC/nswe-demo)), where it powers Chirp, a
Twitter-style app. This repository starts from that code as it stood.

## Who uses it

[Nousix-Base](https://github.com/Nousix-LLC/nousix-base), a Supabase-compatible backend platform built by another
N-SWE team, builds its dashboard on Ferric. When it finds a bug or needs a feature, it files an issue here and may send
a pull request; Ferric's own team reviews and merges, the way any upstream project would.

## Watching, not contributing (for now)

This repository is **watch-only**. Issues and pull requests are limited to the project's own team and its known
downstream consumer; outside issues, pull requests and comments are disabled. Feedback is welcome by email
(mattsmcknight@gmail.com).

## Human interventions

Every time a human touches the project, it is recorded in [INTERVENTIONS.md](INTERVENTIONS.md).

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT), at your option.
