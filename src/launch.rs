//! Запуск EFI-файла через LoadImage + StartImage.
//! В uefi 0.33 новый API: uefi::boot::load_image / start_image.

extern crate alloc;

use core::mem::MaybeUninit;
use uefi::boot::LoadImageSource;
use uefi::proto::device_path::build::media::FilePath;
use uefi::proto::device_path::build::DevicePathBuilder;

pub fn run(path: &str) -> Result<(), &'static str> {
    let image = uefi::boot::image_handle();

    let path_cs = uefi::CString16::try_from(path)
        .map_err(|_| "Некорректный путь")?;

    let mut buf = [MaybeUninit::<u8>::uninit(); 512];
    let dp = DevicePathBuilder::with_buf(&mut buf)
        .push(&FilePath { path_name: &path_cs })
        .map_err(|_| "Ошибка построения DevicePath")?
        .finalize()
        .map_err(|_| "Ошибка финализации DevicePath")?;

    let new_image = uefi::boot::load_image(
        image,
        LoadImageSource::FromFilePath {
            file_path: dp,
            from_boot_manager: false,
        },
    )
    .map_err(|_| "LoadImage: не удалось загрузить файл")?;

    uefi::boot::start_image(new_image)
        .map_err(|_| "StartImage: образ завершился с ошибкой")?;

    Ok(())
}
