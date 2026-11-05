"""Exercise the real shell gate with isolated tool stand-ins, including failures."""

import os
import shutil
import subprocess
import tempfile
from pathlib import Path

from check_advisories import MAX_AGE, check_snapshot

ROOT = Path(__file__).resolve().parent.parent
TOOLS = (
    "cargo",
    "rustc",
    "rustfmt",
    "python3",
    "ruff",
    "shellcheck",
    "actionlint",
    "cargo-audit",
    "cargo-deny",
    "git",
    "nix",
)


def main():
    now = 2_000_000_000
    check_snapshot(str(now), now)
    check_snapshot(str(now - MAX_AGE), now)
    for invalid in ("", "bad", "0", str(now - MAX_AGE - 1), str(now + 86401)):
        try:
            check_snapshot(invalid, now)
        except ValueError:
            pass
        else:
            raise AssertionError(f"invalid advisory timestamp accepted: {invalid}")
    with tempfile.TemporaryDirectory(prefix="rust-gdp-gate-") as directory:
        folder = Path(directory)
        log = folder / "calls"
        for tool in TOOLS:
            binary = folder / tool
            binary.write_text(
                "#!/bin/sh\n"
                f'printf "%s\\n" "{tool} $*" >> "$GATE_LOG"\n'
                f'case "{tool} $*" in\n'
                '  "${GATE_FAIL:-never-match}"*) exit 42 ;;\n'
                "esac\n"
            )
            binary.chmod(0o755)
        os.symlink(shutil.which("dirname"), folder / "dirname")
        bash = shutil.which("bash")
        environment = {
            **os.environ,
            "PATH": str(folder),
            "GATE_LOG": str(log),
            "RUST_GDP_NIX_SHELL": "1",
            "RUSTSEC_ADVISORY_DB": str(folder / "pinned-db"),
            "RUSTSEC_ADVISORY_DB_TIMESTAMP": str(now),
        }

        def run(*arguments, **changes):
            log.write_text("")
            result = subprocess.run(
                [bash, str(ROOT / "ci.sh"), *arguments],
                env={**environment, **changes},
                capture_output=True,
                text=True,
                check=False,
            )
            return result, log.read_text()

        result, calls = run("--offline")
        assert result.returncode == 0, result.stderr
        assert "cargo audit --no-fetch" in calls
        assert "cargo deny --offline --locked --all-features check bans licenses sources" in calls
        assert "shellcheck ci.sh scripts/verify.sh" in calls
        assert "cargo test --release" in calls
        assert "cargo package" in calls
        assert "python3 scripts/test_package.py" in calls

        result, calls = run()
        assert result.returncode == 0, result.stderr
        assert "cargo fetch --locked" in calls
        assert "cargo audit --db" in calls and "--no-fetch" not in calls

        # Every stage must stop the build when its underlying tool fails.
        for stage in (
            "cargo audit",
            "cargo deny",
            "shellcheck",
            "actionlint",
            "ruff check",
            "cargo fmt",
            "cargo clippy",
            "cargo test",
            "python3 scripts/codegen.py",
            "cargo package",
            "python3 scripts/test_package.py",
        ):
            result, calls = run("--offline", GATE_FAIL=stage)
            assert result.returncode == 42, (stage, result.stderr)
            assert calls.splitlines()[-1].startswith(stage), (stage, calls)

        # Missing security tools and offline databases must fail before tests start.
        audit = folder / "cargo-audit"
        audit.unlink()
        result, calls = run("--offline")
        assert result.returncode != 0 and "cargo-audit" in result.stderr
        assert not calls
        audit.write_text("#!/bin/sh\nexit 0\n")
        audit.chmod(0o755)
        result, calls = run("--offline", RUSTSEC_ADVISORY_DB="")
        assert result.returncode != 0 and not calls
        result, calls = run("--offline", RUSTSEC_ADVISORY_DB_TIMESTAMP="")
        assert result.returncode != 0 and not calls
        result, calls = run("--skip-security")
        assert result.returncode == 2 and not calls
        result, calls = run("--offline", "unexpected")
        assert result.returncode == 2 and not calls

        result, calls = run("--offline", RUST_GDP_NIX_SHELL="")
        assert result.returncode == 0 and calls.startswith("nix ")
        assert "develop --command bash ./ci.sh --offline" in calls
    print("Gate failure, missing-tool, offline, and Nix bootstrap checks passed")


if __name__ == "__main__":
    main()
