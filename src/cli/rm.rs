use std::{env, fs, io};

use host_rs::{app::App, app_data::usr::is_admin};
#[allow(unused)]
pub fn init(app: &mut App, matches: &clap::ArgMatches, action: &mut bool) -> std::io::Result<()> {
    if !is_admin() {
        panic!("Administrator privilages required");
    };
    if matches.get_flag("rm-self") {
        app.clear_all();
        rm_self(app)?;
    };
    Ok(())
}

fn rm_self(app: &mut App) -> io::Result<()> {
    let pkg_path = env::current_exe().unwrap();
    fs::remove_file(pkg_path)?;
    fs::remove_file(&app.data.allow_path)?;
    fs::remove_file(&app.data.block_path)?;
    fs::remove_file(&app.data.redirect_path)?;
    fs::remove_file(&app.data.db_path)?;
    fs::remove_file(&app.data.hosts.ads.path)?;
    fs::remove_file(&app.data.hosts.fakenews.path)?;
    fs::remove_file(&app.data.hosts.gambling.path)?;
    fs::remove_file(&app.data.hosts.porn.path)?;
    fs::remove_file(&app.data.hosts.social.path)?;
    Ok(())
}
