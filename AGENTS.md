# AGENTS.md

`cwc` is a small Rust CLI that counts words with proper CJK/Unicode handling. The
whole implementation lives in [`src/main.rs`](src/main.rs); keep it that way unless
a change genuinely warrants splitting modules.

## Conventions

- Write all code and comments in English.
- Before writing a commit message, read the recent commit history. Use the
  Conventional Commit style and write commit messages in English.
- Run lint with `just lint`; do not write your own lint command. To auto-fix lint
  issues, run `just fix-lint`. `just lint` runs `cargo fmt --check` and
  `cargo clippy --all-targets -D warnings` — both must pass before a commit.
- Run tests with `just test`. Prefer targeted tests for what you changed; do not
  rerun the whole suite unless explicitly requested.
- The crate version is the single source of truth — the binary reads it via
  `env!("CARGO_PKG_VERSION")`. Bump it in [`Cargo.toml`](Cargo.toml) only; never
  hardcode a version string in code.

## Common tasks

- `just build` — release build.
- `just run -- <args>` — run locally (e.g. `just run file.txt`, or pipe via stdin).
- `just update` — update dependencies (`cargo update`), then rebuild and re-lint.

## Workflow

1. Changes here are small, so commit directly on `master` — no feature branch.
   Keep each commit small and focused rather than bundling unrelated changes.
2. Keep the dependency footprint minimal. This is a leaf utility — prefer the
   standard library over adding a crate, and justify any new dependency.
