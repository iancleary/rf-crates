# Public RF crate workspace

This repository contains exactly `rfconversions`, `touchstone`, `gainlineup`, and
`linkbudget` under `crates/`. Do not import private application source or add other
workspace members without explicit authorization.

## Start here

- Workspace model and commands: [README.md](README.md)
- Migration decisions and provenance: [docs/workspace.md](docs/workspace.md)
- Reviewed numerical changes: [docs/rfconversions-review.md](docs/rfconversions-review.md)
- Release and cutover boundaries: [docs/releases.md](docs/releases.md)
- Crate-specific RF contracts: each member's `AGENTS.md`

## Workflow

Check Git status and preserve existing work. Use GitHub and `gh` for repository
and PR operations. Publication, remote merges, repository archival, and releases
require explicit authorization. GitLab application workflows do not apply here.

Use `just check` at the workspace root; `just ci` also builds release artifacts.
Use `cargo test -p <crate>` for focused tests. Cargo owns one root lockfile and
local versioned path dependencies. Keep existing integration, documentation,
feature, lint, and package checks. Run Python through `uv run`.

Root release policy supersedes inherited standalone release instructions.
`just cut-release` and member release scripts are intentionally blocked until
cutover is approved. Nested `.github` workflows are historical source files,
not active monorepo automation. Do not promote their publish steps to root CI.

Each worktree must have its own Cargo target directory. Keep reusable procedures
in self-contained skill folders. Extend existing documentation and skill-owned
numerical comparison tools instead of duplicating them.
