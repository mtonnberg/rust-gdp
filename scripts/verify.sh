#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked --offline -- -D warnings
cargo test --locked --offline
cargo test --no-default-features --locked --offline
cargo test --release --locked --offline
RUSTDOCFLAGS="${RUSTDOCFLAGS:-} -D warnings" cargo doc --no-deps --all-features --locked --offline
python3 scripts/codegen.py
cargo package --allow-dirty --locked --offline
