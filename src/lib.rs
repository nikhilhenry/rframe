use std::error::Error;
mod rframe;
use rframe::frame;
mod utils;
pub struct Config {
    pub file_path: String,
    pub output_path: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // skip the name of the program
        args.next();
        let file_path = match args.next() {
            Some(path) => path,
            None => return Err("Image path not provided"),
        };
        let output_path = match args.next() {
            Some(path) => utils::process_path(path),
            None => return Err("Output path not provided"),
        };
        Ok(Config {
            file_path,
            output_path,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // read user image
    let screenshot = match image::open(config.file_path) {
        Ok(file) => file,
        Err(e) => return Err(Box::new(e)),
    };
    // generate correct bezel
    let bezel = match frame::Bezel::build(String::from("iPhone13")) {
        Ok(file) => file,
        Err(e) => return Err(Into::into(e)),
    };

    let img = match frame::render(screenshot, bezel.frame) {
        Ok(img) => img,
        Err(e) => return Err(e),
    };
    if let Err(e) = img.save(config.output_path) {
        return Err(Box::new(e));
    }
    return Ok(());
}
