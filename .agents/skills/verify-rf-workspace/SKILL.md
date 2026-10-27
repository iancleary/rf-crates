---
name: verify-rf-workspace
description: Verify changes in the four-crate public RF Cargo workspace, including local dependency resolution, numerical comparisons, and package archives. Use after cross-crate changes or workspace maintenance, not to authorize publication or import additional repositories.
---

# Verify the RF workspace

## Prerequisites and boundary

Run from the target workspace root with its installed Rust toolchain, `just`,
`uv`, and cached Cargo dependencies. Read `AGENTS.md` and the affected crate's
instructions. Use GitHub/`gh` for public PR work; remote writes need authorization.

The workspace contains only rfconversions, touchstone, gainlineup, and linkbudget.
The repository owns the Cargo/just verification commands and workspace checker;
this skill does not duplicate those tools. Imported numerical generators remain
packaged with the owning crate's literature-review skill.

## Procedure

1. Check Git status. Preserve unrelated edits and use a separate target directory
   for each worktree. Inspect `docs/workspace.md` when changing dependencies or
   import boundaries.
2. Run `just workspace-check`. Confirm that all RF dependencies resolve locally,
   exactly four members exist, licenses remain present, and member lockfiles have
   not returned. Do not relax this check to accommodate an accidental fifth crate.
3. Run the affected crate's public workflows and existing integration tests before
   and after behavioral changes. Numerical expectations must be independently
   derived, not copied from the new implementation.
4. Run `CARGO_NET_OFFLINE=true just check`. This includes all-target/all-feature
   Clippy, all-feature and library-only tests, retained numerical comparisons,
   documentation, and workspace package verification. It requires a clean
   committed checkout for packaging; inspect and commit only authorized changes.
5. Run `CARGO_NET_OFFLINE=true just ci` before delivery when release builds are
   relevant. Inspect the generated package archives and dependency graph, not
   only the compile result.
6. Review the diff and report failures and unverified platforms/MSRVs. A local
   CI-equivalent run is not proof that GitHub Actions has run.
7. Follow `docs/releases.md` for pending cutover and release boundaries. Do not
   publish, merge upstream PRs, archive repositories, or enable credentials as
   a side effect of verification.

## Failure and recovery

- Missing offline dependencies: report the missing package and obtain approved
  setup; do not install tools implicitly.
- A fixture assumes `target/` exists: make test output independent of Cargo's
  artifact location; do not weaken the real output assertion.
- A workspace-only API fails package verification: inspect version requirements
  and the packaged dependency graph. Do not claim a dependency is published
  merely because Cargo staged it locally.
- A numerical comparison disagrees: check input units, exact binary64 values,
  reference precision, and classification before adjusting a tolerance.
- Correct migration errors with focused commits. Original source histories are
  preserved, so do not reset or rewrite the original repositories.
