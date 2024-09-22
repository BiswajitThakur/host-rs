use std::fs;
use std::path::PathBuf;

use crossterm::style::Stylize;
use host_utils::{
    download_from_url, filter_host_from_vec_str, host_path, is_comment, is_valid_url, read_file,
    UserData,
};
use host_utils::{App, StoragePath};

use clap::{Arg, ArgAction, Command};

pub fn cmd(name: &'static str, about: &'static str, version: &'static str) -> Command {
    Command::new(name)
        .about(about)
        .version(version)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("insert")
                .short_flag('i')
                .long_flag("insert")
                .aliases(["add", "append", "push"])
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
                .long_flag("rm")
                .aliases(["pop", "delete"])
                .about("Remove allow, block, redirect host and host sources")
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
                )
                .arg(
                    Arg::new("self")
                        .long("self")
                        .help("Self unistall")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("import")
                .short_flag('I')
                .long_flag("import")
                .about("Import host or url from file.")
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
                .about("Expoer user data (you can import it later).")
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
                .about("print allow, block, redirect and host source.")
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
        .subcommand(
            Command::new("update")
                .short_flag('u')
                .long_flag("update")
                .about("Update sources or self. ")
                .arg(
                    Arg::new("sources")
                        .long("sources")
                        .help("Update host sources.")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("self")
                        .long("self")
                        .help("Self Update")
                        .conflicts_with("sources")
                        .action(ArgAction::SetTrue),
                ),
        )
}

pub enum UpdateOps {
    ThisApp,
    Sources,
}

#[allow(unused)]
pub enum CliApp {
    Add(CliArgs),
    Remove(CliArgs),
    Import(CliArgs),
    Export(CliArgs),
    Show,
    Update(UpdateOps),
}

pub enum CliArgs {
    Allow(Vec<String>),
    Block(Vec<String>),
    Redirect(Vec<String>),
    Sources(Vec<String>),
    RemoveSelf,
}

impl CliApp {
    pub fn init(name: &'static str, about: &'static str, version: &'static str) -> Self {
        cli_app(name, about, version)
    }
    pub fn run(&self, parent: &'static str) {
        run_app(self, parent);
    }
}

