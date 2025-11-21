#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$SCRIPT_DIR/.."
OUT="$ROOT/target/appimage"

cargo build --release --target x86_64-unknown-linux-gnu
mkdir -p "$OUT"
cp "$ROOT/target/x86_64-unknown-linux-gnu/release/librepods" "$OUT/librepods"

cat > "$OUT/librepods.desktop" <<'EOF'
[Desktop Entry]
Name=LibrePods NG
Exec=librepods
Icon=librepods
Type=Application
Categories=AudioVideo;
EOF

curl -L https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage -o "$OUT/appimagetool"
chmod +x "$OUT/appimagetool"
"$OUT/appimagetool" "$OUT" "$ROOT/LibrePods-x86_64.AppImage"
sha256sum "$ROOT/LibrePods-x86_64.AppImage" > "$ROOT/LibrePods-x86_64.AppImage.sha256"
