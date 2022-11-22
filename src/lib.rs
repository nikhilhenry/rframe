use rust_embed::RustEmbed;
#[derive(RustEmbed)]
#[folder = "assets"]
struct Asset;
pub struct Bezel {
    pub name: String,
    pub frame: image::DynamicImage,
}

impl Bezel {
    pub fn new(name: String) -> Result<Bezel, &'static str> {
        let frame = match name.as_str() {
            "iphone13" => Asset::get("iphone13.png").unwrap(),
            _ => return Err("Phone bezel not supported"),
        };
        let frame = image::load_from_memory(&frame.data).unwrap();
        Ok(Bezel { name, frame })
    }
}
