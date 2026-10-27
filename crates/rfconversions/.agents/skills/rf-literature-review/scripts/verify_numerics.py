#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.11"
# dependencies = []
# ///
"""Run the retained numerical comparisons from the rfconversions crate directory."""

import json
from pathlib import Path
import subprocess
import sys
import tempfile


def main() -> None:
    scripts = Path(__file__).resolve().parent
    metadata = json.loads(subprocess.check_output(
        ["cargo", "metadata", "--format-version", "1", "--no-deps", "--offline", "--locked"],
        text=True,
    ))
    subprocess.run(
        ["cargo", "build", "-p", "rfconversions", "--lib", "--offline", "--locked"],
        check=True,
    )
    target = Path(metadata["target_directory"]) / "debug"
    with tempfile.TemporaryDirectory(prefix="rf-numerical-verification-") as directory:
        for name in ("noise", "power", "y_factor"):
            executable = Path(directory) / f"compare_{name}"
            subprocess.run([
                "rustc", "--edition", "2021", str(scripts / f"compare_{name}.rs"),
                "--extern", f"rfconversions={target / 'librfconversions.rlib'}",
                "-L", f"dependency={target / 'deps'}", "-o", str(executable),
            ], check=True)
            references = subprocess.check_output(
                [sys.executable, str(scripts / f"{name}_references.py")], text=True,
            )
            subprocess.run([str(executable)], input=references, text=True, check=True)
    subprocess.run([
        "cargo", "test", "-p", "rfconversions", "--offline", "--locked",
        "--test", "noise_conversion_accuracy", "--test", "power_addition",
        "--test", "y_factor_calibration",
    ], check=True)


if __name__ == "__main__":
    main()
