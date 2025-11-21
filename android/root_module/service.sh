#!/system/bin/sh
set -euo pipefail

MODDIR=${0%/*}

mkdir -p /system/priv-app/LibrePods
cp "$MODDIR/LibrePods.apk" /system/priv-app/LibrePods/
chmod 644 /system/priv-app/LibrePods/LibrePods.apk
chcon u:object_r:system_file:s0 /system/priv-app/LibrePods/LibrePods.apk

echo "allow bluetooth_manager librepods:bluetooth_socket { read write };" >> "$MODDIR/sepolicy.rule"
