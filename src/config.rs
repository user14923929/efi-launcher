extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct Entry {
    pub title: String,
    pub path:  String,
    pub icon:  char,
}

pub fn parse(raw: &[u8]) -> Result<Vec<Entry>, &'static str> {
    let text = core::str::from_utf8(raw).map_err(|_| "launcher.cfg: не UTF-8")?;
    let mut entries = Vec::new();
    let mut cur: Option<EntryBuilder> = None;

    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') { continue; }
        if line == "[[entry]]" {
            if let Some(b) = cur.take() { entries.push(b.build()?); }
            cur = Some(EntryBuilder::default());
            continue;
        }
        if let Some(ref mut b) = cur {
            if let Some(v) = kv(line, "title") { b.title = Some(unquote(v)); }
            else if let Some(v) = kv(line, "path") { b.path = Some(unquote(v)); }
            else if let Some(v) = kv(line, "icon") { b.icon = unquote(v).chars().next().unwrap_or('>'); }
        }
    }
    if let Some(b) = cur.take() { entries.push(b.build()?); }
    Ok(entries)
}

fn kv<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    if line.starts_with(key) {
        let rest = line[key.len()..].trim_start();
        if rest.starts_with('=') { return Some(rest[1..].trim()); }
    }
    None
}

fn unquote(s: &str) -> String {
    let s = s.trim();
    if s.len() >= 2 && ((s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\''))) {
        String::from(&s[1..s.len()-1])
    } else { String::from(s) }
}

#[derive(Default)]
struct EntryBuilder { title: Option<String>, path: Option<String>, icon: char }

impl EntryBuilder {
    fn build(self) -> Result<Entry, &'static str> {
        Ok(Entry {
            title: self.title.ok_or("entry без поля title")?,
            path:  self.path.ok_or("entry без поля path")?,
            icon:  if self.icon == '\0' { '>' } else { self.icon },
        })
    }
}
