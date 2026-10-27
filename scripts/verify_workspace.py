#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.11"
# dependencies = []
# ///
"""Check the four-crate workspace boundary and local dependency resolution."""

import json
from pathlib import Path
import subprocess
import sys
import tomllib


ROOT = Path(__file__).resolve().parents[1]
CRATES = {"rfconversions", "touchstone", "gainlineup", "linkbudget"}


def verify() -> None:
    result = subprocess.run(
        ["cargo", "metadata", "--format-version", "1", "--locked", "--offline"],
        cwd=ROOT,
        check=True,
        capture_output=True,
        text=True,
    )
    metadata = json.loads(result.stdout)
    packages = {package["id"]: package for package in metadata["packages"]}
    members = [packages[identifier] for identifier in metadata["workspace_members"]]
    if len(members) != 4 or {package["name"] for package in members} != CRATES:
        raise ValueError("workspace must contain exactly the four approved RF crates")
    if Path(metadata["workspace_root"]).resolve() != ROOT:
        raise ValueError("Cargo resolved a different workspace root")
    for package in members:
        directory = ROOT / "crates" / package["name"]
        if Path(package["manifest_path"]).resolve() != directory / "Cargo.toml":
            raise ValueError(f"unexpected package location: {package['name']}")
        if package["license"] != "MIT" or not (directory / "LICENSE").is_file():
            raise ValueError(f"missing MIT license: {package['name']}")
        if (directory / "Cargo.lock").exists():
            raise ValueError(f"obsolete member lockfile: {package['name']}")
    for package in packages.values():
        if package["name"] in CRATES and package["id"] not in metadata["workspace_members"]:
            raise ValueError(f"registry copy of workspace crate: {package['name']}")
    manifest = tomllib.loads((ROOT / "Cargo.toml").read_text())
    for name, dependency in manifest["workspace"]["dependencies"].items():
        if name in CRATES and not {"path", "version"} <= dependency.keys():
            raise ValueError(f"{name} requires both local path and registry version")
    print("Verified exactly four workspace crates, local RF dependencies, and licenses.")


def main() -> int:
    try:
        verify()
    except (ValueError, OSError, subprocess.CalledProcessError) as error:
        print(f"Workspace verification failed: {error}", file=sys.stderr)
        if isinstance(error, subprocess.CalledProcessError) and error.stderr:
            print(error.stderr, file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
