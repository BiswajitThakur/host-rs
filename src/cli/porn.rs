use host_rs::app::App;

pub async fn init(
    app: &mut App,
    matches: &clap::ArgMatches,
    action: &mut bool,
) -> Result<(), reqwest::Error> {
    if matches.get_flag("block-porn") {
        *action = true;
        app.data.hosts.porn.is_enable = true;
        return Ok(app.block_porn().await?);
    };
    if matches.get_flag("unblock-porn") {
        *action = true;
        app.data.hosts.porn.is_enable = false;
        app.unblock_porn();
    };
    Ok(())
}
