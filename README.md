# EFI Launcher

Минималистичный UEFI-загрузчик на Rust — запускает `.efi`-файлы по меню из конфига.  
Аналог Ventoy, но для EFI-бинарников вместо ISO-образов.

```
┌────────────────────────────────────────────────────────────────────────────────┐
│          EFI Launcher  v0.1  |  github.com/you/efi-launcher                    │
├────────────────────────────────────────────────────────────────────────────────┤
│                                                                                │
│  > M  Memtest86+ — диагностика RAM          \tools\memtest86.efi               │
│    L  GRUB → Arch Linux (CachyOS)           \tools\grubx64.efi                 │
│    W  Windows Boot Manager                  \tools\bootmgfw.efi                │
│    $  UEFI Shell                            \tools\shellx64.efi                │
│  ─────────────────────────────────────────────────────────────────────────     │
│    ↺  Перезагрузить                                                            │
│    ⏻  Выключить                                                                │
│                                                                                │
│  ↑↓  выбор     Enter  запустить     R  перезагрузить     S  выключить         │
└────────────────────────────────────────────────────────────────────────────────┘
```

## Возможности

- Текстовое TUI меню (SimpleTextOutput — работает на любом UEFI)
- Конфиг `launcher.cfg` в формате упрощённого TOML прямо на ESP
- Таймаут с автовыбором записи по умолчанию
- Быстрые клавиши R (reboot) / S (shutdown)
- Запуск через стандартные `LoadImage` + `StartImage`
- Сборка в ~30 КБ EFI-бинарника

## Структура ESP

```
ESP/
├── EFI/
│   └── BOOT/
│       └── BOOTX64.EFI   ← лаунчер (fallback bootloader)
├── launcher.cfg           ← конфиг
└── tools/
    ├── memtest86.efi
    ├── grubx64.efi
    └── shellx64.efi
```

## launcher.cfg

```toml
timeout = 10    # секунд до автовыбора (0 = ждать вечно)
default = 0     # индекс записи по умолчанию

[[entry]]
title = "Memtest86+"
path  = "\\tools\\memtest86.efi"
icon  = "M"

[[entry]]
title = "GRUB → Linux"
path  = "\\tools\\grubx64.efi"
icon  = "L"
```

## Сборка

Нужен **Rust nightly** (для `build-std`):

```bash
rustup toolchain install nightly
rustup target add x86_64-unknown-uefi --toolchain nightly
rustup component add rust-src --toolchain nightly
```

Сборка и копирование в `esp/`:

```bash
chmod +x build.sh
./build.sh
```

## Тест в QEMU

```bash
# Arch/CachyOS
sudo pacman -S qemu-system-x86 edk2-ovmf

chmod +x run-qemu.sh
./run-qemu.sh
```

## Запись на USB

```bash
# Монтируем ESP-раздел на флешке
sudo mount /dev/sdX1 /mnt/esp

# Копируем
sudo cp -r esp/* /mnt/esp/

sudo umount /mnt/esp
```

## CI

GitHub Actions собирает EFI-бинарник на каждый пуш в `main`  
и публикует архив `efi-launcher.zip` как артефакт релиза.

## Лицензия

MIT
