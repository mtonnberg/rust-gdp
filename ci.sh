#!/usr/bin/env bash
# One gate for CI builds and local pre-push checks. Nix is the only prerequisite.
set -euo pipefail
cd "$(dirname "$0")"

if [[ "${RUST_GDP_NIX_SHELL:-}" != 1 ]]; then
  command -v nix >/dev/null || { echo 'Install Nix to run ./ci.sh.' >&2; exit 1; }
  exec nix --extra-experimental-features 'nix-command flakes' develop --command bash ./ci.sh "$@"
fi

if (( $# > 1 )) || { (( $# == 1 )) && [[ "$1" != --offline ]]; }; then
  echo 'Usage: ./ci.sh [--offline]' >&2
  exit 2
fi
for tool in cargo rustc rustfmt python3 ruff shellcheck actionlint cargo-audit cargo-deny git; do
  command -v "$tool" >/dev/null || { echo "Missing required gate tool: $tool" >&2; exit 1; }
done

if [[ "${1:-}" == --offline ]]; then
  : "${RUSTSEC_ADVISORY_DB:?Offline checks require the pinned RustSec database from Nix.}"
  : "${RUSTSEC_ADVISORY_DB_TIMESTAMP:?Offline checks require the pinned RustSec timestamp from Nix.}"
  python3 scripts/check_advisories.py "$RUSTSEC_ADVISORY_DB_TIMESTAMP"
  cargo audit --no-fetch --db "$RUSTSEC_ADVISORY_DB" --deny warnings
else
  cargo fetch --locked
  cargo audit --db "${CARGO_TARGET_DIR:-target}/rustsec-advisory-db" --deny warnings
fi
cargo deny --offline --locked --all-features check bans licenses sources

shellcheck ci.sh scripts/*.sh
actionlint .github/workflows/ci.yml
ruff check scripts
ruff format --check scripts
cargo fmt --all -- --check
rustfmt --check --edition 2021 scripts/codegen.rs
cargo clippy --all-targets --all-features --locked --offline -- -D warnings
cargo clippy --all-targets --no-default-features --locked --offline -- -D warnings
cargo test --all-features --locked --offline
cargo test --no-default-features --locked --offline
cargo test --release --all-features --locked --offline
cargo test --release --no-default-features --locked --offline
RUSTDOCFLAGS="${RUSTDOCFLAGS:-} -D warnings" cargo doc --no-deps --all-features --locked --offline
python3 scripts/test_gate.py
python3 scripts/codegen.py
cargo run --example validated_config --locked --offline
cargo package --allow-dirty --locked --offline
python3 scripts/test_package.py
