use rframe::Config;
use std::env;
use std::process;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
    if let Err(e) = rframe::run(config) {
        eprintln!("{}", e);
        process::exit(1);
    }
}
