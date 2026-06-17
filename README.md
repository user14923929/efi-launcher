# EFI Launcher

Minimal UEFI bootloader in Rust — launches `.efi` files from a menu defined in a config.  
Like Ventoy, but for EFI binaries instead of ISO images.

```
┌────────────────────────────────────────────────────────────────────────────────┐
│          EFI Launcher  v0.1  |  github.com/you/efi-launcher                    │
├────────────────────────────────────────────────────────────────────────────────┤
│                                                                                │
│  > M  Memtest86+ — RAM diagnostics             \tools\memtest86.efi            │
│    L  GRUB → System              \tools\grubx64.efi                            │
│    W  Windows Boot Manager                     \tools\bootmgfw.efi             │
│    $  UEFI Shell                               \tools\shellx64.efi             │
│  ─────────────────────────────────────────────────────────────────────────     │
│    ↺  Reboot                                                                   │
│    ⏻  Shutdown                                                                 │
│                                                                                │
│  ↑↓  select     Enter  launch     R  reboot     S  shutdown                    │
└────────────────────────────────────────────────────────────────────────────────┘
```

## Features

- Text TUI menu (SimpleTextOutput — works on any UEFI)
- Config `launcher.cfg` in simplified TOML format directly on the ESP
- Timeout with auto-selection of the default entry
- Hotkeys R (reboot) / S (shutdown)
- Launch via standard `LoadImage` + `StartImage`
- Builds into ~30 KB EFI binary

## ESP layout

```
ESP/
├── EFI/
│   └── BOOT/
│       └── BOOTX64.EFI   ← launcher (fallback bootloader)
├── launcher.cfg           ← config
└── tools/
    ├── memtest86.efi
    ├── grubx64.efi
    └── shellx64.efi
```

## launcher.cfg

```toml
timeout = 10    # seconds before auto-selection (0 = wait forever)
default = 0     # default entry index

[[entry]]
title = "Memtest86+"
path  = "\\tools\\memtest86.efi"
icon  = "M"

[[entry]]
title = "GRUB → Linux"
path  = "\\tools\\grubx64.efi"
icon  = "L"
```

## Build

Requires **Rust nightly** (for `build-std`):

```bash
rustup toolchain install nightly
rustup target add x86_64-unknown-uefi --toolchain nightly
rustup component add rust-src --toolchain nightly
```

Build and copy to `esp/`:

```bash
chmod +x build.sh
./build.sh
```

## QEMU test

```bash
# Arch/CachyOS
sudo pacman -S qemu-system-x86 edk2-ovmf

chmod +x run-qemu.sh
./run-qemu.sh
```

## Write to USB

```bash
# Mount the ESP partition on the flash drive
sudo mount /dev/sdX1 /mnt/esp

# Copy files
sudo cp -r esp/* /mnt/esp/

sudo umount /mnt/esp
```

## CI

GitHub Actions builds the EFI binary on every push to `main`  
and publishes `efi-launcher.zip` as a release artifact.

## License

MIT
