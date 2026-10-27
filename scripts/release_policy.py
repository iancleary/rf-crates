#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.11"
# dependencies = []
# ///
"""Prepare workspace versions and enforce bounded, crate-specific release checks."""

import argparse
import hashlib
import json
import re
import subprocess
import sys
import time
import tomllib
from pathlib import Path
from urllib.error import HTTPError, URLError
from urllib.request import Request, urlopen

ROOT = Path(__file__).resolve().parents[1]
CRATES = ("rfconversions", "touchstone", "gainlineup", "linkbudget")
REPOSITORY = "https://github.com/iancleary/rf-crates"
VERSION = re.compile(r"(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)")


def command(*arguments: str) -> str:
    return subprocess.check_output(arguments, cwd=ROOT, text=True).strip()


def manifest(crate: str) -> Path:
    return ROOT / "crates" / crate / "Cargo.toml"


def package_version(crate: str) -> str:
    return tomllib.loads(manifest(crate).read_text())["package"]["version"]


def stable_version(version: str) -> None:
    if VERSION.fullmatch(version) is None:
        raise ValueError("this release contract requires an exact stable X.Y.Z version")


def registry_entry(crate: str, version: str, timeout: float = 30) -> dict | None:
    request = Request(
        f"https://index.crates.io/{crate[:2]}/{crate[2:4]}/{crate}",
        headers={
            "User-Agent": f"rf-crates-release ({REPOSITORY})",
            "Cache-Control": "no-cache",
        },
    )
    try:
        with urlopen(request, timeout=timeout) as response:
            for line in response:
                entry = json.loads(line)
                if entry["vers"] == version:
                    return entry
    except HTTPError as error:
        if error.code != 404:
            raise
    return None


def dependencies(crate: str) -> dict[str, str]:
    package = tomllib.loads(manifest(crate).read_text())
    workspace = tomllib.loads((ROOT / "Cargo.toml").read_text())
    workspace_dependencies = workspace["workspace"]["dependencies"]
    required = {}
    for name, specification in package.get("dependencies", {}).items():
        if name not in CRATES:
            continue
        if isinstance(specification, dict) and specification.get("workspace"):
            specification = workspace_dependencies[name]
        version = (
            specification if isinstance(specification, str)
            else specification["version"]
        )
        stable_version(version)
        required[name] = version
    return required


def validate(crate: str, version: str) -> dict:
    stable_version(version)
    if package_version(crate) != version:
        raise ValueError(f"release version does not match {crate}'s committed manifest")
    remote = command("git", "remote", "get-url", "origin")
    if remote not in (REPOSITORY, REPOSITORY + ".git"):
        raise ValueError("origin is not the approved public RF repository")
    remote_reference = command("git", "ls-remote", "origin", "refs/heads/main").split()
    if not remote_reference:
        raise ValueError("origin/main is missing")
    subprocess.run(
        ["git", "merge-base", "--is-ancestor", "HEAD", remote_reference[0]],
        cwd=ROOT, check=True,
    )
    notes = ROOT / "release" / "notes" / f"{crate}.md"
    heading = f"# {crate} {version}"
    if not notes.is_file() or notes.read_text().splitlines()[:1] != [heading]:
        raise ValueError(f"release/notes/{crate}.md must start with '{heading}'")
    if registry_entry(crate, version) is not None:
        raise ValueError(
            f"{crate} {version} already exists; inspect publication, do not republish"
        )
    required = dependencies(crate)
    for name, required_version in required.items():
        entry = registry_entry(name, required_version)
        if entry is None or entry["yanked"]:
            raise ValueError(
                f"required registry dependency is unavailable: {name} {required_version}"
            )
    return {
        "crate": crate, "version": version,
        "tag": f"{crate}-v{version}", "dependencies": required,
    }


def replace_once(text: str, pattern: str, replacement: str) -> str:
    updated, count = re.subn(pattern, replacement, text, flags=re.MULTILINE)
    if count != 1:
        raise ValueError(f"expected one version declaration, found {count}")
    return updated


