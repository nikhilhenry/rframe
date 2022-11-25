pub mod frame {
    use image::DynamicImage;
    use rust_embed::RustEmbed;
    use std::error::Error;
    #[derive(RustEmbed)]
    #[folder = "assets"]
    struct Asset;

    pub struct Bezel {
        pub name: String,
        pub frame: image::DynamicImage,
    }

    impl Bezel {
        pub fn build(name: String) -> Result<Bezel, &'static str> {
            let frame = match name.as_str() {
                "iPhone13" => Asset::get("iphone13.png").unwrap(),
                _ => return Err("Device bezel not supported"),
            };
            let frame = image::load_from_memory(&frame.data).unwrap();
            Ok(Bezel { name, frame })
        }
    }

    pub fn render(
        screenshot: DynamicImage,
        bezel: DynamicImage,
    ) -> Result<image::RgbaImage, Box<dyn Error>> {
        // make a new image with the dimensions of the bezel
        let mut img: image::RgbaImage = image::ImageBuffer::new(bezel.width(), bezel.height());
        let x_cords = bezel.width() / 2 - screenshot.width() / 2;
        let y_cords = bezel.height() / 2 - screenshot.height() / 2;
        image::imageops::overlay(&mut img, &screenshot, x_cords.into(), y_cords.into());
        image::imageops::overlay(&mut img, &bezel, 0, 0);
        Ok(img)
    }
}
