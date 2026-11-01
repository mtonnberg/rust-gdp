"""Run consumer tests from the package Cargo actually produced."""

import json
import subprocess
from pathlib import Path

metadata = json.loads(
    subprocess.check_output(
        ["cargo", "metadata", "--no-deps", "--format-version", "1", "--locked", "--offline"],
        text=True,
    )
)
package = metadata["packages"][0]
manifest = (
    Path(metadata["target_directory"])
    / "package"
    / f"{package['name']}-{package['version']}"
    / "Cargo.toml"
)
subprocess.run(
    [
        "cargo",
        "test",
        "--manifest-path",
        str(manifest),
        "--all-features",
        "--locked",
        "--offline",
    ],
    check=True,
)
