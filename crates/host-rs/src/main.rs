use std::env::args;

mod cli;

fn main() {
    if args().collect::<Vec<_>>().len() < 2 {
        println!("Usage: {} [OPTIONS] [COMMAND]", env!("CARGO_PKG_NAME"));
        std::process::exit(1);
    }
    if let Err(err) = cli::run() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
