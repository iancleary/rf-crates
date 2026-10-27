# Independent crate releases

## Mental model

There are four packages, not one workspace version. Each release has its own
manifest version, notes, tag, GitHub release, and crates.io artifact:
`rfconversions-v0.8.0`, for example. The runner uses `--latest=false`, but GitHub
may still display an automatically selected repository-wide “latest release”
(as observed for the initial Touchstone release). That label is not authoritative:
four independent package version lines do not have a useful shared latest version.

Preparation and publication are separate:

1. [The workspace helper](../scripts/release_policy.py) prepares one package's
   version, any corresponding root workspace requirement, and `Cargo.lock`.
   It defaults to no implicit action: select `--dry-run` or `--apply`.
2. Review, check, commit, and push those files to `main` with package-specific
   [release notes](../release/notes/). No package has been published yet.
3. The unchanged [maintained release runner](../scripts/release.py) executes the
   selected [crate contract](../release/). It checks a clean checkout, the exact
   version, public main ancestry, registry dependencies, and release notes;
   requires **every push workflow for the selected commit** to have succeeded,
   including workflow-file validation; runs all local checks; then creates the
   qualified tag and GitHub release.
4. That GitHub release triggers [the publication workflow](../.github/workflows/release.yml).
   It calls the same [workspace verification workflow](../.github/workflows/ci.yml)
   used by pull requests, without passing registry credentials.
5. Only after verification succeeds does a separate job dry-run and publish
   **one** package. It then waits up to ten minutes for its non-yanked index
   entry and verifies the published checksum against the uploaded archive.

A GitHub release is the publishing request, **not proof that crates.io accepted
it**. Treat a release as complete only after publication and checksum verification
succeed. Normally the workflow establishes both. A post-upload workflow failure
requires the explicit read-only recovery below, with the failure and recovery
recorded separately; never describe a failed workflow as a pass.

## Dependency order

Publish dependencies before consumers:

```text
rfconversions → touchstone → gainlineup
             └────────────→ gainlineup
             └────────────→ linkbudget
```

`linkbudget` can follow any time after its required rfconversions version is
available. A simple serial order is rfconversions, touchstone, gainlineup,
linkbudget. Wait for successful publication and index visibility between them;
there is no arbitrary fixed sleep. Publication workflows share one concurrency
group and never intentionally cancel an active publication.

The root workspace requirements must name a version that contains the API a
consumer uses. In particular, gainlineup's stable noise addition needs
rfconversions 0.8 or later. Workspace packaging stages local dependencies in
Cargo's temporary registry. It **does not** establish real registry availability;
the selected-package `cargo publish --dry-run` does that separately.

## Prerequisites and authorization

- A clean `main` checkout with the approved HTTPS `origin`, current remote refs,
  installed stable Rust, rustfmt, Clippy, `just`, `uv`, and authenticated `gh`.
- Explicit authorization for the selected package releases. No command in a
  document is standing permission to publish future releases.
- `CARGO_REGISTRY_TOKEN` stored as a GitHub Actions secret. One token can serve
  these four existing crates: use `publish-update`, restricted to their names.
  Do not enable `publish-new` merely for these version updates. Ownership and
  token permissions are ultimately verified by a real authorized upload.
- The token appears only in the publish step, never ordinary CI or command-line
  arguments. Dry-run/package checks do not prove token authorization.
- Exact stable `X.Y.Z` versions; prerelease publication is not supported by this
  contract. Historical member release scripts remain disabled.

Branch protection and a protected publishing environment are recommended
additional controls, but are not assumed to have been configured. The workflow
validates approved repository identity and that a release commit belongs to
public `main` before it exposes the registry token.

## Procedure

Example for an explicitly authorized minor rfconversions release:

