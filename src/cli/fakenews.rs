use host_rs::app::App;

pub async fn init(
    app: &mut App,
    matches: &clap::ArgMatches,
    action: &mut bool,
) -> Result<(), reqwest::Error> {
    if matches.get_flag("block-fakenews") {
        *action = true;
        app.data.hosts.fakenews.is_enable = true;
        return Ok(app.block_fakenews().await?);
    };
    if matches.get_flag("unblock-fakenews") {
        *action = true;
        app.data.hosts.fakenews.is_enable = false;
        app.unblock_fakenews();
    };
    Ok(())
}
