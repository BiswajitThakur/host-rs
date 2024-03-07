use host_rs::app::App;
pub fn init(app: &mut App, matches: &clap::ArgMatches, action: &mut bool) {
    let block_web_args = matches
        .get_many::<String>("block-web")
        .unwrap_or_default()
        .map(|v| v.clone())
        .collect::<Vec<_>>();
    if block_web_args.len() > 0 {
        *action = true;
        app.add_to_block_list(block_web_args);
    };
    let unblock_web_args = matches
        .get_many::<String>("unblock-web")
        .unwrap_or_default()
        .map(|v| v.clone())
        .collect::<Vec<_>>();
    if unblock_web_args.len() > 0 {
        *action = true;
        app.add_to_allow_list(unblock_web_args);
    };
}