```bash
just prepare-release rfconversions --bump minor --dry-run
just prepare-release rfconversions --bump minor --apply
# Review Cargo.toml, the member manifest, Cargo.lock, and release notes.
# Update release/notes/rfconversions.md to start with '# rfconversions X.Y.Z'.
# Commit the reviewed preparation, run just ci, and push it to origin/main.
# Wait for all push workflows, not only the named workspace verification run.
uv run --offline --no-project scripts/release_policy.py check-workflows

uv run --offline --no-project scripts/release.py check \
  --config release/rfconversions.toml --json
uv run --offline --no-project scripts/release.py plan \
  --config release/rfconversions.toml --version X.Y.Z --json
```

Use the exact `version`, `target_commit`, and `config_sha256` from that plan:

```bash
just cut-release rfconversions --dry-run --version X.Y.Z \
  --expected-head COMMIT --expected-config SHA256 --json
just cut-release rfconversions --apply --version X.Y.Z \
  --expected-head COMMIT --expected-config SHA256 --json

gh run list --workflow release.yml --repo iancleary/rf-crates
gh run watch RUN_ID --exit-status --repo iancleary/rf-crates
uv run --offline --no-project scripts/release_policy.py \
  wait-index rfconversions X.Y.Z --timeout 600
```

The tag runner does not bump manifests. This deliberate split prevents an
individual-Cargo-manifest runner from forgetting root workspace requirements.
The preparation helper uses Cargo's offline resolver to update the existing
lockfile while retaining unrelated locked dependencies. If a required cache
entry is absent, stop and fetch the required dependencies through the normal
Cargo workflow rather than substituting another dependency or bypassing checks.

Repeat the whole procedure with the next package only after the previous
publication is confirmed. For later releases, update the appropriate notes;
the initially checked-in minor-release notes are not reusable claims about
future changes.

## Recovery of an existing, unpublished release

When the GitHub release already exists but its version is absent from crates.io,
explicitly authorized recovery uses the corrected workflow on `main`, not a
moved tag or a recreated release. First verify current main's push checks with
`release_policy.py check-workflows`. Inspect the original tag commit and release.
Dispatch requires the GitHub CLI token to have **Actions: read and write** on
this repository; permission to push Git commits does not imply this permission.
An HTTP 403 from the dispatch API is a hard stop, not permission to change
credentials or substitute another trigger. The authorized operator can adjust
the token permissions or use Actions → Publish RF crate → Run workflow with
the same inputs. Never paste tokens into commands, documentation, or chat.

Then dispatch with publication disabled:

```bash
gh workflow run release.yml --repo iancleary/rf-crates --ref main \
  -f tag=touchstone-v0.17.0 \
  -f expected_commit=e55f46b2c5ef0db9d4c107d267887580ac360299 \
  -F publish=false
```

Find the resulting `workflow_dispatch` run with `gh run list --workflow
release.yml --event workflow_dispatch`, inspect its inputs and source commit,
and wait for success. Repeat the same command with `-F publish=true` only with
publication authorization. Wait for that run's upload and checksum checks before
releasing a consumer. The dry-run neither uploads nor performs the post-upload
index check; it does run the complete workspace and selected-package checks.

Both automatic release events and recovery dispatches resolve the existing,
non-draft, non-prerelease GitHub release to an exact main-branch ancestor. The
expected commit must match. Verification and publication check out that same
commit; neither silently builds the current main source. Dispatch is accepted
only from `main`. The registry token is confined to the conditional upload step,
and the existing preflight rejects already-published versions.

## Verification and operational limits

The shared gate preserves workspace membership, formatting, all-target and
all-feature Clippy, all-feature tests, library-only tests, independent numerical
comparisons, warnings-denied documentation, all four package archives, and
release builds. The publisher additionally verifies the selected artifact
against real crates.io dependencies, rather than accepting only a local path
build. Its target directory is isolated from workspace packaging. Archive lookup
requires exactly one matching file beneath that target's `package/` directory:
Cargo 1.99 stages publication archives under `package/tmp-crate/`, not directly
under `package/`. Never accept a stale archive left by an earlier workspace build.