fn run_app(app: &CliApp, parent: &'static str) {
    let etc_host_content = || read_file(host_path()).unwrap();
    let st: StoragePath = [dirs::data_dir().unwrap(), parent.into()]
        .into_iter()
        .collect::<PathBuf>()
        .into();
    let allow = read_file(st.get_allow()).unwrap();
    let block = read_file(st.get_block()).unwrap();
    let redirect = read_file(st.get_redirect()).unwrap();
    let sources = read_file(st.get_sources()).unwrap();
    let binding = etc_host_content();
    let mut my_app = App::new(
        parent,
        UserData::init(&allow, &block, &redirect, &sources),
        binding.lines().collect(),
    )
    .unwrap();

    match app {
        CliApp::Add(v) => match v {
            CliArgs::Allow(u) => {
                let args: Vec<&str> = u.iter().map(|f| f.as_str()).collect();
                my_app.add_allow(&args);
                my_app.save();
            }
            CliArgs::Block(u) => {
                let args: Vec<&str> = u.iter().map(|f| f.as_str()).collect();
                my_app.add_block(&args);
                my_app.save();
            }
            CliArgs::Redirect(u) => {
                let args: Vec<&str> = u.iter().map(|f| f.as_str()).collect();
                if args.len() % 2 != 0 {
                    eprintln!("Error: Envalid argument length, length must be even");
                    std::process::exit(1);
                };
                let mut r = Vec::<(&str, &str)>::with_capacity(args.len() / 2);
                let mut iter = args.iter();
                while let (Some(u), Some(v)) = (iter.next(), iter.next()) {
                    r.push((u, v));
                }
                my_app.add_redirect(&r);
                my_app.save();
            }
            CliArgs::Sources(u) => {
                let args: Vec<&str> = u.iter().map(|f| f.as_str()).collect();
                let mut valid_urls = Vec::with_capacity(args.len());
                for url in args.into_iter() {
                    if is_comment(url) {
                        continue;
                    };
                    if !is_valid_url(url) {
                        eprintln!("Invalid url: {}", url.dark_red().to_owned());
                        continue;
                    };
                    valid_urls.push(url);
                }
                my_app.add_sources(&valid_urls);
                let downloaded: Vec<Result<String, _>> =
                    valid_urls.iter().map(download_from_url).collect();
                let mut downloaded_str: Vec<&str> = Vec::with_capacity(downloaded.len());
                let mut total_cap = 0;
                for i in downloaded.iter() {
                    match i {
                        Ok(ref t) => {
                            total_cap += t.len() / 15;
                            downloaded_str.push(t)
                        }
                        Err(ref e) => {
                            eprintln!("{}", e)
                        }
                    }
                }
                let hosts = filter_host_from_vec_str(downloaded_str, total_cap);
                my_app.add_etc_host(hosts);
                my_app.save();
            }
            _ => unreachable!(),
        },
        CliApp::Remove(v) => match v {
            CliArgs::Allow(u) => {
                let args: Vec<&str> = u.iter().map(|f| f.as_str()).collect();
                my_app.rm_allow(&args);
                my_app.save();
            }
            CliArgs::Block(u) => {
                let args: Vec<&str> = u.iter().map(|f| f.as_str()).collect();
                my_app.rm_block(&args);
                my_app.save();
            }
            CliArgs::Redirect(u) => {
                let args: Vec<&str> = u.iter().map(|f| f.as_str()).collect();
                my_app.rm_redirect(&args);
                my_app.save();
            }
            CliArgs::Sources(u) => {
                let args: Vec<&str> = u.iter().map(|f| f.as_str()).collect();
                my_app.rm_sources(&args);
                my_app.save();
            }
            CliArgs::RemoveSelf => {
                let bin_path = std::env::current_exe().unwrap();
                fs::remove_file(st.get_allow()).unwrap();
                println!("{:?}: removed", st.get_allow());
                fs::remove_file(st.get_block()).unwrap();
                println!("{:?}: removed", st.get_block());
                fs::remove_file(st.get_redirect()).unwrap();
                println!("{:?}: removed", st.get_redirect());
                fs::remove_file(st.get_sources()).unwrap();
                println!("{:?}: removed", st.get_sources());
                my_app.restore_etc_host_file();
                println!("{:?}: restored", host_path());
                fs::remove_file(&bin_path).unwrap();
                println!("{:?}: removed", bin_path);
                println!("Uninstall success...");
                std::process::exit(0);
            }
        },
        CliApp::Import(v) => match v {
            CliArgs::Allow(u) => {
                let mut strs = Vec::with_capacity(u.len());
                for p in u.iter() {
                    let f = fs::read_to_string(p).unwrap_or_else(|_| {
                        panic!(
                            "{}: faild to read file: {}",
                            "ERROR".red().bold(),
                            p.clone().italic()
                        )
                    });
                    strs.push(f);
                }
                let mut f = Vec::with_capacity(strs.len() * 500);
                for i in strs.iter() {
                    for j in i.lines() {
                        f.push(j);
                    }
                }
                my_app.add_allow(&f);
                my_app.save();
            }
            CliArgs::Block(u) => {
                let mut strs = Vec::with_capacity(u.len());
                for p in u.iter() {
                    let f = fs::read_to_string(p).unwrap_or_else(|_| {
                        panic!(
                            "{}: faild to read file: {}",
                            "ERROR".red().bold(),
                            p.clone().italic()
                        )
                    });
                    strs.push(f);
                }
                let mut f = Vec::with_capacity(strs.len() * 500);
                for i in strs.iter() {
                    for j in i.lines() {
                        f.push(j);
                    }
                }
                my_app.add_block(&f);
                my_app.save();
            }
            CliArgs::Redirect(u) => {
                let mut strs = Vec::with_capacity(u.len());
                for p in u.iter() {
                    let f = fs::read_to_string(p).unwrap_or_else(|_| {
                        panic!(
                            "{}: faild to read file: {}",
                            "ERROR".red().bold(),
                            p.clone().italic()
                        )
                    });
                    strs.push(f);
                }
                let mut f = Vec::with_capacity(strs.len() * 500);
                for i in strs.iter() {
                    for j in i.lines() {
                        let x: Vec<&str> = j.split_whitespace().collect();
                        if x.len() > 1 {
                            f.push((x[0], x[1]));
                        };
                    }
                }
                my_app.add_redirect(&f);
                my_app.save();
            }
            CliArgs::Sources(u) => {
                let mut strs = Vec::with_capacity(u.len());
                for p in u.iter() {
                    let f = fs::read_to_string(p).unwrap_or_else(|_| {
                        panic!(
                            "{}: faild to read file: {}",
                            "ERROR".red().bold(),
                            p.clone().italic()
                        )
                    });
                    strs.push(f);
                }
                let mut f = Vec::with_capacity(strs.len() * 500);
                for i in strs.iter() {
                    for j in i.lines() {
                        f.push(j);
                    }
                }
                my_app.add_block(&f);
                my_app.save();
            }
            _ => unreachable!(),
        },
        CliApp::Export(v) => match v {
            CliArgs::Allow(u) => {
                let path = PathBuf::from(u[0].clone());
                my_app.export_allow(path);
                std::process::exit(0);
            }
            CliArgs::Block(u) => {
                let path = PathBuf::from(u[0].clone());
                my_app.export_block(path);
                std::process::exit(0);
            }
            CliArgs::Redirect(u) => {
                let path = PathBuf::from(u[0].clone());
                my_app.export_redirect(path);
                std::process::exit(0);
            }
            CliArgs::Sources(u) => {
                let path = PathBuf::from(u[0].clone());
                my_app.export_sources(path);
                std::process::exit(0);
            }
            _ => unreachable!(),
        },
        CliApp::Show => {
            todo!()
        }
        CliApp::Update(u) => match u {
            UpdateOps::Sources => {
                let urls = my_app.get_sources().iter().map(|f| f.as_str());
                let downloaded: Vec<Result<String, _>> = urls.map(download_from_url).collect();
                let mut downloaded_str: Vec<&str> = Vec::with_capacity(downloaded.len());
                let mut total_cap = 0;
                for i in downloaded.iter() {
                    match i {
                        Ok(ref t) => {
                            total_cap += t.len() / 15;
                            downloaded_str.push(t);
                        }
                        Err(ref e) => {
                            eprintln!("{}", e);
                            std::process::exit(1);
                        }
                    }
                }
                let hosts = filter_host_from_vec_str(downloaded_str, total_cap);
                my_app.clear_host();
                my_app.add_etc_host(hosts);
                my_app.save();
            }
            UpdateOps::ThisApp => {
                todo!("This features is not yet implemented.")
            }
        },
    }
}

