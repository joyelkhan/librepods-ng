#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$SCRIPT_DIR/.."
OUT="$ROOT/target/deb"

cargo build --release --target x86_64-unknown-linux-gnu
mkdir -p "$OUT/DEBIAN" "$OUT/usr/bin"

cp "$ROOT/target/x86_64-unknown-linux-gnu/release/librepods" "$OUT/usr/bin/"
chmod 755 "$OUT/usr/bin/librepods"

cat > "$OUT/DEBIAN/control" <<'EOF'
Package: librepods-ng
Version: 1.0.0
Architecture: amd64
Maintainer: Rivers Engineering <dev@librepods.local>
Description: Unlock AirPods features on non-Apple devices
EOF

dpkg-deb --build "$OUT" "$ROOT/librepods-ng_1.0.0_amd64.deb"
sha256sum "$ROOT/librepods-ng_1.0.0_amd64.deb" > "$ROOT/librepods-ng_1.0.0_amd64.deb.sha256"
