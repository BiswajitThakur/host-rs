use host_rs::app::App;

pub async fn init(
    app: &mut App,
    matches: &clap::ArgMatches,
    action: &mut bool,
) -> Result<(), reqwest::Error> {
    if matches.get_flag("update-sources") {
        *action = true;
        sources(app).await?;
    };
    if matches.get_flag("update-self") {
        println!("This features is not implemented yet.");
    };
    Ok(())
}

async fn sources(app: &mut App) -> Result<(), reqwest::Error> {
    if app.data.hosts.ads.is_enable {
        app.block_ads().await?;
    };
    if app.data.hosts.fakenews.is_enable {
        app.block_fakenews().await?;
    };
    if app.data.hosts.gambling.is_enable {
        app.block_gambling().await?;
    };
    if app.data.hosts.porn.is_enable {
        app.block_porn().await?;
    };
    if app.data.hosts.social.is_enable {
        app.block_social().await?;
    }
    Ok(())
}
