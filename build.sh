#!/usr/bin/env bash
set -euo pipefail
TARGET="x86_64-unknown-uefi"
OUT="target/${TARGET}/release/efi-launcher.efi"
DEST="esp/EFI/BOOT/BOOTX64.EFI"
echo "==> Building..."
cargo build --release -Z build-std=core,compiler_builtins,alloc \
    -Z build-std-features=compiler-builtins-mem --target "${TARGET}"
echo "==> Copying ${OUT} -> ${DEST}"
mkdir -p esp/EFI/BOOT
cp "${OUT}" "${DEST}"
echo "==> Done! To run: ./run-qemu.sh"
