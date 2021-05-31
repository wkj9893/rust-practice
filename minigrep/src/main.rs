use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let config = match Config::new(env::args()) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        }
    };

    match minigrep::run(config) {
        Ok(_) => (),
        Err(err) => {
            eprintln!("Application error: {}", err);
            process::exit(1);
        }
    }
}
