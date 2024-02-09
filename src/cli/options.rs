use clap::{Arg, ArgAction};

pub fn init() -> clap::ArgMatches {
    clap::command!()
        .arg(
            Arg::new("block-web")
                .short('b')
                .long("block-web")
                .aliases(["bw", "BW"])
                .help("Block websites.")
                .num_args(1..)
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("unblock-web")
                .short('u')
                .long("unblock-web")
                .aliases(["uw", "UW"])
                .help("Unblock websites.")
                .conflicts_with("block-web")
                .num_args(1..)
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("add-redirect")
                .long("add-redirect")
                .aliases(["ar", "AR"])
                .help("Redirect website")
                .num_args(2)
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("rm-redirect")
                .long("rm-redirect")
                .aliases(["rr", "RR"])
                .help("remove from redirect")
                .conflicts_with("add-redirect")
                .num_args(1..)
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("block-ads")
                .long("block-ads")
                .aliases(["ba", "BA"])
                .help("Block Ads.")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("unblock-ads")
                .long("unblock-ads")
                .aliases(["ua", "UA", "uba"])
                .help("Unblock all ads.")
                .conflicts_with("block-ads")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("block-porn")
                .long("block-porn")
                .aliases(["bp", "BP"])
                .help("Block all porn sites.")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("unblock-porn")
                .long("unblock-porn")
                .aliases(["up", "UP", "ubp"])
                .help("Unblock porn websites.")
                .conflicts_with("block-porn")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("block-fakenews")
                .long("block-fakenews")
                .aliases(["bf", "BF"])
                .help("Block fakenews websites.")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("unblock-fakenews")
                .long("unblock-fakenews")
                .aliases(["uf", "UF", "ubf"])
                .help("Unblock fakenews websites.")
                .conflicts_with("block-fakenews")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("block-social")
                .long("block-social")
                .aliases(["bs", "BS"])
                .help("Block all social websites.")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("unblock-social")
                .long("unblock-social")
                .aliases(["us", "US", "ubs"])
                .help("Unlock social websites.")
                .conflicts_with("block-social")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("block-gambling")
                .long("block-gambling")
                .aliases(["bg", "BG"])
                .help("Block gambling websites.")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("unblock-gambling")
                .long("unblock-gambling")
                .aliases(["ug", "UG", "ubg"])
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
            Arg::new("rm-self")
                .long("remove-self")
                .alias("rm-self")
                .help("Uninstall")
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
