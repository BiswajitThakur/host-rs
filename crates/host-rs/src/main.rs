mod cli;

fn main() {
    if let Err(err) = cli::run() {
        eprintln!("{}", err);
    }
}
