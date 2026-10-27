---
name: rf-literature-review
description: Evaluate published RF conversion and numerical methods against rfconversions' scalar API and produce sourced, bounded improvement proposals. Use for literature discovery or refreshing formula-quality priorities, not system modeling or automatic implementation.
---

# RF literature to a scalar-conversion production proposal

## Prerequisites and boundaries

- Work in the rfconversions checkout. Read `AGENTS.md`, inspect Git status, and
  preserve unrelated work.
- Read `README.md`, `src/lib.rs`, and `docs/agent-operating-loop.md` before
  selecting the owning module. This skill needs no separate research document.
- A literature request permits public source retrieval, not paid services,
  purchased papers, uploaded measurements, installations, delegation, external
  publication, or runtime changes without the applicable authorization.
- Keep this crate a small leaf dependency with explicit, searchable scalar
  functions. Ordered hardware chains belong in gainlineup, measured S-parameter
  networks in touchstone, and complete radio links in linkbudget. Existing
  narrow cascade helpers do not authorize a system simulator or generic unit API.

## Working sequence

1. Identify one real conversion question and its caller-visible outcome. Start in
   `src/power.rs`, `src/frequency.rs`, `src/noise.rs`, `src/p1db.rs`, or
   `src/constants.rs`; inspect existing integration coverage and record the Git
   baseline. Do not propose a conversion already supplied by the crate.
2. Separate an RF-domain gap from a numerical implementation gap. A familiar
   formula may need better range handling, not a new API. Conversely, a different
   reference convention must not be disguised as a numerical correction.
3. Find primary papers, standards, or authoritative metrology references. Verify
   authors/issuing body, title, year, DOI/document identifier, and revision against
   publisher, institutional, or bibliographic metadata. For constants, record
   definition, units, exact versus measured status, and applicable revision.
4. Inspect the relevant equations and assumptions in legitimate open copies when
   available. Use an available PDF-reading skill or existing tooling without
   implicit installations. Record sections/pages and distinguish inspected text
   from metadata-only, abstract-only, or unread-equation evidence.
5. Specify the smallest useful production slice: owning scalar function, units,
   domain, reference values, error/NaN/infinity behavior, compatibility impact,
   and a concrete consumer example. Evaluate stable elementary formulations such
   as `ln_1p` or `exp_m1` only where their mathematical argument and domain match;
   do not introduce a heavier numerical dependency by default.
6. Audit conventions before comparing numbers:
   - power ratios use `10 log10`, amplitude ratios use `20 log10` only under the
     appropriate impedance assumptions;
   - dBm, dBW, watts, milliwatts, and dimensionless dB have different references;
   - frequency is Hz and vacuum wavelength is meters;
   - NF is logarithmic, F is linear, and equivalent noise temperature uses the
     crate's `constants::T0` reference, currently 290 K;
   - kTB uses physical temperature and noise bandwidth, not an arbitrary NF;
   - true input/output P1dB references must account for compressed versus
     small-signal gain. Do not replace the crate's convention from memory.
7. Define independent acceptance references before recommending a change:
   analytic points, dimensional checks, authoritative constants, and, where
   numerical range matters, independent high-precision values. Include near-zero
   differences, extreme finite values, singular/domain boundaries, and ordinary
   RF operating values. A round trip or shared dependency is insufficient alone.
8. Rank by user value, numerical risk, compatibility, and verification cost.
   Benchmark actual public conversions before claiming speedups. Prefer narrow
   formula improvements over a general unit system or simulation framework.
9. For a substantive review, extend an existing research page if present;
   otherwise add a clearly proposed, source-linked page under `docs/` and link it
   from README. Do not represent a recommendation as implemented functionality
   or duplicate the operational guide with another process document.

## Verification and common failures

- Inspect the intended diff, check local Markdown links, and run
  `git diff --check`. Run Python calculations through `uv run` without implicitly
  installing dependencies. Keep any required skill helper inside this folder,
  with paths resolved relative to the skill directory.
- Existing `tests/readme_examples.rs`, `tests/practical_scenarios.rs`, and
  `tests/frequency_conversions.rs` can establish the current baseline. They do
  not verify a proposed feature. Follow repository gates for any separately
  authorized behavior or public-example change; no new suite is needed for a
  research-only edit.
- Do not claim universal accuracy from finite outputs, cancellation-prone round
  trips, or agreement with the same underlying dependency. Use relative error
  away from zero and appropriate absolute/log-domain error near nulls.
- On a missing paper or failed extraction, use a legitimate alternate source or
  mark the claim incomplete. Do not bypass access controls or reuse reference
  code without checking its license.
- If a candidate requires modulation, path loss, network matrices, or stage
  state, hand it to the owning sibling crate instead of expanding this one.

## Completion and recovery

Report a bounded recommendation, source/access depth, verification evidence,
compatibility concerns, and unresolved assumptions. Keep runtime behavior intact
until an implementation is separately authorized. Research does not authorize
commits, pushes, releases, or resets of unrelated work. Remove or demote claims
when evidence fails or the current API already solves the problem.
