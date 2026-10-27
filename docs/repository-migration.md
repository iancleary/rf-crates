# Repository handoff and bounded work items

The four crates and their source history are consolidated; versions rfconversions
0.8.0, touchstone 0.17.0, gainlineup 0.24.0, and linkbudget 0.8.0 are published.
This handoff moves issue ownership and pending proposals, not feature approval.
See [workspace provenance](workspace.md) and [release operations](releases.md).

## Completion contract

- [x] Transfer the five open issues using GitHub's native transfer operation,
  preserving authorship and discussion history.
- [x] Preserve fifteen pending PR heads and dependency stacks in unmerged monorepo
  PRs, retaining review-ready status after migration verification. Each proposal
  has one bounded implementation issue and explicit acceptance.
- [x] Verify the four integrated rfconversions PR heads are ancestors of main and
  the 0.8.0 release, then close their originals with that evidence.
- [x] Verify every replacement before closing its original PR. Preserve original
  discussions through source links; do not represent copied reviews as new reviews.
- [x] Update current README installation/source links and migration guidance.
- [x] Add original-repository notices and disable their legacy publishing workflows
  only after all open work has a verified destination.
- [x] Verify the final remote state against the checked-in work-item inventory.

No pending feature PR merge into main, new package publication, history rewrite,
or repository archival is authorized by this procedure. Protection settings and unresolved release
readiness findings remain separately bounded follow-up work.

## Handoff results

All five original issues were transferred, not copied. GitHub redirects their
old URLs to the same issue records in this repository.

| Original issue | Current tracker |
| --- | --- |
| rfconversions #34 | [#1 — 1.0 readiness](https://github.com/iancleary/rf-crates/issues/1) |
| touchstone #87 | [#2 — 1.0 readiness](https://github.com/iancleary/rf-crates/issues/2) |
| gainlineup #78 | [#3 — 1.0 readiness](https://github.com/iancleary/rf-crates/issues/3) |
| gainlineup #79 | [#4 — literature-backed modeling](https://github.com/iancleary/rf-crates/issues/4) |
| linkbudget #84 | [#5 — literature-backed modeling](https://github.com/iancleary/rf-crates/issues/5) |

The fifteen replacements are open and unmerged. Each preserved source head is
recorded in the [machine-readable inventory](../.agents/skills/rehome-rf-work-items/references/work-items.json).
Original review conversations remain on the linked, now-closed source PRs.

