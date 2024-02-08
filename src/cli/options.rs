use clap::{Arg, ArgAction};

pub fn init() -> clap::ArgMatches {
    clap::command!()
        .arg(
            Arg::new("block-web")
                .short('b')
                .long("block-web")
                .help("Block websites.")
                .num_args(1..)
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("unblock-web")
                .short('u')
                .long("unblock-web")
                .help("Unblock websites.")
                .conflicts_with("block-web")
                .num_args(1..)
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("block-ads")
                .long("block-ads")
                .help("Block Ads.")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("unblock-ads")
                .long("unblock-ads")
                .help("Unblock all ads.")
                .conflicts_with("block-ads")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("block-porn")
                .long("block-porn")
                .help("Block all porn sites.")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("unblock-porn")
                .long("unblock-porn")
                .help("Unblock porn websites.")
                .conflicts_with("block-porn")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("block-fakenews")
                .long("block-fakenews")
                .help("Block fakenews websites.")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("unblock-fakenews")
                .long("unblock-fakenews")
                .help("Unblock fakenews websites.")
                .conflicts_with("block-fakenews")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("block-social")
                .long("block-social")
                .help("Block all social websites.")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("unblock-social")
                .long("unblock-social")
                .help("Unlock social websites.")
                .conflicts_with("block-social")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("block-gambling")
                .long("block-gambling")
                .help("Block gambling websites.")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("unblock-gambling")
                .long("unblock-gambling")
                .help("Unblock gambling websites.")
                .conflicts_with("block-gambling")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("update-self")
                .long("update-self")
                .help("Update if available.")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("update-sources")
                .long("update-sources")
                .help("Update host sources if available.")
                .action(ArgAction::SetTrue),
        )
        .get_matches()
}
