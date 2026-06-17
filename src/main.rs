#![no_main]
#![no_std]

extern crate alloc;

mod config;
mod launch;
mod menu;
mod ui;

use alloc::vec::Vec;
use uefi::prelude::*;
use uefi::proto::media::file::{File, FileAttribute, FileMode};
use uefi::proto::media::fs::SimpleFileSystem;

#[entry]
fn main() -> Status {
    uefi::helpers::init();

    let mut screen = ui::Screen::new();
    screen.clear();
    screen.banner();

    let entries: Vec<config::Entry> = match load_config() {
        Ok(e) => e,
        Err(msg) => {
            screen.error(msg);
            screen.wait_key();
            return Status::ABORTED;
        }
    };

    if entries.is_empty() {
        screen.error("launcher.cfg: нет ни одной записи [entry]");
        screen.wait_key();
        return Status::ABORTED;
    }

    loop {
        let choice = menu::run(&mut screen, &entries);
        match choice {
            menu::Action::Launch(idx) => {
                screen.status("Запуск...");
                match launch::run(&entries[idx].path) {
                    Ok(_) => {}
                    Err(e) => {
                        screen.error(e);
                        screen.wait_key();
                    }
                }
            }
            menu::Action::Reboot => {
                uefi::runtime::reset(
                    uefi::table::runtime::ResetType::WARM,
                    Status::SUCCESS,
                    None,
                );
            }
            menu::Action::Shutdown => {
                uefi::runtime::reset(
                    uefi::table::runtime::ResetType::SHUTDOWN,
                    Status::SUCCESS,
                    None,
                );
            }
        }
    }
}

fn load_config() -> Result<Vec<config::Entry>, &'static str> {
    let image = uefi::boot::image_handle();

    let fs_handle = uefi::boot::get_image_file_system(image)
        .map_err(|_| "Не удалось получить файловую систему")?;

    let mut sfs = uefi::boot::open_protocol_exclusive::<SimpleFileSystem>(fs_handle)
        .map_err(|_| "SimpleFileSystem недоступен")?;

    let mut root = sfs
        .open_volume()
        .map_err(|_| "Не удалось открыть корень ESP")?;

    let mut file = root
        .open(
            cstr16!("launcher.cfg"),
            FileMode::Read,
            FileAttribute::empty(),
        )
        .map_err(|_| "Файл launcher.cfg не найден на ESP")?
        .into_regular_file()
        .ok_or("launcher.cfg — не обычный файл")?;

    let mut raw: Vec<u8> = Vec::new();
    let mut buf = [0u8; 4096];
    loop {
        let n = file.read(&mut buf).map_err(|_| "Ошибка чтения launcher.cfg")?;
        if n == 0 { break; }
        raw.extend_from_slice(&buf[..n]);
    }

    config::parse(&raw)
}
