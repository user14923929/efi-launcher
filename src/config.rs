//! Парсер конфига launcher.cfg.
//!
//! Формат — упрощённый TOML без внешних зависимостей (нет std, нет serde).
//!
//! ```toml
//! timeout = 5          # секунд до автовыбора (0 = ждать вечно)
//! default = 0          # индекс записи по умолчанию
//!
//! [[entry]]
//! title = "Memtest86+"
//! path  = "\\tools\\memtest86.efi"
//!
//! [[entry]]
//! title = "GRUB → Linux"
//! path  = "\\tools\\grubx64.efi"
//! icon  = "*"          # необязательно, один символ
//! ```

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct Config {
    /// Таймаут в секундах (0 = без таймаута)
    pub timeout: u32,
    /// Индекс записи по умолчанию
    pub default: usize,
    pub entries: Vec<Entry>,
}

#[derive(Debug, Clone)]
pub struct Entry {
    /// Отображаемое имя в меню
    pub title: String,
    /// Путь к EFI-файлу (Windows-style: \\tools\\grub.efi)
    pub path: String,
    /// Необязательная иконка (один символ)
    pub icon: char,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            timeout: 0,
            default: 0,
            entries: Vec::new(),
        }
    }
}

/// Парсим сырые байты конфига, возвращаем список записей или сообщение об ошибке.
pub fn parse(raw: &[u8]) -> Result<Vec<Entry>, &'static str> {
    let text = core::str::from_utf8(raw).map_err(|_| "launcher.cfg: не UTF-8")?;
    let mut cfg = Config::default();
    let mut cur: Option<EntryBuilder> = None;

    for line in text.lines() {
        let line = line.trim();

        // Пропускаем комментарии и пустые строки
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Начало новой секции [[entry]]
        if line == "[[entry]]" {
            if let Some(b) = cur.take() {
                cfg.entries.push(b.build()?);
            }
            cur = Some(EntryBuilder::default());
            continue;
        }

        // Глобальные параметры
        if cur.is_none() {
            if let Some(val) = kv(line, "timeout") {
                cfg.timeout = val.parse::<u32>().unwrap_or(0);
            } else if let Some(val) = kv(line, "default") {
                cfg.default = val.parse::<usize>().unwrap_or(0);
            }
            continue;
        }

        // Параметры текущей секции [entry]
        if let Some(ref mut b) = cur {
            if let Some(val) = kv(line, "title") {
                b.title = Some(unquote(val));
            } else if let Some(val) = kv(line, "path") {
                b.path = Some(unquote(val));
            } else if let Some(val) = kv(line, "icon") {
                b.icon = unquote(val).chars().next().unwrap_or('>');
            }
        }
    }

    // Последняя незакрытая секция
    if let Some(b) = cur.take() {
        cfg.entries.push(b.build()?);
    }

    Ok(cfg.entries)
}

// --- helpers ---

fn kv<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let line = line.trim();
    if line.starts_with(key) {
        let rest = line[key.len()..].trim_start();
        if rest.starts_with('=') {
            return Some(rest[1..].trim());
        }
    }
    None
}

fn unquote(s: &str) -> String {
    let s = s.trim();
    if (s.starts_with('"') && s.ends_with('"'))
        || (s.starts_with('\'') && s.ends_with('\''))
    {
        String::from(&s[1..s.len() - 1])
    } else {
        String::from(s)
    }
}

#[derive(Default)]
struct EntryBuilder {
    title: Option<String>,
    path: Option<String>,
    icon: char,
}

impl EntryBuilder {
    fn build(self) -> Result<Entry, &'static str> {
        Ok(Entry {
            title: self.title.ok_or("entry без поля title")?,
            path: self.path.ok_or("entry без поля path")?,
            icon: if self.icon == '\0' { '>' } else { self.icon },
        })
    }
}
