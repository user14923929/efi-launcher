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
use uefi::runtime::ResetType;

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
        screen.error("launcher.cfg: no [entry] records found");
        screen.wait_key();
        return Status::ABORTED;
    }

    loop {
        let choice = menu::run(&mut screen, &entries);
        match choice {
            menu::Action::Launch(idx) => {
                screen.status("Launching...");
                match launch::run(&entries[idx].path) {
                    Ok(_) => {}
                    Err(e) => {
                        screen.error(e);
                        screen.wait_key();
                    }
                }
            }
            menu::Action::Reboot => {
                uefi::runtime::reset(ResetType::WARM, Status::SUCCESS, None);
            }
            menu::Action::Shutdown => {
                uefi::runtime::reset(ResetType::SHUTDOWN, Status::SUCCESS, None);
            }
        }
    }
}

fn load_config() -> Result<Vec<config::Entry>, &'static str> {
    let image = uefi::boot::image_handle();
    // get_image_file_system returns ScopedProtocol<SimpleFileSystem> directly
    let mut sfs = uefi::boot::get_image_file_system(image)
        .map_err(|_| "Failed to get file system")?;

    let mut root = sfs
        .open_volume()
        .map_err(|_| "Failed to open ESP root")?;

    let mut file = root
        .open(
            cstr16!("launcher.cfg"),
            FileMode::Read,
            FileAttribute::empty(),
        )
        .map_err(|_| "launcher.cfg not found on ESP")?
        .into_regular_file()
        .ok_or("launcher.cfg is not a regular file")?;

    let mut raw: Vec<u8> = Vec::new();
    let mut buf = [0u8; 4096];
    loop {
        let n = file.read(&mut buf).map_err(|_| "Error reading launcher.cfg")?;
        if n == 0 { break; }
        raw.extend_from_slice(&buf[..n]);
    }

    config::parse(&raw)
}
