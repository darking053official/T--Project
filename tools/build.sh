#!/usr/bin/env bash
set -e

echo "[+] Build workspace"
cargo build

chmod +x tools/build.sh
