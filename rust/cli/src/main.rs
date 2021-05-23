use std::process;

mod config;
// mod profiler;
mod runner;

fn main() {
    let config = config::Config::new().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = runner::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
