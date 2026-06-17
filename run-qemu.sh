#!/usr/bin/env bash
set -euo pipefail
find_ovmf() {
    for f in /usr/share/edk2/x64/OVMF_CODE.fd /usr/share/ovmf/x64/OVMF_CODE.fd \
              /usr/share/OVMF/OVMF_CODE.fd /usr/share/edk2-ovmf/x64/OVMF_CODE.fd; do
        [[ -f "$f" ]] && echo "$f" && return
    done
    echo ""
}
OVMF=$(find_ovmf)
if [[ -z "$OVMF" ]]; then
    echo "[ERR] OVMF not found. Arch: sudo pacman -S edk2-ovmf"
    exit 1
fi
echo "==> OVMF: ${OVMF}"
qemu-system-x86_64 -machine q35 -m 256M \
    -drive if=pflash,format=raw,readonly=on,file="${OVMF}" \
    -drive format=raw,file=fat:rw:esp \
    -net none -nographic -serial mon:stdio
