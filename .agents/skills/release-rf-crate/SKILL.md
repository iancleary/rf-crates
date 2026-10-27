---
name: release-rf-crate
description: Prepare and publish an explicitly authorized RF workspace crate through its checked-in release contract, then verify registry availability before releasing a consumer.
---

# Release one RF crate

Use for `rfconversions`, `touchstone`, `gainlineup`, or `linkbudget` releases in
this repository. Read [the maintained release procedure](../../../docs/releases.md)
first, then the selected `release/<crate>.toml` contract. The owning repository
provides the shared runners; this skill deliberately does not duplicate them.

## Prerequisites

Require explicit publication authorization, a clean main checkout, GitHub access,
existing Rust/just/uv tools, and the configured GitHub registry secret. Never read
or print that secret. Preserve independent package versions and MIT licenses.

## Sequence

1. Inspect Git status, current manifests, and live registry versions. Confirm the
   requested bump and dependency order; do not infer permission for other crates.
2. Use `just prepare-release <crate> --bump <level> --dry-run`, then the same
   command with `--apply`. Review the package version, root workspace requirements,
   root lockfile, and crate-specific notes. Commit and check the preparation,
   then push it to main within the authorized release scope.
3. Wait for all push workflows for the prepared commit. The contract's
   `check-workflows` gate rejects absent, pending, or failed runs, including
   workflow-file validation failures hidden by a successful workspace run.
   Run the selected contract's `check`, then `plan --version X.Y.Z`. Preserve the
   returned commit and config checksum. Run `just cut-release <crate> --dry-run`
   with those exact guards; only then repeat with `--apply`.
4. Inspect the resulting `Publish RF crate` run and wait for success. Confirm
   the exact version through `release_policy.py wait-index`; a GitHub release
   alone is not completion evidence.
5. Proceed to the next authorized crate only after its dependencies are indexed
   and the prior publication is confirmed. Stop at any failed gate.

## Verification and recovery

Keep complete workspace checks and selected-package registry dry-runs. The hosted
publisher verifies the actual uploaded archive checksum in the sparse index.
Its isolated publication target must contain exactly one selected archive; do not
assume Cargo uses the same archive path as workspace packaging. After an upload,
use the documented checksum-only recovery rather than rerunning publication.
Do not claim that dry-run verifies token scope or that stable CI proves MSRV.

Preparation failures restore original version files. After any public mutation,
inspect the actual tag, workflow, and registry state. Use the maintained runner's
explicit resume contract only for its supported GitHub-release failure boundary;
use the recovery section of the release guide for publication-job failures.
If a GitHub release exists but its registry version is absent, explicitly
authorized recovery dispatches `release.yml` from main with the existing tag,
its exact expected commit, and `publish=false` first. Verify the completed dry-run
before dispatching with `publish=true`. Both runs must test the tagged source,
not current main; do not treat a skipped upload as publication success.
Dispatch needs Actions read/write permission on the CLI token. On HTTP 403,
stop and ask the operator to grant that permission or use the GitHub workflow UI
with the same inputs. Do not change credentials or substitute a trigger.
Never move public tags, silently republish an existing version, yank a package,
close upstream PRs, or archive repositories as part of ordinary recovery.
