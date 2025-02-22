use std::{fs::OpenOptions, io::Write};

use anyhow::Result;
use rascii_art::{charsets, RenderOptions};
use xcap::{
    image::{DynamicImage, ImageBuffer, Rgba},
    Window,
};

fn main() -> Result<()> {
    let windows = Window::all()?;

    let mut window = windows
        .into_iter()
        .find(|w| w.title().contains("Chocolate Doom"))
        .expect("Doom not running");

    let mut buf = String::new();

    let mut back = false;

    loop {
        window.refresh()?;

        let image = if back {
            ImageBuffer::<Rgba<u8>, Vec<u8>>::from_pixel(200, 80, Rgba([255, 255, 255, 1]))
        } else {
            window.capture_image()?
        };

        buf.clear();

        rascii_art::render_image_to(
            &image.into(),
            &mut buf,
            &RenderOptions::new()
                .width(200)
                .height(80)
                .colored(true)
                .charset(charsets::DEFAULT),
        )?;

        let mut config = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open("/home/matilde/.config/pbfetch/config.txt")?;

        config.write_all(buf.as_bytes())?;

        config.flush()?;

        // back = !back
    }
}
