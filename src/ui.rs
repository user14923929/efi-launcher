use uefi::proto::console::text::{Color, Key};

pub const WIDTH: usize = 80;
pub const C_TITLE:  (Color, Color) = (Color::Black,     Color::LightCyan);
pub const C_NORMAL: (Color, Color) = (Color::LightGray, Color::Black);
pub const C_SELECT: (Color, Color) = (Color::Black,     Color::LightGray);
pub const C_ERROR:  (Color, Color) = (Color::LightRed,  Color::Black);
pub const C_DIM:    (Color, Color) = (Color::DarkGray,  Color::Black);
pub const C_STATUS: (Color, Color) = (Color::Yellow,    Color::Black);

pub struct Screen;

impl Screen {
    pub fn new() -> Self { Screen }

    pub fn clear(&self) {
        uefi::system::with_stdout(|o| { o.clear().ok(); });
    }
    pub fn color(&self, (fg, bg): (Color, Color)) {
        uefi::system::with_stdout(|o| { o.set_color(fg, bg).ok(); });
    }
    pub fn print(&self, s: &str) {
        use uefi::CString16;
        if let Ok(cs) = CString16::try_from(s) {
            uefi::system::with_stdout(|o| { o.output_string(&cs).ok(); });
        }
    }
    pub fn println(&self, s: &str) { self.print(s); self.print("\r\n"); }

    pub fn hline(&self, ch: char) {
        let mut tmp = [0u8; 4];
        let s = ch.encode_utf8(&mut tmp);
        for _ in 0..WIDTH { self.print(s); }
        self.print("\r\n");
    }
    pub fn banner(&self) {
        self.color(C_TITLE);
        self.hline(' ');
        self.centered(" EFI Launcher  v0.1 ");
        self.hline(' ');
        self.color(C_NORMAL);
    }
    pub fn centered(&self, s: &str) {
        let pad = WIDTH.saturating_sub(s.len()) / 2;
        for _ in 0..pad { self.print(" "); }
        self.print(s); self.print("\r\n");
    }
    pub fn menu_row(&self, selected: bool, icon: char, title: &str, hint: &str) {
        if selected { self.color(C_SELECT); } else { self.color(C_NORMAL); }
        let prefix = if selected { " > " } else { "   " };
        self.print(prefix);
        let mut tmp = [0u8; 4];
        self.print(icon.encode_utf8(&mut tmp));
        self.print("  "); self.print(title);
        let used = prefix.len() + 1 + 2 + title.len() + 2 + hint.len();
        for _ in 0..WIDTH.saturating_sub(used) { self.print(" "); }
        self.print(hint); self.print("\r\n");
        self.color(C_NORMAL);
    }
    pub fn status(&self, msg: &str) {
        self.color(C_STATUS); self.println(msg); self.color(C_NORMAL);
    }
    pub fn error(&self, msg: &str) {
        self.color(C_ERROR); self.print("[ERR] "); self.println(msg); self.color(C_NORMAL);
    }
    pub fn wait_key(&self) {
        self.color(C_DIM); self.println("Press any key..."); self.color(C_NORMAL);
        loop {
            let k = uefi::system::with_stdin(|i| i.read_key().ok().flatten());
            if k.is_some() { break; }
            uefi::boot::stall(5_000);
        }
    }
    pub fn read_key(&self) -> Option<Key> {
        uefi::system::with_stdin(|i| i.read_key().ok().flatten())
    }
    pub fn stall_ms(&self, ms: usize) { uefi::boot::stall(ms * 1_000); }
}
