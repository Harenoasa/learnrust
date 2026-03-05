use std::env;
use std::process;
use minigrep::Config;

fn main() {

    // let args :Vec<String> = env::args().collect();
    let args = env::args();
    let config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("\x1b[31mProblem parsing arguments: {}\x1b[0m", err);
        process::exit(1);
    });
    if let Err(e) =  minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}

