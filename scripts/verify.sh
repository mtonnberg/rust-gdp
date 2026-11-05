#!/usr/bin/env bash
# Compatibility entry point; ci.sh owns the complete gate.
set -euo pipefail
exec bash "$(dirname "$0")/../ci.sh" "$@"
