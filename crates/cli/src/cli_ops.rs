use clap::{Arg, ArgAction, Command};

pub fn cmd(name: &'static str, about: &'static str, version: &'static str) -> Command {
    Command::new(name)
        .about(about)
        .version(version)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .short_flag('A')
                .long_flag("add")
                .aliases(["insert", "append", "push"])
                .about("Add host or url to allow, block, redirect, sources list.")
                .arg(
                    Arg::new("allow")
                        .short('a')
                        .long("allow")
                        .conflicts_with_all(["block", "redirect", "sources"])
                        .help("Add host to allow list & removed from block list.")
                        .action(ArgAction::Set)
                        .num_args(1..),
                )
                .arg(
                    Arg::new("block")
                        .short('b')
                        .long("block")
                        .conflicts_with_all(["redirect", "sources"])
                        .help("Add to block list & remove from allow list.")
                        .action(ArgAction::Set)
                        .num_args(1..),
                )
                .arg(
                    Arg::new("redirect")
                        .short('r')
                        .long("redirect")
                        .conflicts_with("sources")
                        .help("Add to redirect list & remove from allow and block.")
                        .action(ArgAction::Set)
                        .num_args(1..),
                )
                .arg(
                    Arg::new("sources")
                        .short('s')
                        .long("sources")
                        .help("Add url to sources list")
                        .action(ArgAction::Set)
                        .num_args(1..),
                ),
        )
        .subcommand(
            Command::new("remove")
                .short_flag('R')
                .long_flag("remove")
                .aliases(["rm", "pop", "delete"])
                .about(".....")
                .arg(
                    Arg::new("allow")
                        .short('a')
                        .long("allow")
                        .conflicts_with_all(["block", "redirect", "sources"])
                        .help("Remove host from allow list.")
                        .action(ArgAction::Set)
                        .num_args(1..),
                )
                .arg(
                    Arg::new("block")
                        .short('b')
                        .long("block")
                        .conflicts_with_all(["redirect", "sources"])
                        .help("Remove host from block list.")
                        .action(ArgAction::Set)
                        .num_args(1..),
                )
                .arg(
                    Arg::new("redirect")
                        .short('r')
                        .long("redirect")
                        .conflicts_with("sources")
                        .help("Remove host from redirect list.")
                        .action(ArgAction::Set)
                        .num_args(1..),
                )
                .arg(
                    Arg::new("sources")
                        .short('s')
                        .long("sources")
                        .help("Remove url from sources list")
                        .action(ArgAction::Set)
                        .num_args(1..),
                ),
        )
        .subcommand(
            Command::new("import")
                .short_flag('I')
                .long_flag("import")
                .about(".....")
                .arg(
                    Arg::new("allow")
                        .short('a')
                        .long("allow")
                        .conflicts_with_all(["block", "redirect", "sources"])
                        .help("import allow list.")
                        .action(ArgAction::Set)
                        .num_args(1..),
                )
                .arg(
                    Arg::new("block")
                        .short('b')
                        .long("block")
                        .conflicts_with_all(["redirect", "sources"])
                        .help("import block list.")
                        .action(ArgAction::Set)
                        .num_args(1..),
                )
                .arg(
                    Arg::new("redirect")
                        .short('r')
                        .long("redirect")
                        .conflicts_with("sources")
                        .help("import redirect list.")
                        .action(ArgAction::Set)
                        .num_args(1..),
                )
                .arg(
                    Arg::new("sources")
                        .short('s')
                        .long("sources")
                        .help("import sources list")
                        .action(ArgAction::Set)
                        .num_args(1..),
                ),
        )
        .subcommand(
            Command::new("export")
                .short_flag('E')
                .long_flag("export")
                .about(".....")
                .arg(
                    Arg::new("allow")
                        .short('a')
                        .long("allow")
                        .conflicts_with_all(["block", "redirect", "sources"])
                        .help("export allow list.")
                        .action(ArgAction::Set)
                        .num_args(1),
                )
                .arg(
                    Arg::new("block")
                        .short('b')
                        .long("block")
                        .conflicts_with_all(["redirect", "sources"])
                        .help("export block list.")
                        .action(ArgAction::Set)
                        .num_args(1),
                )
                .arg(
                    Arg::new("redirect")
                        .short('r')
                        .long("redirect")
                        .conflicts_with("sources")
                        .help("export redirect list.")
                        .action(ArgAction::Set)
                        .num_args(1),
                )
                .arg(
                    Arg::new("sources")
                        .short('s')
                        .long("sources")
                        .help("expoort sources list")
                        .action(ArgAction::Set)
                        .num_args(1),
                ),
        )
        .subcommand(
            Command::new("show")
                .short_flag('s')
                .long_flag("show")
                .about(".....")
                .arg(
                    Arg::new("allow")
                        .short('a')
                        .long("allow")
                        .conflicts_with_all(["block", "redirect", "sources"])
                        .help("show allow list.")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("block")
                        .short('b')
                        .long("block")
                        .conflicts_with_all(["redirect", "sources"])
                        .help("show block list.")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("redirect")
                        .short('r')
                        .long("redirect")
                        .conflicts_with("sources")
                        .help("show redirect list.")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("sources")
                        .short('s')
                        .long("sources")
                        .help("show sources list")
                        .action(ArgAction::SetTrue),
                ),
        )
        .arg(
            Arg::new("update")
                .short('u')
                .long("update")
                .help("Self update")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("uninstall")
                .long("uninstall")
                .help("Self unistall")
                .action(ArgAction::SetTrue),
        )
}
