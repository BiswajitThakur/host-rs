use host_rs::app::App;

pub async fn init(
    app: &mut App,
    matches: &clap::ArgMatches,
    action: &mut bool,
) -> Result<(), reqwest::Error> {
    if matches.get_flag("block-gambling") {
        *action = true;
        app.data.hosts.gambling.is_enable = true;
        return Ok(app.block_gambling().await?);
    };
    if matches.get_flag("unblock-gambling") {
        *action = true;
        app.data.hosts.gambling.is_enable = false;
        app.unblock_gambling();
    };
    Ok(())
}

