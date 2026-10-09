## Execution Task — Permission Posture: execute

READ-WRITE. You are authorized to carry out the request below, including the external-state changes it entails, within
its scope and nothing beyond it:

- the GitHub repository `Nousix-LLC/ferric` (code, branches, pull requests, reviews, issues, labels, milestones,
  GitHub Actions workflows, releases) and its GitHub Project, `Ferric Delivery`;
- the project's k3s cluster (kubectl context `k3d-nswe`): the `ferric*` namespaces you create, if you need any (for
  example for SSR examples or benchmarks). Do not touch other projects' namespaces or applications on the cluster.

Do not touch other repositories. Never force-push to `main`, and never rewrite merged history. The repository is
public: never commit secrets, tokens or credentials, and do not change its visibility, its interaction limits, or its
branch protection. Publishing to crates.io or npm is the owner's decision: prepare a release, then ask.

## Taskflow & Output Location

Author your taskflow (thread iterations, every forked DAG's `_FORK.md`, and all lifecycle and synthesis records) under
`~/nousix-runtime/ferric/taskflow/`. The product's source of truth is the GitHub repository; the taskflow is the record
of how the work was run, and it stays out of the public repository.

## The Request

Take Ferric, a small Rust/WebAssembly web-application framework, to a 1.0 that frontend developers choose, developed as
a real open-source project in public. The repository starts from Ferric as it was built in `Nousix-LLC/nswe-demo` (the
`ferric`, `ferric-router` and `ferric-http` crates, unchanged).

### Who it is for, and why they would choose it

Ferric is for **TypeScript frontend developers who build with AI coding agents**, not for Rust developers looking for a
frontend. Its identity:

> **Correctness-first UI, written by AI, reachable from your TypeScript.**
> Your types become Ferric's types; your agent writes components the compiler proves; adopt it one component at a time,
> inside the Vite app you already have.

Ferric flips the usual relationship between a framework and the AI that writes code with it. Instead of teaching AI an
existing framework built for human authors, Ferric is **designed around how a methodology-driven AI engineering team
(N-SWE) works**: briefs, contracts, methodologies, verification and independent review. Human TypeScript developers
benefit for the same reasons (explicit, typed, verifiable, small), but N-SWE's workflow sets the design.

The reasoning behind that, which every design decision should serve:

- TypeScript won because it made correctness affordable and adoptable: a superset you could move to one file at a time.
  Ferric goes one step further (types that are sound and enforced at runtime, every UI state accounted for) and must be
  just as adoptable: incremental, mechanical, never a rewrite.
- Rust's main cost is that it is hard to write; its main benefit is that code that compiles is far more likely to be
  correct. When an AI agent writes the code, the agent pays the cost and the developer keeps the benefit: the compiler
  becomes the verifier of AI-written UI.
- Rendering speed and build tooling are not the pitch. WebAssembly still reaches the DOM through JavaScript glue, and the
  JavaScript toolchain (Vite, Rolldown, Oxc) is already fast. Ferric competes on correctness, AI-writability, shared
  types, a small dependency surface, and the cases where WebAssembly genuinely beats JavaScript (local-first and offline
  apps, sync, heavy client-side data, in-browser compute).
- It is the small framework in a field of large ones (the Flask to Leptos' and Dioxus' Django): a small core, extensions
  as separate crates, no required tooling beyond cargo and one standard WebAssembly build tool.

### Design principles (plan, build and review against these)

1. **Correctness first.** Every UI state is a type the compiler makes you handle:
   - async data as a resource that is loading, ready or failed, matched exhaustively, so no screen forgets its loading
     or error state;
   - routes as a typed enum, so a broken link does not compile;
   - forms as typed state, so unvalidated input cannot be submitted;
   - accessibility basics required by the types (for example an image needs alt text);
   - untrusted data must be parsed into a real type before use ("parse, don't validate").
2. **Built for AI to write.**
   - One obvious way to do each thing; few, consistent patterns.
   - No required macros. Components are plain Rust functions and markup is a plain-Rust builder API, so an agent and its
     reviewer see exactly the code that runs and compiler errors point at the user's code. Small derives may exist as
     optional conveniences; a large DSL macro is never the only way.
   - Errors that teach the fix: custom diagnostics that tell the writer what to change, so the compiler is something an
     agent can iterate against.
   - Documentation that is literally true and machine-readable: every API documented as a precise contract, every
     example a compiled doctest, an `llms.txt`, and an agent skill for building with and migrating to Ferric.
   - Verification built in: deterministic server-rendered output for snapshot tests, and a small render–interact–assert
     test harness that runs without a browser.
3. **Reachable from TypeScript.** Adoption is incremental and mechanical (see "The TypeScript path").
4. **Small core, extensions outside it.** The core stays small; routing, HTTP, forms, SSR integrations and the like are
   separate crates. Measure and publish compile time, bundle size and dependency count, and keep them honest.

5. **Built for N-SWE, measured in tokens.**
   - Every API must be expressible as methodology guidance ("do this, check that") and checkable in review. If a
     feature can't be described that way, it is the wrong API for Ferric.
   - Each feature should make a step of N-SWE's brief → build → verify → review loop more reliable: typed UI states map
     to acceptance criteria, deterministic server-rendered output to snapshot checks, errors that teach the fix to DAGs
     repairing their own failures.
   - **Cost to build is a design goal, and measuring it honestly is part of the work.** Raw token counts mislead (they
     are dominated by cached context); the real saving is work that never has to happen because it was right the first
     time. Define and publish a fair metric for it (candidates: first-pass correctness, the share of effort spent on
     rework and retries, iterations to a verified green build, review findings per change) before claiming a number.

### Ferric's own methodology

Ferric ships with its own methodology bundle, versioned with the framework: how to build a Ferric app (structure, state,
data, forms, routing, SSR, the TypeScript path) and how to review one. Every release updates the guidance alongside the
code. The `llms.txt` and the agent skill are exports of this bundle for other agents. Nousix-Base's dashboard team uses
it, and its friction points come back upstream as issues, so the framework and its methodology improve together from
real AI usage.

### The TypeScript path

Types are what TypeScript teams have invested most in, so the path starts there.

- **`ferric-ts`**: generates Ferric types from existing TypeScript types (`.ts` / `.d.ts`), using a Rust TypeScript parser
  (Oxc). Interfaces, discriminated unions, literal unions, optionals, arrays, records, branded types, promises and
  behavioral interfaces map to structs, enums, options, collections, newtypes, async functions and traits. The output is
  plain, readable Rust with no macro expansion to hide what runs.
- **Framework primitives for what Rust lacks.** Ferric is a framework, so where TypeScript has a concept plain Rust does
  not, Ferric provides it as a framework primitive and the converter maps onto it, rather than flagging it or forcing a
  poor fit. For example:
  - `Partial<T>` → a patch type (every field optional, applicable to a `T`), which forms and PATCH endpoints need anyway;
  - `Pick` / `Omit` → projections that convert from the full type;
  - `keyof T` and `T[K]` → a generated field enum with typed field access (tables, sorting, form bindings);
  - structural compatibility ("has these fields, so it fits") → a shape/conformance trait the converter generates;
  - untagged unions of object shapes → a one-of type discriminated by structure when data arrives;
  - conditional and mapped types → resolved by the converter into concrete Ferric types for the uses the code actually
    has;
  - `unknown` / `any` → an unknown value that must be parsed into a real type before it can be used.
  These primitives should be good on their own terms for Rust-first users too. Only genuinely open-ended type-level
  computation is flagged for a human or agent to decide; nothing is silently approximated.
- **Shared types, one source of truth.** The same types serve an Axum backend and the Ferric frontend, and TypeScript
  types can be generated back for code that stays in TypeScript, so frontend, backend and the remaining TypeScript never
  drift.
- **A Vite plugin** so Ferric drops into the toolchain TypeScript developers already use, with type sync in watch mode
  (type drift is a build error).
- **Mount inside existing apps**: Ferric components usable inside a React or Vue app (as islands or web components) with
  generated TypeScript typings, so a team can move one component at a time.
- **A concept map** from React to Ferric (props interface → props type, state hook → signal, effect hook → effect, memo
  hook → memo, function component → function component), as docs and as the agent skill.

### What 1.0 includes

- Server-side rendering and hydration, with an Axum integration (the owner's must-have).
- The correctness primitives in principle 1 and the TypeScript primitives above.
- `ferric-ts`, the Vite plugin, and mounting inside React/Vue apps.
- Components with props, children and composition; context for shared state.
- Forms and two-way binding; full DOM event coverage.
- The router (params, nested routes, links) and the HTTP/JSON client at 1.0 quality.
- Docs: a guide, a tutorial for TypeScript developers, API docs, examples, a starter template, `llms.txt`, the agent
  skill.
- Releases with semver and a changelog, ready to publish.
- **Ferric's own methodology bundle** (above), plus its `llms.txt` and agent-skill exports.
- **A published, honest agent benchmark**: a fixed set of realistic UI tasks, built by N-SWE with Ferric and its
  methodology, and by N-SWE and other coding agents in React/TypeScript, measuring the cost-to-build metric defined above
  (not raw tokens), first-try success, compile/type errors, review findings, and runtime bugs found by tests. Publish
  the method and the results whichever way they come out.
- Honest performance numbers (compile time, bundle size, dependency count; a js-framework-benchmark entry).

### How to run the project

Don't try to do the whole build in one DAG. Use the repository's GitHub Project to plan the work: design work (each
area's design written up and reviewed before it is built), then implementation. Changes land as pull requests, and you
schedule DAGs to review those pull requests with the code-review methodologies. CI runs on GitHub Actions. Report bugs as
issues and spawn DAGs to fix them.

Treat the GitHub Project and Issues like a real Jira environment: they are where you manage and schedule the work and the
code reviews.

## Your downstream consumer

`Nousix-LLC/nousix-base`, a Supabase-compatible platform built by another N-SWE team, builds its dashboard on Ferric. It
will file issues here (bugs, missing features) and may open pull requests. Its issues and pull requests carry the
`from:nousix-base` label and say so in their first line. They are a trusted request source: triage them on the board
like any other work, and put their pull requests through your own review gate before anything merges. You own Ferric's
design: accept, adapt or decline a request on its merits and against the principles above, and say why on the issue.

## Constraints

- The repository is watch-only for the public: work comes from the project's own board, from its downstream consumer
  above, and from the owner, never from other outside parties.
- `INTERVENTIONS.md` records every human touch on the project; when the owner does something for the project, it is
  added there.
- License: MIT OR Apache-2.0. Keep license headers and crate metadata consistent with it.

## What done looks like

- The 1.0 scope is delivered, documented and released, with CI green on `main`.
- A TypeScript developer can follow the tutorial, generate Ferric types from their own TypeScript, and mount a Ferric
  component in their existing Vite app.
- The agent benchmark is published, including the cost-to-build comparison against React/TypeScript.
- Nousix-Base's dashboard runs on Ferric, and its upstream issues are resolved or answered.
- The repository's GitHub Project, issues, pull requests and reviews tell the full story of how Ferric reached 1.0.

## Context

- You are the overseer of this engagement. Each unit of work you schedule is its own forked DAG, including design,
  code, reviews, devops and bug fixes.
- Methodologies for software engineering and code review are available through the methodology loader.
- The repository currently holds the three Ferric crates, a workspace manifest, this file, a README, licenses and
  `INTERVENTIONS.md`.
