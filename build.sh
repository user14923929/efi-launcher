#!/usr/bin/env bash
# build.sh — собирает EFI Launcher и копирует в esp/EFI/BOOT/
set -euo pipefail

TARGET="x86_64-unknown-uefi"
OUT="target/${TARGET}/release/efi-launcher.efi"
DEST="esp/EFI/BOOT/BOOTX64.EFI"

echo "==> Сборка..."
cargo build --release -Z build-std=core,compiler_builtins,alloc \
    -Z build-std-features=compiler-builtins-mem \
    --target "${TARGET}"

echo "==> Копирование ${OUT} → ${DEST}"
mkdir -p esp/EFI/BOOT
cp "${OUT}" "${DEST}"

echo "==> Готово: ${DEST}"
echo ""
echo "Для запуска в QEMU:"
echo "  ./run-qemu.sh"