| Source PR | Bounded task | Replacement PR | PR base |
| --- | --- | --- | --- |
| [touchstone #88](https://github.com/iancleary/touchstone/pull/88) | [#6](https://github.com/iancleary/rf-crates/issues/6) | [#21 — numerical solves](https://github.com/iancleary/rf-crates/pull/21) | main |
| [touchstone #89](https://github.com/iancleary/touchstone/pull/89) | [#7](https://github.com/iancleary/rf-crates/issues/7) | [#22 — sampled quality](https://github.com/iancleary/rf-crates/pull/22) | main |
| [touchstone #90](https://github.com/iancleary/touchstone/pull/90) | [#8](https://github.com/iancleary/rf-crates/issues/8) | [#23 — mixed mode](https://github.com/iancleary/rf-crates/pull/23) | main |
| [touchstone #91](https://github.com/iancleary/touchstone/pull/91) | [#9](https://github.com/iancleary/rf-crates/issues/9) | [#24 — renormalization](https://github.com/iancleary/rf-crates/pull/24) | #21 |
| [touchstone #92](https://github.com/iancleary/touchstone/pull/92) | [#10](https://github.com/iancleary/rf-crates/issues/10) | [#25 — port connection](https://github.com/iancleary/rf-crates/pull/25) | #21 |
| [touchstone #93](https://github.com/iancleary/touchstone/pull/93) | [#11](https://github.com/iancleary/rf-crates/issues/11) | [#26 — experimental vector fitting](https://github.com/iancleary/rf-crates/pull/26) | main |
| [gainlineup #80](https://github.com/iancleary/gainlineup/pull/80) | [#12](https://github.com/iancleary/rf-crates/issues/12) | [#27 — literature scope](https://github.com/iancleary/rf-crates/pull/27) | main |
| [gainlineup #81](https://github.com/iancleary/gainlineup/pull/81) | [#13](https://github.com/iancleary/rf-crates/issues/13) | [#28 — spectral noise](https://github.com/iancleary/rf-crates/pull/28) | #27 |
| [gainlineup #82](https://github.com/iancleary/gainlineup/pull/82) | [#14](https://github.com/iancleary/rf-crates/issues/14) | [#29 — passive temperature](https://github.com/iancleary/rf-crates/pull/29) | #28 |
| [linkbudget #85](https://github.com/iancleary/linkbudget/pull/85) | [#15](https://github.com/iancleary/rf-crates/issues/15) | [#30 — performance profiles](https://github.com/iancleary/rf-crates/pull/30) | main |
| [linkbudget #86](https://github.com/iancleary/linkbudget/pull/86) | [#16](https://github.com/iancleary/rf-crates/issues/16) | [#31 — atmospheric noise](https://github.com/iancleary/rf-crates/pull/31) | #30 |
| [linkbudget #87](https://github.com/iancleary/linkbudget/pull/87) | [#17](https://github.com/iancleary/rf-crates/issues/17) | [#32 — rain attenuation](https://github.com/iancleary/rf-crates/pull/32) | #31 |
| [linkbudget #88](https://github.com/iancleary/linkbudget/pull/88) | [#18](https://github.com/iancleary/rf-crates/issues/18) | [#33 — margin uncertainty](https://github.com/iancleary/rf-crates/pull/33) | #32 |
| [linkbudget #89](https://github.com/iancleary/linkbudget/pull/89) | [#19](https://github.com/iancleary/rf-crates/issues/19) | [#34 — Gaussian tails](https://github.com/iancleary/rf-crates/pull/34) | #33 |
| [linkbudget #90](https://github.com/iancleary/linkbudget/pull/90) | [#20](https://github.com/iancleary/rf-crates/issues/20) | [#35 — finite-blocklength planning](https://github.com/iancleary/rf-crates/pull/35) | #34 |

The four rfconversions proposals
[#36](https://github.com/iancleary/rfconversions/pull/36),
[#37](https://github.com/iancleary/rfconversions/pull/37),
[#38](https://github.com/iancleary/rfconversions/pull/38), and
[#39](https://github.com/iancleary/rfconversions/pull/39) were already integrated
and released in 0.8.0. Their originals were closed with release evidence, not
replaced with duplicate implementation PRs.

All four original repositories have zero open issues/PRs, a prominent handoff
notice, and a manually disabled legacy CI/publication workflow. They remain
unarchived so historical releases and discussions stay available. No credentials
were moved or revoked by this handoff.

## Bounded follow-up

These are explicit remaining tasks, not claims that controls or RF changes have
already been implemented. Settings changes, archival, and publication require
separate operator authorization.

| Task | Outcome |
| --- | --- |
| [#36](https://github.com/iancleary/rf-crates/issues/36) | Define and verify main-branch protection around existing workspace CI |
| [#37](https://github.com/iancleary/rf-crates/issues/37) | Protect the registry publication boundary and review credential ownership |
| [#38](https://github.com/iancleary/rf-crates/issues/38) | Record retain-or-archive decisions for the four original repositories |
| [#39](https://github.com/iancleary/rf-crates/issues/39) | Refresh immutable packaged READMEs through future documentation-only patch releases |
| [#40](https://github.com/iancleary/rf-crates/issues/40) | Revalidate the three recorded rfconversions correctness findings |
| [#41](https://github.com/iancleary/rf-crates/issues/41) | Revalidate the six recorded Touchstone correctness findings |
| [#42](https://github.com/iancleary/rf-crates/issues/42) | Revalidate the seven recorded gainlineup correctness findings |

## Classification

Use labels as orthogonal selectors, not as substitutes for a written contract:

| Labels | Meaning |
| --- | --- |
| `crate:rfconversions`, `crate:touchstone`, `crate:gainlineup`, `crate:linkbudget` | Owning crate; use more than one only for an explicitly cross-crate change |
| `area:repository` | Shared repository infrastructure rather than RF behavior |
| `type:tracking` | Aggregates decisions and child issues; never implement the whole tracker as one change |
| `type:task` | One independently reviewable outcome with acceptance criteria |
| `status:needs-scoping` | Historical findings need current-source triage and bounded child issues before implementation |
| `status:review` | A bounded proposal exists and requires technical review, not implicit approval |
| `status:blocked` | An explicitly linked predecessor must land or be otherwise resolved first |
| `migration:imported` | Work transferred from an original public crate repository |
| Existing `bug`, `enhancement`, `documentation` | Nature of the change; retained where appropriate |

A task body must state **Scope**, **Non-goals**, **Dependencies**, and
**Acceptance criteria**. Historical verification is attributed to the original
PR; monorepo verification is recorded separately. A tracking issue does not
promise all proposed capabilities for 1.0. Findings based on old commits must be
rechecked before they become current defects or release blockers.

The graph is recorded in GitHub itself, not only in prose: eighteen native
parent/child links connect the fifteen proposal tasks and three readiness
reviews to their trackers. Nine native `blocked by` edges encode the proposal
predecessors. Labels make those same relationships searchable.

## Dependency preservation

- Touchstone numerical solves precede real-reference renormalization and
  arbitrary port connection. Sampled quality, mixed-mode views, and experimental
  fitting are independent proposals.
- Gainlineup literature scope precedes spectral-noise integration, which precedes
  temperature-aware passive loss.
- Linkbudget preserves the six-step sequence: sourced DVB-S2 profiles,
  atmospheric sky noise, rain attenuation, conditional Monte Carlo uncertainty,
  Gaussian tails, then finite-blocklength planning.

A child PR targets its migrated predecessor branch. Retarget it to main only
after that predecessor lands and its diff is reviewed again. Use history-preserving
merges for these stacks: squashing or rebasing would discard the ancestry that
the handoff audit proves. Remove `status:blocked` only when the recorded dependency
is resolved. Ordinary CI runs for all PR target branches; publication remains
confined to approved releases.

## Verification and recovery

Native issue transfer preserves the issue record. PRs cannot be transferred in
that manner: preserve the original commits through a subtree merge, adapt only
workspace integration, and open a linked review-ready replacement. Never close a source
PR merely because a branch was pushed. Verify the replacement URL, intended
base, source-head ancestry, changed-file boundary, and local and hosted check
results first.

All fifteen proposal branches passed full workspace `just ci` with offline Cargo
resolution and a target directory local to each worktree. Each hosted `verify`
check passed, including checks for non-main PR bases. Source content was compared
with the captured original head; differences are confined to crate placement and
README/manifest/lockfile integration.

The retained Touchstone examples, gainlineup spectral-noise example and reference
script, and linkbudget receiver workflow and Gaussian/finite-blocklength reference
scripts were also run. The receiver example's format, Clippy, tests, real CLI, and
independent CSV verifier passed on its three applicable proposal branches.
Plot generation was not exercised.

One integration correction was needed in PR #33: the excluded receiver example
still reported that it belonged to the parent workspace. Giving that existing
non-publishable example an explicit `[workspace]` with resolver 2 and regenerating
its lockfile fixed the real Cargo invocation. The root still has exactly four
public members; the standalone example resolves their current local versions.
For example, on the PR #33 branch:

```bash
cd crates/linkbudget/examples/montecarlo-cn0-target
export CARGO_TARGET_DIR="$(git rev-parse --show-toplevel)/target"
cargo run --offline --locked
uv run --offline --no-project python verify_output.py
```

A migration check is not independent RF-model approval, hardware validation,
MSRV certification, or proof that all independent proposal branches can be
combined without integration work. The conditional Monte Carlo example does
not establish calibrated deployment availability.

Run the [read-only audit](../.agents/skills/rehome-rf-work-items/SKILL.md) from the
workspace root:

```bash
uv run --offline --no-project .agents/skills/rehome-rf-work-items/scripts/verify_migration.py --phase complete
```

It checks label meanings, transferred issue identities, bounded contracts, native
relationships, PR bases and source ancestry, hosted checks, closure links,
integrated release ancestry, source notices, and disabled old workflows. This is
a handoff-state audit, not a permanent gate against intentionally merging the
proposals later. The completed audit passed; a read-only negative check with a
deliberately incorrect parent in a local inventory copy correctly failed.

If a permissions or API failure interrupts the handoff, inspect the actual
remote state before retrying. Do not create duplicate issues or PRs to overcome
an ambiguous response. Issue-list results briefly lagged successful creations
and closures during this handoff; read the individual item by its returned URL
before treating it as missing or retrying a write. A blocked check remains visible;
migration is not a claim
that the proposed RF model is approved or that all historical test claims were
reproduced.

Historical version/commit links remain pointed at their original evidence.
Current development links move to `rf-crates`. Editing a repository README does
not change the README bundled in an immutable published crate; refreshed text
can accompany a separately authorized future patch release.

## Decision trail

1. Preserve the five broad issues as historical trackers, not oversized execution
   tasks. Give pending PR proposals their own acceptance-bounded child issues.
2. Preserve the original PR commit graph and dependency ordering; do not flatten
   stacked proposals onto main or merge them during migration.
3. Keep original repositories available for old release artifacts and review
   discussions. Disable obsolete publishing after handoff; archival is deferred.
4. Preserve original review-ready status after proving monorepo compatibility.
   `status:blocked` and native dependencies represent sequencing, not a draft
   marker that hides an otherwise reviewable proposal.
5. Make the standalone receiver example own its workspace boundary rather than
   adding another public member or changing the proposed RF calculations.
