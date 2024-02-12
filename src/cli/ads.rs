use host_rs::app::App;

pub async fn init(
    app: &mut App,
    matches: &clap::ArgMatches,
    action: &mut bool,
) -> Result<(), reqwest::Error> {
    if matches.get_flag("block-ads") {
        *action = true;
        app.data.hosts.ads.is_enable = true;
        return Ok(app.block_ads().await?);
    };
    if matches.get_flag("unblock-ads") {
        *action = true;
        app.data.hosts.ads.is_enable = false;
        app.unblock_ads();
    };
    Ok(())
}
