use std::error::Error;

mod cli;
fn main() -> Result<(), Box<dyn Error>> {
    Ok(cli::Cli::init()?)
}
