#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.11"
# dependencies = []
# ///
"""Read-only audit of the recorded RF issue and PR handoff against GitHub."""

import argparse
import base64
import json
from pathlib import Path
import subprocess
import sys

DEFAULT_INVENTORY = Path(__file__).resolve().parents[1] / "references/work-items.json"


def api(endpoint: str):
    return json.loads(subprocess.check_output(
        ["gh", "api", endpoint], text=True, timeout=120,
    ))


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def labels(item: dict) -> set[str]:
    return {label["name"] for label in item["labels"]}


def verify(inventory: dict, phase: str) -> dict:
    repository = inventory["repository"]
    prefix = f"repos/{repository}"
    configured = {item["name"]: item for item in api(f"{prefix}/labels?per_page=100")}
    for name, (color, description) in inventory["labels"].items():
        require(name in configured, f"missing label: {name}")
        require(configured[name]["color"].lower() == color.lower(), f"label color: {name}")
        require(configured[name]["description"] == description, f"label meaning: {name}")
    result = {"phase": phase, "trackers": [], "tasks": [], "followups": [], "pulls": [], "integrated": []}
    for tracker in inventory["trackers"]:
        crate, number = tracker["crate"], tracker["number"]
        require(number is not None, f"tracker not transferred: {tracker}")
        item = api(f"{prefix}/issues/{number}")
        source = api(f"repos/iancleary/{crate}/issues/{tracker['source_number']}")
        require(item["id"] == source["id"], f"source issue does not redirect to #{number}")
        require(item["title"] == tracker["title"], f"tracker title differs: #{number}")
        expected = {f"crate:{crate}", "type:tracking", "status:needs-scoping", "migration:imported"}
        require(expected <= labels(item), f"tracker classification differs: #{number}")
        require("## Historical review (preserved)" in item["body"], f"missing source review: #{number}")
        expected_children = {
            entry["issue"] for entry in inventory["proposals"]
            if entry["crate"] == crate and entry["tracker"] == tracker["source_number"]
        } | {entry["number"] for entry in inventory.get("followups", []) if entry["parent"] == number}
        children = api(f"{prefix}/issues/{number}/sub_issues")
        require({entry["number"] for entry in children} == expected_children, f"native child links differ: #{number}")
        result["trackers"].append(item["html_url"])
    for followup in inventory.get("followups", []):
        item = api(f"{prefix}/issues/{followup['number']}")
        require(set(followup["labels"]) <= labels(item), f"follow-up labels differ: {item['html_url']}")
        for heading in ("Scope", "Non-goals", "Dependencies", "Acceptance criteria"):
            require(f"## {heading}" in item["body"], f"follow-up lacks {heading}: {item['html_url']}")
        result["followups"].append(item["html_url"])
    for proposal in inventory["proposals"]:
        crate, number = proposal["crate"], proposal["issue"]
        require(number is not None, f"proposal lacks a bounded issue: {proposal}")
        item = api(f"{prefix}/issues/{number}")
        for heading in ("Scope", "Non-goals", "Dependencies", "Acceptance criteria"):
            require(f"## {heading}" in item["body"], f"#{number} lacks {heading}")
        expected = {f"crate:{crate}", "type:task", "migration:imported", proposal["category"]}
        expected.add("status:blocked" if proposal["predecessor"] else "status:review")
        require(expected <= labels(item), f"task classification differs: #{number}")
        require(proposal["source_commit"] in item["body"], f"source commit absent: #{number}")
        predecessor = next((entry for entry in inventory["proposals"]
                            if entry["crate"] == crate and entry["source_number"] == proposal["predecessor"]), None)
        blocked_by = api(f"{prefix}/issues/{number}/dependencies/blocked_by")
        expected_dependencies = {predecessor["issue"]} if predecessor else set()
        require({entry["number"] for entry in blocked_by} == expected_dependencies, f"native dependency differs: #{number}")
        result["tasks"].append(item["html_url"])
        if phase == "issues":
            continue
        require(proposal["pull"] is not None, f"proposal PR not migrated: {crate}#{proposal['source_number']}")
        pull = api(f"{prefix}/pulls/{proposal['pull']}")
        require(expected - {"type:task"} <= labels(pull), f"PR classification differs: {pull['html_url']}")
        base = predecessor["branch"] if predecessor else "main"
        require(pull["head"]["ref"] == proposal["branch"], f"wrong proposal branch: {pull['html_url']}")
        require(pull["base"]["ref"] == base, f"lost dependency stack: {pull['html_url']}")
        require(pull["state"] == "open" and not pull["draft"] and not pull["merged"],
                f"proposal is not open, unmerged, and review-ready: {pull['html_url']}")
        require(f"Closes #{number}" in pull["body"], f"PR does not own its bounded task: {pull['html_url']}")
        source_url = f"https://github.com/iancleary/{crate}/pull/{proposal['source_number']}"
        require(source_url in pull["body"], f"PR source provenance missing: {pull['html_url']}")
        comparison = api(f"{prefix}/compare/{proposal['source_commit']}...{pull['head']['sha']}")
        require(comparison["status"] in ("ahead", "identical"), f"source head not preserved: {pull['html_url']}")
        files = api(f"{prefix}/pulls/{proposal['pull']}/files?per_page=100")
        require(0 < len(files) < 100, f"empty diff or file audit needs pagination: {pull['html_url']}")
        require(all(entry["filename"].startswith(f"crates/{crate}/") or entry["filename"] == "Cargo.lock"
                    for entry in files), f"proposal exceeds crate boundary: {pull['html_url']}")
        require("## Verification" in pull["body"], f"verification record absent: {pull['html_url']}")
        checks = json.loads(subprocess.check_output(
            ["gh", "pr", "view", str(proposal["pull"]), "--repo", repository,
             "--json", "statusCheckRollup"], text=True, timeout=120,
        ))["statusCheckRollup"]
        require(bool(checks), f"hosted checks missing: {pull['html_url']}")
        require(all(check.get("conclusion", check.get("state")) == "SUCCESS" for check in checks),
                f"hosted checks not successful: {pull['html_url']}")
        if phase == "complete":
            original = api(f"repos/iancleary/{crate}/pulls/{proposal['source_number']}")
            require(original["state"] == "closed" and not original["merged"], f"source PR not retired: {original['html_url']}")
            require(original["head"]["sha"] == proposal["source_commit"], f"source changed after capture: {original['html_url']}")
            comments = api(f"repos/iancleary/{crate}/issues/{proposal['source_number']}/comments?per_page=100")
            require(any(pull["html_url"] in comment["body"] for comment in comments),
                    f"source closure lacks destination link: {original['html_url']}")
        result["pulls"].append(pull["html_url"])
    if phase == "complete":
        for item in inventory["integrated"]:
            original = api(f"repos/iancleary/{item['crate']}/pulls/{item['source_number']}")
            require(original["state"] == "closed" and not original["merged"],
                    f"integrated source PR not retired without merging: {original['html_url']}")
            require(original["head"]["sha"] == item["source_commit"], f"integrated source head changed: {original['html_url']}")
            for target in ("main", "rfconversions-v0.8.0"):
                comparison = api(f"{prefix}/compare/{item['source_commit']}...{target}")
                require(comparison["status"] in ("ahead", "identical"),
                        f"integrated head missing from {target}: {original['html_url']}")
            comments = api(f"repos/iancleary/{item['crate']}/issues/{item['source_number']}/comments?per_page=100")
            release_url = f"https://github.com/{repository}/releases/tag/rfconversions-v0.8.0"
            require(any(release_url in comment["body"] for comment in comments),
                    f"integrated closure lacks release evidence: {original['html_url']}")
            result["integrated"].append(original["html_url"])
        for crate in ("rfconversions", "touchstone", "gainlineup", "linkbudget"):
            original = f"repos/iancleary/{crate}"
            require(not api(f"{original}/issues?state=open&per_page=100"), f"unaccounted open work remains: {crate}")
            workflows = api(f"{original}/actions/workflows")["workflows"]
            require(bool(workflows) and all(workflow["state"] == "disabled_manually" for workflow in workflows),
                    f"legacy workflows not manually disabled: {crate}")
            require(not api(original)["archived"], f"repository archived outside this handoff: {crate}")
            readme = api(f"{original}/contents/README.md")
            text = base64.b64decode(readme["content"]).decode()
            require("<!-- rf-crates-repository-handoff -->" in text[:1200]
                    and "https://github.com/iancleary/rf-crates" in text[:1200], f"migration notice missing: {crate}")
    return result


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--inventory", type=Path, default=DEFAULT_INVENTORY)
    parser.add_argument("--phase", choices=("issues", "proposals", "complete"), default="complete")
    arguments = parser.parse_args()
    try:
        result = verify(json.loads(arguments.inventory.read_text()), arguments.phase)
        print(json.dumps(result, indent=2, sort_keys=True))
        return 0
    except (ValueError, KeyError, OSError, subprocess.SubprocessError) as error:
        print(f"Migration audit failed: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
