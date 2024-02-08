use std::collections::HashSet;

use host_rs::app::App;

pub async fn init(
    app: &mut App,
    matches: &clap::ArgMatches,
    action: &mut bool,
) -> Result<(), reqwest::Error> {
    if matches.get_flag("block-fakenews") {
        *action = true;
        app.data.hosts.fakenews.is_enable = true;
        return Ok(block(app).await?);
    };
    if matches.get_flag("unblock-fakenews") {
        *action = true;
        app.data.hosts.fakenews.is_enable = false;
        unblock(app);
    };
    Ok(())
}

#[allow(dead_code)]
async fn block(app: &mut App) -> Result<(), reqwest::Error> {
    let mut downloads = Vec::<HashSet<String>>::with_capacity(app.data.hosts.fakenews.urls.len());
    let mut capacity: usize = 0;
    for url in app.data.hosts.fakenews.urls.iter() {
        if !url.is_enable {
            continue;
        };
        let tmp: String = host_rs::host_rw::get::get(&url.url).await?;
        let f = host_rs::host_rw::filter::host(tmp);
        capacity += f.len();
        downloads.push(f);
    }
    let mut result = HashSet::<String>::with_capacity(capacity);
    for i in downloads {
        result.extend(i);
    }
    result.remove("127.0.0.1");
    result.remove("localhost");
    result.remove("0.0.0.0");
    if &result == &app.host.fakenews {
        return Ok(());
    };
    app.host.fakenews.clear();
    app.host.fakenews.extend(result);
    Ok(())
}

#[allow(dead_code)]
fn unblock(app: &mut App) {
    app.data.hosts.fakenews.is_enable = false;
    app.host.fakenews.clear();
}
