use host_rs::app::App;

pub async fn init(
    app: &mut App,
    matches: &clap::ArgMatches,
    action: &mut bool,
) -> Result<(), reqwest::Error> {
    if matches.get_flag("block-social") {
        *action = true;
        app.data.hosts.social.is_enable = true;
        return Ok(app.block_social().await?);
    };
    if matches.get_flag("unblock-social") {
        *action = true;
        app.data.hosts.social.is_enable = false;
        app.unblock_social();
    };
    Ok(())
}
