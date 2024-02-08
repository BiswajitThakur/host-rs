use std::error::Error;

mod cli;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Ok(cli::Cli::init().await?)
}
