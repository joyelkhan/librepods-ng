#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$SCRIPT_DIR/.."

cat > "$ROOT/com.librepods.LibrePodsNG.json" <<'EOF'
{
  "app-id": "com.librepods.LibrePodsNG",
  "runtime": "org.freedesktop.Platform",
  "runtime-version": "23.08",
  "sdk": "org.freedesktop.Sdk",
  "command": "librepods",
  "modules": [
    {
      "name": "librepods",
      "buildsystem": "cargo",
      "build-options": {
        "build-args": ["--release"]
      },
      "sources": [
        {
          "type": "dir",
          "path": "."
        }
      ]
    }
  ]
}
EOF

flatpak-builder --user --install "$ROOT/build" "$ROOT/com.librepods.LibrePodsNG.json"
