mod cli_ops;
mod commands;

use std::path::PathBuf;

use host_utils::{host_path, read_file, HashList, H, R};
use storage::{Container, StoragePath};

use crate::cli_ops::cmd;

pub enum CliApp {
    Add(CliArgs),
    Remove(CliArgs),
    Import,
    Export,
    Show,
    UpdateSources,
    UpdateSelf,
}

pub enum CliArgs {
    Allow(Vec<String>),
    Block(Vec<String>),
    Redirect(Vec<String>),
    Sources(Vec<String>),
}

impl CliApp {
    pub fn init(name: &'static str, about: &'static str, version: &'static str) -> Self {
        let app = cli_app(name, about, version);
        app
    }
    pub fn run(&self, parent: &'static str) {
        // println!("host: {:?}", host_path());
        run_app(self, parent);
    }
}

fn run_app(app: &CliApp, parent: &'static str) {
    let parent: StoragePath = [dirs::data_dir().unwrap(), parent.into()]
        .into_iter()
        .collect::<PathBuf>()
        .into();
    let etc_host_content = || read_file(host_path()).unwrap();
    match app {
        CliApp::Add(v) => match v {
            CliArgs::Allow(u) => {
                let args: Vec<&str> = u.into_iter().map(|f| f.as_str()).collect();
                let _ = commands::add::allow(
                    args,
                    etc_host_content()
                        .lines()
                        .into_iter()
                        .map(|f| f.as_ref())
                        .collect(),
                    &parent,
                );
            }
            CliArgs::Block(u) => {
                let args: Vec<&str> = u.into_iter().map(|f| f.as_str()).collect();
                let _ = commands::add::block(
                    args,
                    etc_host_content()
                        .lines()
                        .into_iter()
                        .map(|f| f.as_ref())
                        .collect(),
                    &parent,
                );
            }
            CliArgs::Redirect(u) => {
                println!("Add to redirect: {:?}", u);
            }
            CliArgs::Sources(u) => {
                println!("Add to source: {:?}", u);
            }
        },
        CliApp::Remove(v) => match v {
            CliArgs::Allow(u) => {
                println!("Remove to Allow: {:?}", u);
            }
            CliArgs::Block(u) => {
                println!("Remove to Block: {:?}", u);
            }
            CliArgs::Redirect(u) => {
                println!("Remove to redirect: {:?}", u);
            }
            CliArgs::Sources(u) => {
                println!("Remove to source: {:?}", u);
            }
        },
        CliApp::Import => {
            todo!()
        }
        CliApp::Export => {
            todo!()
        }
        CliApp::Show => {
            todo!()
        }
        CliApp::UpdateSources => {
            todo!()
        }
        CliApp::UpdateSelf => {
            todo!()
        }
    }
}

pub fn cli_app(name: &'static str, about: &'static str, version: &'static str) -> CliApp {
    let matches = cmd(name, about, version).get_matches();
    match matches.subcommand() {
        Some(("add", add_matches)) => {
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
            };
            unreachable!()
        }
        Some(("import", _import_matches)) => {
            todo!()
        }
        Some(("export", _export_matches)) => {
            todo!()
        }
        Some(("show", _show_matches)) => {
            todo!()
        }
        _ => unreachable!(),
    }
}
