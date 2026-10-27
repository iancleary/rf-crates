# Workspace architecture and migration record

## Ownership

`rfconversions` owns scalar RF math. `touchstone` owns measured network data and
S-parameter operations. `gainlineup` owns ordered hardware stages. `linkbudget`
owns complete radio links. The workspace makes changes across those boundaries
verifiable in one checkout; it does not combine their APIs or release versions.

```text
rfconversions
  ├── touchstone
  │     └── gainlineup
  ├── gainlineup
  └── linkbudget
```

The root manifest lists exactly four members. The
[workspace verifier](../scripts/verify_workspace.py) checks membership, paths,
licenses, member lockfile absence, and absence of registry copies of those four
crates in the resolved graph. This prevents a local edit from being tested
against an older downloaded RF dependency by mistake.

Standalone examples and Touchstone fuzzing retain their own manifests and
lockfiles outside the member set. They are not additional published workspace
crates. Their external dependencies remain external; no other crate repository
was imported.

## Source provenance

Imports used `git subtree add --prefix=crates/<crate> <commit>` without squash.
Original commit objects and authors remain reachable through the monorepo merge
history. The initial imported tree of each crate was compared byte-for-byte to
its source tree with `git diff <source>^{tree} HEAD:crates/<crate>`.

| Repository | Imported main commit |
| --- | --- |
| [rfconversions](https://github.com/iancleary/rfconversions) | `061c86863d61eac08d6c01ca148bdb428f31dc1c` |
| [touchstone](https://github.com/iancleary/touchstone) | `8b58738d17a21f2a9e605a4afb12b36bed173913` |
| [gainlineup](https://github.com/iancleary/gainlineup) | `8064f126647c03298dd9b81da2a528e8759038e5` |
| [linkbudget](https://github.com/iancleary/linkbudget) | `e69c692748b415832837a8ef857a74e3269179a5` |

The rfconversions PR heads were then integrated locally with
`git subtree merge --prefix=crates/rfconversions <head>`, preserving their history:

| PR | Imported head |
| --- | --- |
| [36](https://github.com/iancleary/rfconversions/pull/36) | `ba03edcabd9cc647a83aadeab3da56d4a792195e` |
| [37](https://github.com/iancleary/rfconversions/pull/37) | `1a492febe14f307fb92b7d69a08edfab1f71e7e8` |
| [38](https://github.com/iancleary/rfconversions/pull/38) | `5b7bb28d52d3d2995fddcc770a31228b8f3a6927` |
| [39](https://github.com/iancleary/rfconversions/pull/39) | `1fe4d852d1d16e926dfb6071f4590da7a79b5fee` |

Verify ancestry with `git merge-base --is-ancestor <commit> HEAD`. Inspect original
paths with `git show <commit>:src/power.rs`; inspect imported paths in newer
commits under `crates/`. This is a one-time consolidation, not a bidirectional
subtree synchronization policy. GitHub issues, PR discussions, branch settings,
and old release tags are not migrated by preserving commit history.

Only the four public GitHub repositories were import sources. Original local
checkouts and branches were not changed. No private application history, local
build outputs, credentials, or unrelated open feature branches were imported.

## Migration decisions and observed corrections

1. Preserve crate names, independent versions, licenses, and module ownership.
2. Replace member lockfiles with one root lockfile. Excluded demonstration/fuzz
   packages still own their independent locks.
3. Resolve all shared RF dependencies locally using versioned workspace paths.
   This moves gainlineup from Touchstone 0.14 to the local 0.16 API.
4. The compiler caught a private-field access in gainlineup's CLI adapter:
   `network.f.clone()`. Replace it with `network.frequencies().to_vec()`;
   the owned frequency vector and its units remain unchanged.
5. The standalone linkbudget baseline exposed a test that assumed a local
   `target/` directory existed. The plot-generation test now creates and cleans
   a temporary output directory, so workspace and relocated-target runs exercise
   the actual HTML generator without depending on build-directory placement.
6. Add one root CI workflow. Retain imported nested workflows for provenance,
   but do not activate their standalone publish events.
7. Guard old release scripts before mutation. The subsequent release cutover
   retains those guards and adds independent crate contracts using the unchanged
   maintained release runner; see the [release procedure](releases.md).

Baseline all-feature tests passed for rfconversions, touchstone, and gainlineup.
Linkbudget's initial relocated-target run failed only its HTML-output-directory
assumption. After the two migration corrections, the all-feature workspace tests,
all-target Clippy, and four-package artifact verification passed locally.
The final verification record belongs in the implementation handoff; do not
interpret this initial baseline as a hosted CI or minimum-Rust-version result.

## Working across crates

From the workspace root:

```bash
cargo tree -p gainlineup
cargo test -p touchstone --all-features --locked
cargo test --workspace --all-features --locked
cargo package --workspace --locked
```

Cargo still runs package tests from their package directories, so existing RF
fixtures under `files/` retain their meaning. Run a file-based CLI from its crate
directory when using the paths in that crate's README. Prefer existing CLI
integration tests in automated verification; interactive plotting may open a
browser and is not a headless smoke check.

A workspace package build uses local dependencies. A registry consumer does not.
The package verification step checks normalized manifests and extracted sources;
publication still requires new versions and dependency availability.

## Recovery boundaries

Work in separate branches/worktrees with separate Cargo targets. Do not reset an
original checkout to recreate the import. Correct a local migration with an
ordinary commit or reconstruct a fresh destination from the recorded source
commits. Public publication, upstream PR state changes, and repository archival
remain separately authorized actions.
