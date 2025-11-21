#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$SCRIPT_DIR/.."
OUT="$ROOT/target/dmg"

cargo build --release --target x86_64-apple-darwin
mkdir -p "$OUT/LibrePods.app/Contents/MacOS"

cp "$ROOT/target/x86_64-apple-darwin/release/librepods" "$OUT/LibrePods.app/Contents/MacOS/"
chmod 755 "$OUT/LibrePods.app/Contents/MacOS/librepods"

cat > "$OUT/LibrePods.app/Contents/Info.plist" <<'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleExecutable</key>
  <string>librepods</string>
  <key>CFBundleName</key>
  <string>LibrePods NG</string>
  <key>CFBundleVersion</key>
  <string>1.0.0</string>
</dict>
</plist>
EOF

hdiutil create -volname "LibrePods NG" -srcfolder "$OUT" "$ROOT/LibrePods-1.0.0.dmg"
sha256sum "$ROOT/LibrePods-1.0.0.dmg" > "$ROOT/LibrePods-1.0.0.dmg.sha256"