pub fn cli_app(name: &'static str, about: &'static str, version: &'static str) -> CliApp {
    let matches = cmd(name, about, version).get_matches();
    match matches.subcommand() {
        Some(("insert", add_matches)) => {
            if add_matches.contains_id("allow") {
                return CliApp::Add(CliArgs::Allow(
                    add_matches
                        .get_many::<String>("allow")
                        .unwrap()
                        .map(|f| f.to_owned())
                        .collect(),
                ));
            } else if add_matches.contains_id("block") {
                return CliApp::Add(CliArgs::Block(
                    add_matches
                        .get_many::<String>("block")
                        .unwrap()
                        .map(|f| f.to_owned())
                        .collect(),
                ));
            } else if add_matches.contains_id("redirect") {
                return CliApp::Add(CliArgs::Redirect(
                    add_matches
                        .get_many::<String>("redirect")
                        .unwrap()
                        .map(|f| f.to_owned())
                        .collect(),
                ));
            } else if add_matches.contains_id("sources") {
                return CliApp::Add(CliArgs::Sources(
                    add_matches
                        .get_many::<String>("sources")
                        .unwrap()
                        .map(|f| f.to_owned())
                        .collect(),
                ));
            };
            unreachable!()
        }
        Some(("remove", remove_matches)) => {
            if remove_matches.contains_id("allow") {
                return CliApp::Remove(CliArgs::Allow(
                    remove_matches
                        .get_many::<String>("allow")
                        .unwrap()
                        .map(|f| f.to_owned())
                        .collect(),
                ));
            } else if remove_matches.contains_id("block") {
                return CliApp::Remove(CliArgs::Block(
                    remove_matches
                        .get_many::<String>("block")
                        .unwrap()
                        .map(|f| f.to_owned())
                        .collect(),
                ));
            } else if remove_matches.contains_id("redirect") {
                return CliApp::Remove(CliArgs::Redirect(
                    remove_matches
                        .get_many::<String>("redirect")
                        .unwrap()
                        .map(|f| f.to_owned())
                        .collect(),
                ));
            } else if remove_matches.contains_id("sources") {
                return CliApp::Remove(CliArgs::Sources(
                    remove_matches
                        .get_many::<String>("sources")
                        .unwrap()
                        .map(|f| f.to_owned())
                        .collect(),
                ));
            } else if remove_matches.get_flag("self") {
                return CliApp::Remove(CliArgs::RemoveSelf);
            };
            unreachable!()
        }
        Some(("import", import_matches)) => {
            if import_matches.contains_id("allow") {
                return CliApp::Import(CliArgs::Allow(
                    import_matches
                        .get_many::<String>("allow")
                        .unwrap()
                        .map(|f| f.to_owned())
                        .collect(),
                ));
            } else if import_matches.contains_id("block") {
                return CliApp::Import(CliArgs::Block(
                    import_matches
                        .get_many::<String>("block")
                        .unwrap()
                        .map(|f| f.to_owned())
                        .collect(),
                ));
            } else if import_matches.contains_id("redirect") {
                return CliApp::Import(CliArgs::Redirect(
                    import_matches
                        .get_many::<String>("redirect")
                        .unwrap()
                        .map(|f| f.to_owned())
                        .collect(),
                ));
            } else if import_matches.contains_id("sources") {
                return CliApp::Import(CliArgs::Sources(
                    import_matches
                        .get_many::<String>("sources")
                        .unwrap()
                        .map(|f| f.to_owned())
                        .collect(),
                ));
            };
            unreachable!()
        }
        Some(("export", export_matches)) => {
            if export_matches.contains_id("allow") {
                return CliApp::Export(CliArgs::Allow(
                    export_matches
                        .get_many::<String>("allow")
                        .unwrap()
                        .map(|f| f.to_owned())
                        .collect(),
                ));
            } else if export_matches.contains_id("block") {
                return CliApp::Export(CliArgs::Block(
                    export_matches
                        .get_many::<String>("block")
                        .unwrap()
                        .map(|f| f.to_owned())
                        .collect(),
                ));
            } else if export_matches.contains_id("redirect") {
                return CliApp::Export(CliArgs::Redirect(
                    export_matches
                        .get_many::<String>("redirect")
                        .unwrap()
                        .map(|f| f.to_owned())
                        .collect(),
                ));
            } else if export_matches.contains_id("sources") {
                return CliApp::Export(CliArgs::Sources(
                    export_matches
                        .get_many::<String>("sources")
                        .unwrap()
                        .map(|f| f.to_owned())
                        .collect(),
                ));
            };
            unreachable!()
        }
        Some(("show", _show_matches)) => {
            todo!()
        }
        Some(("update", update_matches)) => {
            if update_matches.get_flag("self") {
                return CliApp::Update(UpdateOps::ThisApp);
            } else if update_matches.get_flag("sources") {
                return CliApp::Update(UpdateOps::Sources);
            }
            unreachable!()
        }
        _ => unreachable!(),
    }
}
