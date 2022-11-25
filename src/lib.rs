use std::env;
use std::error::Error;
mod rframe;
use rframe::frame;
mod utils;
pub struct Config {
    pub file_path: String,
    pub output_path: String,
    pub frames_dir: String,
    pub frame_name: String,
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

        let frames_dir;
        if let Some(..) = args.next() {
            frames_dir = match args.next() {
                Some(val) => val,
                None => return Err("Frames directory not provided"),
            };
        } else {
            frames_dir = match env::var("FRAMES_DIR") {
                Ok(val) => val,
                Err(..) => return Err("Frames directory not set in env"),
            }
        }

        let frame_name = match args.next() {
            Some(frames_name) => frames_name,
            None => String::from("iphone13.png"),
        };
        Ok(Config {
            file_path,
            output_path,
            frames_dir,
            frame_name,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let screenshot = image::open(config.file_path)?;
    let bezel = frame::Bezel::build(format!("{}/{}", config.frames_dir, config.frame_name))?;
    let img = frame::render(screenshot, bezel.frame)?;
    img.save(config.output_path)?;
    return Ok(());
}
