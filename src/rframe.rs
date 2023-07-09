pub mod frame {
    use image::DynamicImage;
    use std::error::Error;
    pub struct Bezel {
        pub frame: image::DynamicImage,
    }

    impl Bezel {
        pub fn build(frame_path: String) -> Result<Bezel, Box<dyn Error>> {
            let frame = image::open(frame_path)?;
            Ok(Bezel { frame })
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
