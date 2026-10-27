# Repository handoff and bounded work items

The four crates and their source history are consolidated; versions rfconversions
0.8.0, touchstone 0.17.0, gainlineup 0.24.0, and linkbudget 0.8.0 are published.
This handoff moves issue ownership and pending proposals, not feature approval.
See [workspace provenance](workspace.md) and [release operations](releases.md).

## Completion contract

- [ ] Transfer the five open issues using GitHub's native transfer operation,
  preserving authorship and discussion history.
- [ ] Preserve fifteen pending PR heads and dependency stacks in monorepo draft
  PRs. Each proposal has one bounded implementation issue and explicit acceptance.
- [ ] Verify the four integrated rfconversions PR heads are ancestors of main,
  then close their original PRs with the monorepo/release evidence.
- [ ] Verify every replacement before closing its original PR. Preserve original
  discussions through source links; do not represent copied reviews as new reviews.
- [ ] Update current README installation/source links and migration guidance.
- [ ] Add original-repository notices and disable their legacy publishing workflows
  only after all open work has a verified destination.
- [ ] Verify the final remote state against the checked-in work-item inventory.

No feature merge, new package publication, history rewrite, or repository archival
is authorized by this procedure. Protection settings and unresolved release
readiness findings remain separately bounded follow-up work.

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
after that predecessor lands and its diff is reviewed again. Ordinary CI runs
for all PR target branches; publication remains confined to approved releases.

## Verification and recovery

Native issue transfer preserves the issue record. PRs cannot be transferred in
that manner: preserve the original commits through a subtree merge, adapt only
workspace integration, and open a linked draft replacement. Never close a source
PR merely because a branch was pushed. Verify the replacement URL, intended
base, source-head ancestry, changed-file boundary, and local check results first.

If a permissions or API failure interrupts the handoff, inspect the actual
remote state before retrying. Do not create duplicate issues or PRs to overcome
an ambiguous response. A blocked check remains visible; migration is not a claim
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
