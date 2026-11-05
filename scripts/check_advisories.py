"""Reject missing, invalid, future-dated, or stale offline advisory snapshots."""

import sys
import time

MAX_AGE = 30 * 24 * 60 * 60


def check_snapshot(timestamp: str, now: float) -> None:
    try:
        published = int(timestamp)
    except ValueError as error:
        raise ValueError("Invalid RustSec snapshot timestamp") from error
    if published <= 0 or published > now + 24 * 60 * 60:
        raise ValueError("Invalid or future-dated RustSec snapshot timestamp")
    if now - published > MAX_AGE:
        raise ValueError("RustSec snapshot is over 30 days old; run nix flake update rustsec")


if __name__ == "__main__":
    try:
        check_snapshot(sys.argv[1], time.time())
    except (ValueError, IndexError) as error:
        sys.exit(str(error))
