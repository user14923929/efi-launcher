#!/usr/bin/env bash
# run-qemu.sh — запускает лаунчер в QEMU с OVMF (без реального железа)
set -euo pipefail

# Ищем OVMF в стандартных местах для Arch/Fedora/Ubuntu
find_ovmf() {
    local candidates=(
        "/usr/share/OVMF/OVMF_CODE.fd"               # Ubuntu/Debian
        "/usr/share/edk2/x64/OVMF_CODE.fd"           # Fedora / CachyOS
        "/usr/share/ovmf/x64/OVMF_CODE.fd"           # Arch
        "/usr/share/edk2-ovmf/x64/OVMF_CODE.fd"      # некоторые Arch-пакеты
    )
    for f in "${candidates[@]}"; do
        if [[ -f "$f" ]]; then
            echo "$f"
            return
        fi
    done
    echo ""
}

OVMF=$(find_ovmf)

if [[ -z "$OVMF" ]]; then
    echo "[ERR] OVMF не найден. Установи пакет:"
    echo "  Arch/CachyOS : sudo pacman -S edk2-ovmf"
    echo "  Fedora       : sudo dnf install edk2-ovmf"
    echo "  Ubuntu       : sudo apt install ovmf"
    exit 1
fi

echo "==> OVMF: ${OVMF}"
echo "==> Запуск QEMU..."

qemu-system-x86_64 \
    -machine q35 \
    -m 256M \
    -drive if=pflash,format=raw,readonly=on,file="${OVMF}" \
    -drive format=raw,file=fat:rw:esp \
    -net none \
    -nographic \
    -serial mon:stdio
