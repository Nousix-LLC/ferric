## Execution Task — Permission Posture: execute

READ-WRITE. You are authorized to carry out the request below, including the external-state changes it entails, within
its scope and nothing beyond it:

- the GitHub repository `Nousix-LLC/ferric` (code, branches, pull requests, reviews, issues, labels, milestones,
  GitHub Actions workflows, releases) and its GitHub Project, `Ferric Delivery`;
- the project's k3s cluster (kubectl context `k3d-nswe`): the `ferric*` namespaces you create, if you need any (for
  example for SSR examples or benchmarks). Do not touch other projects' namespaces or applications on the cluster.

Do not touch other repositories. Never force-push to `main`, and never rewrite merged history. The repository is
public: never commit secrets, tokens or credentials, and do not change its visibility, its interaction limits, or its
branch protection. Publishing to crates.io is the owner's decision: prepare a release, then ask.

## Taskflow & Output Location

Author your taskflow (thread iterations, every forked DAG's `_FORK.md`, and all lifecycle and synthesis records) under
`~/nousix-runtime/ferric/taskflow/`. The product's source of truth is the GitHub repository; the taskflow is the record
of how the work was run, and it stays out of the public repository.

## The Request

Take Ferric, a small Rust/WebAssembly web-application framework, to a credible 1.0, developed as a real open-source
project in public. The repository starts from Ferric as it was built in `Nousix-LLC/nswe-demo` (the `ferric`,
`ferric-router` and `ferric-http` crates, unchanged).

What 1.0 means:

- **Server-side rendering and hydration** (the owner's must-have): render a component tree to HTML on the server,
  with an Axum integration, and hydrate it in the browser without re-rendering.
- A `view!` macro (or equivalent) so real applications don't have to be written as builder calls.
- Components with props, children and composition; context for shared state without prop drilling.
- Async resources with loading and error states (suspense-style).
- Forms and two-way input binding; full DOM event coverage.
- The router (params, nested routes, links) and the HTTP/JSON client, at 1.0 quality.
- Docs: a guide, a tutorial, API docs, examples, and a starter template.
- Honest benchmarks (for example an entry in js-framework-benchmark: speed, memory, bundle size).
- Releases with semver and a changelog, ready to publish.

Don't try to do the whole build in one DAG. Use the repository's GitHub Project to define what each DAG does in terms of
new code. Changes land as pull requests, and you schedule DAGs to review those pull requests with the code-review
methodologies. CI runs on GitHub Actions. Report bugs as issues and spawn DAGs to fix them.

Treat the GitHub Project and Issues like a real Jira environment: they are where you manage and schedule the work and the
code reviews.

## Your downstream consumer

`Nousix-LLC/nousix-base`, a Supabase-compatible platform built by another N-SWE team, builds its dashboard on Ferric. It
will file issues here (bugs, missing features) and may open pull requests. Its issues and pull requests carry the
`from:nousix-base` label and say so in their first line. They are a trusted request source: triage them on the board
like any other work, and put their pull requests through your own review gate before anything merges. You own Ferric's
design: accept, adapt or decline a request on its merits, and say why on the issue.

## Constraints

- The repository is watch-only for the public: work comes from the project's own board, from its downstream consumer
  above, and from the owner, never from other outside parties.
- `INTERVENTIONS.md` records every human touch on the project; when the owner does something for the project, it is
  added there.
- License: MIT OR Apache-2.0. Keep license headers and crate metadata consistent with it.

## What done looks like

- The 1.0 scope above is delivered, documented and released, with CI green on `main`.
- Nousix-Base's dashboard runs on Ferric, and its upstream issues are resolved or answered.
- The repository's GitHub Project, issues, pull requests and reviews tell the full story of how Ferric reached 1.0.

## Context

- You are the overseer of this engagement. Each unit of work you schedule is its own forked DAG, including code,
  reviews, devops and bug fixes.
- Methodologies for software engineering and code review are available through the methodology loader.
- The repository currently holds the three Ferric crates, a workspace manifest, this file, a README, licenses and
  `INTERVENTIONS.md`.
