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
   Link native sub-issues and `blocked by` dependencies without turning every
   extension into a 1.0 requirement. Read back the stored graph, not only Markdown.
4. Fetch exact source PR heads. In an isolated worktree, use an unsquashed
   subtree merge under the owning crate. A stacked child starts from its migrated
   predecessor, not main. Preserve source commits and adapt only integration.
5. Verify source-head ancestry, the changed-file boundary, and the actual source
   content. Run full workspace CI and relevant retained workflows. Record new
   evidence separately from the original author's historical claims.
6. Push a verified branch and create a review-ready PR with the correct base,
   labels, source URL, SHA-bearing bounded issue, and verification. Read back the
   actual PR and successful hosted checks. Recheck that the original head still
   matches the captured SHA before closing it with a destination link. Integrated
   proposals require source-head ancestry in main and the published tag instead
   of duplicate PRs. Preserve all source branches and discussions.
7. Only after every source item is accounted for, add original-repository notices
   and retire obsolete workflows. Keep original histories and discussions.
8. Run the read-only audit and update the handoff checklist. Report blocked checks
   and remaining administrative follow-up separately from completed migration.

## Native issue relationships

Use the returned issue database ID, not its displayed issue number, as the
relationship payload. Inspect existing relationships first and do not replace an
unexpected parent. With reviewed variables identifying existing monorepo issues:

```bash
child_id=$(gh api "repos/$repository/issues/$child" --jq .id)
gh api "repos/$repository/issues/$parent/sub_issues" \
  --method POST -F sub_issue_id="$child_id"
predecessor_id=$(gh api "repos/$repository/issues/$predecessor" --jq .id)
gh api "repos/$repository/issues/$dependent/dependencies/blocked_by" \
  --method POST -F issue_id="$predecessor_id"
```

Read the same relationship endpoints with GET afterward. The audit compares the
native graph to the inventory. After an authorized predecessor merge, review the
child diff before retargeting and updating status; do not squash away preserved
source ancestry or auto-merge the child.

## Verification

From the workspace root:

```bash
uv run --offline --no-project .agents/skills/rehome-rf-work-items/scripts/verify_migration.py --phase issues
uv run --offline --no-project .agents/skills/rehome-rf-work-items/scripts/verify_migration.py --phase proposals
uv run --offline --no-project .agents/skills/rehome-rf-work-items/scripts/verify_migration.py --phase complete
```

The helper resolves its inventory relative to this complete skill folder. It
uses authenticated read-only `gh api`/`gh pr view` requests and performs no remote
writes. It checks labels, issue identities and contracts, native relationships,
open unmerged PRs, ancestry, file boundaries, successful hosted checks, source
closure links, release ancestry, notices, and disabled original workflows.
It checks the recorded handoff state, not future feature correctness. Intentional
retargeting or label evolution after migration requires reviewing the inventory,
not bypassing a failed audit. `--inventory` permits auditing a reviewed copy.

## Failure and recovery

Stop on HTTP 403; do not alter authentication or replace native transfer with a
copy that loses history. After an ambiguous write, inspect GitHub before retrying
and locate the existing item by its source marker. Creation and closure lists
briefly lagged successful writes during this handoff: query the returned item URL
before treating it as missing, and never recreate it merely because a list lags.
Preserve incomplete worktrees
and conflicts for inspection; do not force-push or reset an original repository.
Do not close a source PR while its replacement, provenance, or verification is
missing. An excluded nested example may still need an explicit standalone
`[workspace]`: reproduce with its real Cargo command, keep it non-publishable,
regenerate only the affected lockfile, and rerun the actual example and verifier.
Do not add it to the four public members as a workaround. A deferred roadmap is
not a completed implementation.
