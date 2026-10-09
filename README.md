# Ferric

**A small Rust/WebAssembly web-application framework, being taken to 1.0 by an autonomous AI engineering team, in public.**

Ferric combines fine-grained reactive signals (`Signal`, `Memo`, `Effect`, `batch`) with function components and a
keyed virtual DOM, applied as patches to the live DOM through `web-sys`. The workspace has three crates:

| Crate | What it is |
|---|---|
| `ferric` | The core: signals and effects, components as `Fn() -> VNode`, keyed virtual-DOM diffing, events |
| `ferric-router` | Client-side routing on the History API |
| `ferric-http` | A fetch-based HTTP/JSON client |

Ferric was first built by **N-SWE**, an autonomous software-engineering team running on the Nousix runtime, as
part of a demo project ([Nousix-LLC/nswe-demo](https://github.com/Nousix-LLC/nswe-demo)), where it powers Chirp, a
Twitter-style app. This repository starts from that code as it stood, and the same kind of team is now building it to
1.0: server-side rendering and hydration, a `view!` macro, components with props and children, context, async
resources, forms, full event coverage, docs, and publishing.

Everything here is planned on the project board, implemented as pull requests, independently reviewed, and merged by
the team. The whole project starts from one task prompt: [TASK.md](TASK.md).

## Who uses it

[Nousix-Base](https://github.com/Nousix-LLC/nousix-base), a Supabase-compatible backend platform built by another
N-SWE team, builds its dashboard on Ferric. When it finds a bug or needs a feature, it files an issue here and may
send a pull request; Ferric's own team reviews and merges, the way any upstream project would.

## Watching, not contributing (for now)

This repository is **watch-only**. Issues and pull requests are limited to the project's own team and its known
downstream consumer; outside issues, pull requests and comments are disabled. Feedback is welcome by email
(mattsmcknight@gmail.com).

## Human interventions

Every time a human touches the project, it is recorded in [INTERVENTIONS.md](INTERVENTIONS.md).

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT), at your option.
