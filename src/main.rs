use minigrep::Config;
use std::env;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem passing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file path: {}", config.file_path);
    if let Err(e) = minigrep::run(config) {
        eprintln!("Problem passing arguments: {e}");
        process::exit(1);
    };
}
