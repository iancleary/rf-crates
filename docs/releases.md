# Release policy and repository cutover

## Current boundary

Development, numerical checks, package creation, and package verification work
locally. Automated publication is deliberately **not enabled** during migration.
The root `just cut-release` and imported member release scripts exit nonzero
before modifying files or contacting a forge. An imported `vX.Y.Z` runner cannot
select a crate safely in a four-package repository.

The root GitHub Actions workflow only validates code. It has read-only repository
permissions, no release trigger, and no registry token. Nested source workflows
are retained as history-bearing files; GitHub does not execute them from
`crates/*/.github/workflows`.

## Version and package rules

- Keep the four package names and independent SemVer versions. A monorepo does
  not require a shared version or simultaneous releases.
- Imported versions identify the starting point, not permission to overwrite
  those versions on crates.io. Choose new versions before publishing changes.
- Update a member manifest and relevant root workspace dependency requirements
  together, then regenerate the root lockfile with Cargo.
- Preserve crate licenses and registry-compatible version requirements on local
  path dependencies. Do not substitute Git-only dependencies for publication.
- Future tags must identify the crate, for example `rfconversions-v0.8.0`; the
  example is a naming proposal, not an approved next version.

Dependency publication order, when the corresponding packages changed:

1. `rfconversions`
2. `touchstone` and `linkbudget`, after their required rfconversions version exists
3. `gainlineup`, after its required rfconversions and touchstone versions exist

The numerical PRs add an API consumed by gainlineup. Its eventual published
rfconversions requirement must name a version containing that API. A successful
workspace package check does not prove that version already exists on crates.io.

## Verification before a release

From a clean committed checkout:

```bash
CARGO_NET_OFFLINE=true just ci
```

This preserves format, all-target/all-feature Clippy, all-feature and library-only
tests, independent numerical comparisons, warnings-denied documentation, package
verification, and release builds. `cargo package --workspace --locked` stages
local packages in Cargo's temporary registry; it does not publish them.

Package-manifest MSRV claims remain unchanged. Workspace verification uses the
installed modern Rust toolchain and does not independently certify every older
MSRV. Validate the supported compiler matrix before making a release claim.

## Pending, separately authorized cutover

1. Confirm the GitHub owner/repository name and public visibility.
2. Review the imported history and current tree for public suitability. No private
   application repository was used as an import source.
3. Create the GitHub repository, set `origin`, and publish the reviewed history.
4. Update each package's homepage/repository metadata to the confirmed monorepo
   destination. Until then it intentionally points to the original repository.
5. Configure branch protection and inspect a real GitHub CI run. Local execution
   does not verify hosted runner behavior.
6. Decide whether to merge or close the original rfconversions PRs with a migration
   explanation. Local subtree integration has not changed those PR states.
7. Port one release runner to crate-specific versions, notes, tags, clean-tree
   checks, and dependency order. Require explicit version and crate selection;
   dry-run must not publish. Reuse maintained release tooling instead of adding
   a second competing release framework.
8. Configure registry credentials only after the publication path is approved.
   Validate the runner in an isolated repository before enabling it.
9. Redirect or archive the old repositories only after their outstanding work is
   accounted for. Other open PRs were not silently incorporated.

Do not run `cargo publish`, push tags, close PRs, or archive repositories merely
to complete local workspace setup. If a migration correction is needed before
cutover, make another local commit. The original checkouts and remote histories
remain unchanged, so there is no destructive rollback to perform.
