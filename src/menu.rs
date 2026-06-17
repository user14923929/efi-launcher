use uefi::proto::console::text::{Key, ScanCode};
use crate::config::Entry;
use crate::ui::{Screen, C_DIM, C_NORMAL};

pub enum Action { Launch(usize), Reboot, Shutdown }

pub fn run(screen: &mut Screen, entries: &[Entry]) -> Action {
    let total = entries.len() + 2;
    let mut sel = 0usize;
    let mut redraw = true;
    loop {
        if redraw { draw(screen, entries, sel); redraw = false; }
        match screen.read_key() {
            Some(Key::Special(sc)) => match sc {
                ScanCode::UP   => { sel = if sel == 0 { total-1 } else { sel-1 }; redraw = true; }
                ScanCode::DOWN => { sel = (sel + 1) % total; redraw = true; }
                ScanCode::HOME => { sel = 0; redraw = true; }
                ScanCode::END  => { sel = total-1; redraw = true; }
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
    if sel < n { Action::Launch(sel) } else if sel == n { Action::Reboot } else { Action::Shutdown }
}

fn draw(screen: &mut Screen, entries: &[Entry], sel: usize) {
    screen.clear(); screen.banner(); screen.println("");
    for (i, e) in entries.iter().enumerate() {
        screen.menu_row(sel == i, e.icon, &e.title, &e.path);
    }
    screen.color(C_DIM); screen.hline('-'); screen.color(C_NORMAL);
    let n = entries.len();
    screen.menu_row(sel == n,     'R', "Reboot", "");
    screen.menu_row(sel == n + 1, 'S', "Shutdown",     "");
    screen.println("");
    screen.color(C_DIM);
    screen.println("  Up/Down  select     Enter  launch     R  reboot     S  shutdown");
    screen.color(C_NORMAL);
}