def prepare(crate: str, bump: str, apply: bool) -> dict:
    if command("git", "status", "--porcelain"):
        raise ValueError(
            "prepare requires a clean checkout; preserve and commit existing work first"
        )
    current = package_version(crate)
    stable_version(current)
    major, minor, patch = map(int, current.split("."))
    version = {
        "major": f"{major + 1}.0.0",
        "minor": f"{major}.{minor + 1}.0",
        "patch": f"{major}.{minor}.{patch + 1}",
    }[bump]
    if registry_entry(crate, version) is not None:
        raise ValueError(f"{crate} {version} already exists on crates.io")
    paths = [manifest(crate), ROOT / "Cargo.toml", ROOT / "Cargo.lock"]
    result = {
        "crate": crate, "current_version": current, "version": version,
        "files": [str(path.relative_to(ROOT)) for path in paths],
        "mode": "prepare" if apply else "dry-run",
    }
    if not apply:
        return result
    originals = {path: path.read_bytes() for path in paths}
    try:
        path = manifest(crate)
        path.write_text(replace_once(
            path.read_text(), r'^version = "' + re.escape(current) + r'"$',
            f'version = "{version}"',
        ))
        workspace = ROOT / "Cargo.toml"
        if crate in tomllib.loads(workspace.read_text())["workspace"]["dependencies"]:
            workspace.write_text(replace_once(
                workspace.read_text(),
                r'(^' + re.escape(crate) + r' = \{[^\n]*version = ")'
                + re.escape(current) + r'(")',
                r'\g<1>' + version + r'\2',
            ))
        subprocess.run(
            ["cargo", "metadata", "--offline", "--format-version", "1"],
            cwd=ROOT, check=True, stdout=subprocess.DEVNULL,
        )
        subprocess.run(
            ["cargo", "metadata", "--no-deps", "--locked", "--offline",
             "--format-version", "1"],
            cwd=ROOT, check=True, stdout=subprocess.DEVNULL,
        )
    except BaseException:
        for path, original in originals.items():
            path.write_bytes(original)
        raise
    return result


def wait_for_index(
    crate: str, version: str, timeout: int, archive: Path | None,
) -> dict:
    stable_version(version)
    checksum = hashlib.sha256(archive.read_bytes()).hexdigest() if archive else None
    deadline = time.monotonic() + timeout
    while (remaining := deadline - time.monotonic()) > 0:
        entry = registry_entry(crate, version, min(30, remaining))
        if entry:
            if entry["yanked"]:
                raise ValueError(f"{crate} {version} is yanked")
            if checksum and entry["cksum"] != checksum:
                raise ValueError("published package checksum differs from the uploaded archive")
            return {
                "crate": crate, "version": version,
                "indexed": True, "checksum": entry["cksum"],
            }
        print(f"Waiting for {crate} {version} in the crates.io index...", file=sys.stderr)
        time.sleep(min(10, max(0, deadline - time.monotonic())))
    raise ValueError("index wait expired; inspect the workflow and registry before another release")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest="action", required=True)
    preparation = commands.add_parser(
        "prepare", help="preview or prepare local version files; never publishes",
    )
    preparation.add_argument("crate", choices=CRATES)
    preparation.add_argument("--bump", choices=("major", "minor", "patch"), required=True)
    mode = preparation.add_mutually_exclusive_group(required=True)
    mode.add_argument("--dry-run", action="store_true")
    mode.add_argument("--apply", action="store_true")
    validation = commands.add_parser(
        "validate", help="validate an exact prepared release and registry dependencies",
    )
    validation.add_argument("crate", choices=CRATES)
    validation.add_argument("version")
    preflight = commands.add_parser(
        "preflight", help="validate a release tag for the publishing job",
    )
    preflight.add_argument("--tag", required=True)
    preflight.add_argument("--github-output", type=Path)
    waiting = commands.add_parser(
        "wait-index",
        help="wait for registry visibility, optionally checking the archive checksum",
    )
    waiting.add_argument("crate", choices=CRATES)
    waiting.add_argument("version")
    waiting.add_argument("--timeout", type=int, default=600)
    waiting.add_argument("--archive", type=Path)
    args = parser.parse_args()
    try:
        if args.action == "prepare":
            result = prepare(args.crate, args.bump, args.apply)
        elif args.action == "validate":
            result = validate(args.crate, args.version)
        elif args.action == "preflight":
            crate, separator, version = args.tag.partition("-v")
            if not separator or crate not in CRATES:
                raise ValueError("tag must identify one approved crate and an exact version")
            stable_version(version)
            tag_commit = command("git", "rev-parse", f"refs/tags/{args.tag}^{{commit}}")
            if command("git", "rev-parse", "HEAD") != tag_commit:
                raise ValueError("checkout does not match the release tag")
            result = validate(crate, version)
            if args.github_output:
                with args.github_output.open("a") as output:
                    output.write(f"crate={crate}\nversion={version}\n")
        else:
            if not 1 <= args.timeout <= 1200:
                raise ValueError("timeout must be between 1 and 1200 seconds")
            result = wait_for_index(args.crate, args.version, args.timeout, args.archive)
        print(json.dumps(result, sort_keys=True))
        return 0
    except (ValueError, OSError, URLError, subprocess.CalledProcessError) as error:
        print(f"Release policy failed: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
