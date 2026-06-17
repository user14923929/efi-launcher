//! TUI-меню: стрелки, Enter, R/S.

use uefi::proto::console::text::{Key, ScanCode};

use crate::config::Entry;
use crate::ui::{Screen, C_DIM, C_NORMAL};

pub enum Action {
    Launch(usize),
    Reboot,
    Shutdown,
}

const SYS_REBOOT:   &str = "Перезагрузить";
const SYS_SHUTDOWN: &str = "Выключить";

pub fn run(screen: &mut Screen, entries: &[Entry]) -> Action {
    let total = entries.len() + 2;
    let mut sel: usize = 0;
    let mut needs_redraw = true;

    loop {
        if needs_redraw {
            draw(screen, entries, sel);
            needs_redraw = false;
        }

        match screen.read_key() {
            Some(Key::Special(sc)) => match sc {
                ScanCode::UP   => { sel = if sel == 0 { total - 1 } else { sel - 1 }; needs_redraw = true; }
                ScanCode::DOWN => { sel = (sel + 1) % total; needs_redraw = true; }
                ScanCode::HOME => { sel = 0; needs_redraw = true; }
                ScanCode::END  => { sel = total - 1; needs_redraw = true; }
                _ => {}
            },
            Some(Key::Printable(ch)) => match char::from(ch) {
                '\r' | '\n' => return resolve(sel, entries),
                'r' | 'R'  => return Action::Reboot,
                's' | 'S'  => return Action::Shutdown,
                _ => {}
            },
            None => { screen.stall_ms(5); }
        }
    }
}

fn resolve(sel: usize, entries: &[Entry]) -> Action {
    let n = entries.len();
    if sel < n       { Action::Launch(sel) }
    else if sel == n { Action::Reboot }
    else             { Action::Shutdown }
}

fn draw(screen: &mut Screen, entries: &[Entry], sel: usize) {
    screen.clear();
    screen.banner();
    screen.println("");

    for (i, e) in entries.iter().enumerate() {
        screen.menu_row(sel == i, e.icon, &e.title, &e.path);
    }

    screen.color(C_DIM);
    screen.hline('-');
    screen.color(C_NORMAL);

    let n = entries.len();
    screen.menu_row(sel == n,     'R', SYS_REBOOT,   "");
    screen.menu_row(sel == n + 1, 'S', SYS_SHUTDOWN, "");
    screen.println("");
    screen.color(C_DIM);
    screen.println("  Up/Down  выбор     Enter  запустить     R  перезагрузить     S  выключить");
    screen.color(C_NORMAL);
}
