//! Launch an EFI file via LoadImage + StartImage.

extern crate alloc;

use core::mem::MaybeUninit;
use uefi::boot::LoadImageSource;
use uefi::proto::device_path::build::media::FilePath;
use uefi::proto::device_path::build::DevicePathBuilder;

pub fn run(path: &str) -> Result<(), &'static str> {
    let image = uefi::boot::image_handle();

    let path_cs = uefi::CString16::try_from(path)
        .map_err(|_| "Invalid path")?;

    let mut buf = [MaybeUninit::<u8>::uninit(); 512];
    let dp = DevicePathBuilder::with_buf(&mut buf)
        .push(&FilePath { path_name: &path_cs })
        .map_err(|_| "DevicePath construction error")?
        .finalize()
        .map_err(|_| "DevicePath finalization error")?;

    // The compiler hinted: the variant is called FromDevicePath, not FromFilePath
    let new_image = uefi::boot::load_image(
        image,
        LoadImageSource::FromDevicePath {
            device_path: dp,
            from_boot_manager: false,
        },
    )
    .map_err(|_| "LoadImage: failed to load file")?;

    uefi::boot::start_image(new_image)
        .map_err(|_| "StartImage: image exited with error")?;

    Ok(())
}
