mod cli_ops;
mod commands;

use std::fs;
use std::path::PathBuf;

use crossterm::style::Stylize;
use host_utils::{download_from_url, host_path, read_file, Container};
use host_utils::{App, StoragePath};

use crate::cli_ops::cmd;

pub enum UpdateOps {
    ThisApp,
    Sources,
}

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
        Container::init(&allow, &block, &redirect, &sources),
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
                my_app.add_sources(&args);
                my_app.save();
            }
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
        },
        CliApp::Import(v) => match v {
            CliArgs::Allow(u) => {
                let mut strs = Vec::with_capacity(u.len());
                for p in u.iter() {
                    let f = fs::read_to_string(p).expect(
                        format!(
                            "{}: faild to read file: {}",
                            "ERROR".red().bold(),
                            p.clone().italic()
                        )
                        .as_str(),
                    );
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
                    let f = fs::read_to_string(p).expect(
                        format!(
                            "{}: faild to read file: {}",
                            "ERROR".red().bold(),
                            p.clone().italic()
                        )
                        .as_str(),
                    );
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
                    let f = fs::read_to_string(p).expect(
                        format!(
                            "{}: faild to read file: {}",
                            "ERROR".red().bold(),
                            p.clone().italic()
                        )
                        .as_str(),
                    );
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
                    let f = fs::read_to_string(p).expect(
                        format!(
                            "{}: faild to read file: {}",
                            "ERROR".red().bold(),
                            p.clone().italic()
                        )
                        .as_str(),
                    );
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
        },
        CliApp::Export(v) => match v {
            CliArgs::Allow(u) => {
                let p = PathBuf::from(u[0].clone());
                if let Some(parent) = p.parent() {
                    if !parent.exists() {
                        fs::create_dir_all(parent)
                            .expect(&format!("Faild to create dir: {}", parent.display()));
                    };
                };
                my_app.export_allow(p);
                std::process::exit(0);
            }
            CliArgs::Block(u) => {
                let p = PathBuf::from(u[0].clone());
                if let Some(parent) = p.parent() {
                    if !parent.exists() {
                        fs::create_dir_all(parent)
                            .expect(&format!("Faild to create dir: {}", parent.display()));
                    };
                };
                my_app.export_block(p);
                std::process::exit(0);
            }
            CliArgs::Redirect(u) => {
                let p = PathBuf::from(u[0].clone());
                if let Some(parent) = p.parent() {
                    if !parent.exists() {
                        fs::create_dir_all(parent)
                            .expect(&format!("Faild to create dir: {}", parent.display()));
                    };
                };
                my_app.export_redirect(p);
                std::process::exit(0);
            }
            CliArgs::Sources(u) => {
                let p = PathBuf::from(u[0].clone());
                if let Some(parent) = p.parent() {
                    if !parent.exists() {
                        fs::create_dir_all(parent)
                            .expect(&format!("Faild to create dir: {}", parent.display()));
                    };
                };
                my_app.export_sources(p);
                std::process::exit(0);
            }
        },
        CliApp::Show => {
            todo!()
        }
        CliApp::Update(u) => match u {
            UpdateOps::Sources => {
                let urls = my_app.get_sources().into_iter().map(|f| f.as_str());
                for i in urls {
                    println!("{}", download_from_url(i).unwrap());
                }
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
