#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")" && pwd)"
cd "$ROOT"

# Check required tools
for cmd in cargo python3 dpkg; do
    if ! command -v "$cmd" >/dev/null 2>&1; then
        echo "ERROR: Required command '$cmd' is not installed."
        exit 1
    fi
done

# Read package metadata from Cargo.toml
eval "$(cargo metadata --no-deps --format-version 1 | python3 -c '
import json, sys, shlex

data = json.load(sys.stdin)
pkg = next(
    p for p in data["packages"]
    if p["id"] in data["workspace_members"]
)

metadata = pkg.get("metadata") or {}

values = {
    "APP": pkg["name"],
    "VERSION": pkg["version"],
    "APP_ID": metadata["app-id"],
}

for key, value in values.items():
    print(f"{key}={shlex.quote(value)}")
')"

ARCH="$(dpkg --print-architecture)"

RELEASE="$ROOT/target/release"

DESKTOP="data/applications/$APP_ID.desktop"
ICON="data/icons/hicolor/scalable/apps/$APP_ID.svg"

echo "==> Application: $APP"
echo "==> Version: $VERSION"
echo "==> App ID: $APP_ID"
echo "==> Architecture: $ARCH"

# Build Rust only once
echo "==> Building release..."
cargo build --release

# Generate Debian package without recompiling
echo "==> Building DEB..."
cargo deb --no-build \
    --output "$RELEASE/${APP}_${VERSION}_${ARCH}.deb"