Minimum-Rust-version metadata is preserved, but stable-toolchain CI does not
certify every declared historical compiler. Do not describe these releases as
having passed an MSRV matrix. Original upstream PRs and repository settings are
unchanged; this workflow does not close PRs or archive old repositories.

The vendored runner is unchanged from the release-skills revision recorded in
each contract, with its SHA-256 checked on every config load. Keep workspace
policy in `release_policy.py`, not in a divergent fork of the maintained runner.

## Failure and recovery

- **Preparation fails:** original manifest and lockfile bytes are restored. A
  successful preparation only modifies local files; inspect or revise them
  before committing. Never discard unrelated work.
- **Checks fail:** no release should proceed. Diagnose, repair, and rerun the
  failed gate. An unavailable registry dependency means publish or verify the
  dependency first, not loosen its requirement to an older incompatible API.
- **Tag exists but GitHub release creation failed:** inspect its exact commit;
  use the maintained runner's explicit `--resume --version X.Y.Z
  --expected-head TAG_COMMIT` flow from a clean checkout after reviewing the
  remote tag. Do not move or recreate a public tag.
- **GitHub release exists but the publication job failed:** inspect
  `gh run view RUN_ID --log-failed`. Confirm whether crates.io received the
  package before deciding to retry. If it is absent, an explicitly authorized
  failed-job rerun can repeat the guards and publish step.
- **Upload succeeded but the index wait failed:** do not publish again. Rerun
  only the checksum/index check. If the hosted archive is no longer available,
  rebuild with `cargo publish --dry-run` from a clean checkout of the exact tag,
  using a fresh dedicated target directory. Compare that archive, a downloaded
  registry archive, and the index checksum; verify `.cargo_vcs_info.json` names
  the tagged commit. Record the failed run and this read-only recovery before
  continuing. An existing registry version blocks blind publication retries.
- **Bad published package:** crates.io versions are immutable. A correction
  requires a new version; yanking or changing public metadata requires separate
  authorization. Local resets cannot undo publication.

## First publication recovery record

`rfconversions` 0.8.0 was accepted by crates.io in
[run 40450587741](https://github.com/iancleary/rf-crates/actions/runs/40450587741).
All shared checks, selected-package verification, and upload passed; the final
checksum step failed because it assumed the archive lived directly in
`target/package/`. The original run remains failed; the public tag was not moved
and the package was not republished.

A fresh-target dry-run from tagged commit
`e125e93ba76739ccda53e05806830320c3a1c46f` reproduced the actual staging path. Its
archive and the downloaded crates.io archive both matched the sparse-index
SHA-256 `124797d8aee1373c8cc052f30713d118a56be9db9d37d6674d4939e61ebd3664`;
the downloaded archive's VCS record named that clean tagged commit. The corrected
workflow locates exactly one archive in its isolated publication target, and its
checksum step was rerun successfully against the rebuilt release archive.

The subsequent Touchstone 0.17.0 GitHub release exposed a separate workflow
validation error: `runner.temp` is unavailable in job-level `env`. GitHub rejected
that workflow before starting a publication job; Touchstone was not uploaded.
The target path is now assigned in a step through `RUNNER_TEMP` and `GITHUB_ENV`.
The new `check-workflows` release gate checks all push runs for the exact commit,
so a passing workspace run cannot conceal a rejected publication workflow.

The public `touchstone-v0.17.0` tag still identifies
`e55f46b2c5ef0db9d4c107d267887580ac360299`. Do not move it or recreate its GitHub
release to obtain another event. The guarded workflow-dispatch recovery above
allows publication of that unchanged source after explicit authorization; the
ordinary tag runner remains responsible only for new release requests.

The reusable execution checklist lives in
[the crate release skill](../.agents/skills/release-rf-crate/SKILL.md).
