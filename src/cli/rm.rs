use host_rs::{app::App, app_data::usr::is_admin};

#[allow(unused)]
pub fn init(app: &mut App, matches: &clap::ArgMatches, action: &mut bool) -> std::io::Result<()> {
    if !is_admin() {
        panic!("Administrator privilages required");
    };

    Ok(())
}
