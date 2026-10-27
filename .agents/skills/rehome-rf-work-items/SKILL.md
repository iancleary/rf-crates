---
name: rehome-rf-work-items
description: Rehome or audit the original public RF repositories' issues and dependent PRs using native GitHub operations, bounded task contracts, and the recorded source-to-destination inventory.
---

# Rehome RF work items

Read [the handoff contract](../../../docs/repository-migration.md) and
[the inventory](references/work-items.json). Require explicit authorization for
issue/PR writes, source notices, workflow retirement, and branch publication.
Migration does not authorize feature merges, releases, or archival.

## Sequence

1. Capture current issue bodies, PR heads, bases, and labels before mutation.
   Recheck source heads before import; stop on unexpected concurrent changes.
2. Establish the inventory's label meanings. Transfer issues with `gh issue
   transfer`, not copied replacement issues. Preserve historical findings and
   authorship; qualify bare source-repository issue references after transfer.
3. Classify broad reviews as tracking/needs-scoping. Give each pending proposal
   one task with scope, non-goals, dependencies, and observable acceptance.
   Link children without turning every extension into a 1.0 requirement.
4. Fetch exact source PR heads. In an isolated worktree, use an unsquashed
   subtree merge under the owning crate. A stacked child starts from its migrated
   predecessor, not main. Preserve source commits and adapt only integration.
5. Verify source-head ancestry, the changed-file boundary, and the actual source
   content. Run full workspace CI and relevant retained workflows. Record new
   evidence separately from the original author's historical claims.
6. Push a verified branch and create a review-ready PR with the correct base, labels,
   bounded issue, source URL/SHA, and verification. Read back the actual PR before
   closing the original with a destination link. Integrated proposals require
   source-head ancestry and release evidence instead of duplicate PRs.
7. Only after every source item is accounted for, add original-repository notices
   and retire obsolete workflows. Keep original histories and discussions.
8. Run the read-only audit and update the handoff checklist. Report blocked checks
   and remaining administrative follow-up separately from completed migration.

## Verification

From the workspace root:

```bash
uv run --offline --no-project .agents/skills/rehome-rf-work-items/scripts/verify_migration.py --phase issues
uv run --offline --no-project .agents/skills/rehome-rf-work-items/scripts/verify_migration.py --phase proposals
uv run --offline --no-project .agents/skills/rehome-rf-work-items/scripts/verify_migration.py --phase complete
```

The helper resolves its inventory relative to this complete skill folder. It
uses authenticated read-only `gh api` requests and performs no remote writes.
It checks the recorded handoff state, not future feature correctness. Intentional
retargeting or label evolution after migration requires reviewing the inventory,
not bypassing a failed audit. `--inventory` permits auditing a reviewed copy.

## Failure and recovery

Stop on HTTP 403; do not alter authentication or replace native transfer with a
copy that loses history. After an ambiguous write, inspect GitHub before retrying
and locate the existing item by its source marker. Preserve incomplete worktrees
and conflicts for inspection; do not force-push or reset an original repository.
Do not close a source PR while its replacement, provenance, or verification is
missing. A deferred roadmap is not a completed implementation.
