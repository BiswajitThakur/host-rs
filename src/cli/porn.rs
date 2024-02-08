use std::collections::HashSet;

use host_rs::app::App;

pub async fn init(
    app: &mut App,
    matches: &clap::ArgMatches,
    action: &mut bool,
) -> Result<(), reqwest::Error> {
    if matches.get_flag("block-porn") {
        *action = true;
        app.data.hosts.porn.is_enable = true;
        return Ok(block(app).await?);
    };
    if matches.get_flag("unblock-porn") {
        *action = true;
        app.data.hosts.porn.is_enable = false;
        unblock(app);
    };
    Ok(())
}

#[allow(dead_code)]
async fn block(app: &mut App) -> Result<(), reqwest::Error> {
    let mut downloads = Vec::<HashSet<String>>::with_capacity(app.data.hosts.porn.urls.len());
    let mut capacity: usize = 0;
    for url in app.data.hosts.porn.urls.iter() {
        if !url.is_enable {
            continue;
        };
        let tmp: String = host_rs::host_rw::get::get(&url.url).await?;
        let f: HashSet<String> = host_rs::host_rw::filter::host(tmp);
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
    if &result == &app.host.porn {
        return Ok(());
    };
    app.host.porn.clear();
    app.host.porn.extend(result);
    Ok(())
}

#[allow(dead_code)]
fn unblock(app: &mut App) {
    app.data.hosts.porn.is_enable = false;
    app.host.porn.clear();
}
