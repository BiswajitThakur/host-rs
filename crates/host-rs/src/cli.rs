use std::fs;
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
use crossterm::style::Stylize;

use host_utils::{
    download_from_url, filter_host_from_vec_str, host_path, is_comment, is_valid_url, read_file,
    App, StoragePath, UserData,
};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add host or url to allow, block, redirect, sources list.
    #[command(alias = "add", arg_required_else_help = true)]
    Insert {
        #[command(flatten)]
        command: InsertOps,
    },
    /// Remove allow, block, redirect host and host sources
    #[command(alias = "rm", arg_required_else_help = true)]
    Remove {
        #[command(flatten)]
        command: RemoveOps,
    },
    /// Import host or url from file.
    #[command(arg_required_else_help = true)]
    Import {
        #[command(flatten)]
        command: ImportOps,
    },
    /// Expoer user data (you can import it later).
    #[command(arg_required_else_help = true)]
    Export {
        #[command(flatten)]
        command: ExportOps,
    },
    /// Update sources or self.
    Update {
        #[command(flatten)]
        command: UpdateOps1,
    },
}

#[derive(Args)]
#[group(required = true, multiple = false)]
struct InsertOps {
    /// Add host to allow list & removed from block list.
    #[arg(long, required = true, num_args = 1..)]
    allow: Vec<String>,

    /// Add to block list & remove from allow list.
    #[arg(long, required = true, num_args = 1..)]
    block: Vec<String>,

    /// Add to redirect list & remove from allow and block.
    #[arg(long, required = true, num_args = 1..)]
    redirect: Vec<String>,

    /// Add url to sources list
    #[arg(long, required = true, num_args = 1..)]
    sources: Vec<String>,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
struct RemoveOps {
    /// Remove host from allow list.
    #[arg(long, required = true, num_args = 1..)]
    allow: Vec<String>,

    /// Remove host from block list.
    #[arg(long, required = true, num_args = 1..)]
    block: Vec<String>,

    /// Remove host from redirect list.
    #[arg(long, required = true, num_args = 1..)]
    redirect: Vec<String>,

    /// Remove url from sources list
    #[arg(long, required = true, num_args = 1..)]
    sources: Vec<String>,

    /// Uninstall
    #[arg(long = "self", required = true)]
    _self: bool,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
struct ImportOps {
    /// import allow list from file
    #[arg(long, required = true, num_args = 1)]
    allow: Vec<PathBuf>,

    /// import block list from file
    #[arg(long, required = true, num_args = 1)]
    block: Vec<PathBuf>,

    /// import redirect list from file
    #[arg(long, required = true, num_args = 1)]
    redirect: Vec<PathBuf>,

    /// import source list from file
    #[arg(long, required = true, num_args = 1)]
    sources: Vec<PathBuf>,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
struct ExportOps {
    /// export allow list.
    #[arg(long, required = true, num_args = 1)]
    allow: Vec<PathBuf>,

    /// export block list.
    #[arg(long, required = true, num_args = 1)]
    block: Vec<PathBuf>,

    /// export redirect list.
    #[arg(long, required = true, num_args = 1)]
    redirect: Vec<PathBuf>,

    /// export sources list.
    #[arg(long, required = true, num_args = 1)]
    sources: Vec<PathBuf>,

    /// export all
    #[arg(long, required = true, num_args = 1)]
    all: Vec<PathBuf>,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
struct UpdateOps1 {
    /// Update self
    #[arg(long = "self", required = true)]
    _self: bool,

    /// Update host sources
    #[arg(long, required = true)]
    sources: bool,
}

pub fn run() {
    let etc_host_content = || read_file(host_path()).unwrap();
    let st: StoragePath = [dirs::data_dir().unwrap(), env!("CARGO_BIN_NAME").into()]
        .into_iter()
        .collect::<PathBuf>()
        .into();
    let allow = read_file(st.get_allow()).unwrap();
    let block = read_file(st.get_block()).unwrap();
    let redirect = read_file(st.get_redirect()).unwrap();
    let sources = read_file(st.get_sources()).unwrap();
    let binding = etc_host_content();
    let mut app = App::new(
        env!("CARGO_BIN_NAME"),
        UserData::init(&allow, &block, &redirect, &sources),
        binding.lines().collect(),
    )
    .unwrap();

    let cli = Cli::parse();
    match cli.command {
        Commands::Insert { command } => {
            if !command.allow.is_empty() {
                let args: Vec<&str> = command.allow.iter().map(|f| f.as_str()).collect();
                app.add_allow(&args);
                app.save();
            } else if !command.block.is_empty() {
                let args: Vec<&str> = command.block.iter().map(|f| f.as_str()).collect();
                app.add_block(&args);
                app.save();
            } else if !command.redirect.is_empty() {
                let args: Vec<&str> = command.redirect.iter().map(|f| f.as_str()).collect();
                if args.len() % 2 != 0 {
                    eprintln!("Error: Envalid argument length, length must be even");
                    std::process::exit(1);
                };
                let mut r = Vec::<(&str, &str)>::with_capacity(args.len() / 2);
                let mut iter = args.iter();
                while let (Some(u), Some(v)) = (iter.next(), iter.next()) {
                    r.push((u, v));
                }
                app.add_redirect(&r);
                app.save();
            } else if !command.sources.is_empty() {
                let args: Vec<&str> = command.sources.iter().map(|f| f.as_str()).collect();
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
                app.add_sources(&valid_urls);
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
                app.add_etc_host(hosts);
                app.save();
            } else {
                unreachable!()
            }
        }
        Commands::Remove { command } => {
            if !command.allow.is_empty() {
                let args: Vec<&str> = command.allow.iter().map(|f| f.as_str()).collect();
                app.rm_allow(&args);
                app.save();
            } else if !command.block.is_empty() {
                let args: Vec<&str> = command.block.iter().map(|f| f.as_str()).collect();
                app.rm_block(&args);
                app.save();
            } else if !command.redirect.is_empty() {
                let args: Vec<&str> = command.redirect.iter().map(|f| f.as_str()).collect();
                app.rm_redirect(&args);
                app.save();
            } else if !command.sources.is_empty() {
                let args: Vec<&str> = command.sources.iter().map(|f| f.as_str()).collect();
                app.rm_sources(&args);
                app.save();
            } else if command._self {
                let bin_path = std::env::current_exe().unwrap();
                fs::remove_file(st.get_allow()).unwrap();
                println!("{:?}: removed", st.get_allow());
                fs::remove_file(st.get_block()).unwrap();
                println!("{:?}: removed", st.get_block());
                fs::remove_file(st.get_redirect()).unwrap();
                println!("{:?}: removed", st.get_redirect());
                fs::remove_file(st.get_sources()).unwrap();
                println!("{:?}: removed", st.get_sources());
                fs::remove_file(&bin_path).unwrap();
                println!("{:?}: removed", bin_path);
                app.restore_etc_host_file();
                println!("{:?}: restored", host_path());
                println!("Uninstall success...");
                std::process::exit(0);
            } else {
                unreachable!()
            }
        }
        Commands::Import { command } => {
            if !command.allow.is_empty() {
                let mut strs = Vec::with_capacity(command.allow.len());
                for p in command.allow.iter() {
                    let f = fs::read_to_string(p).unwrap_or_else(|_| {
                        panic!(
                            "{}: faild to read file: {}",
                            "ERROR".red().bold(),
                            p.to_string_lossy().italic()
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
                app.add_allow(&f);
                app.save();
            } else if !command.block.is_empty() {
                let mut strs = Vec::with_capacity(command.block.len());
                for p in command.block.iter() {
                    let f = fs::read_to_string(p).unwrap_or_else(|_| {
                        panic!(
                            "{}: faild to read file: {}",
                            "ERROR".red().bold(),
                            p.to_string_lossy().italic()
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
                app.add_block(&f);
                app.save();
            } else if !command.redirect.is_empty() {
                let mut strs = Vec::with_capacity(command.redirect.len());
                for p in command.redirect.iter() {
                    let f = fs::read_to_string(p).unwrap_or_else(|_| {
                        panic!(
                            "{}: faild to read file: {}",
                            "ERROR".red().bold(),
                            p.to_string_lossy().italic()
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
                app.add_redirect(&f);
                app.save();
            } else if !command.sources.is_empty() {
                todo!()
            } else {
                unreachable!()
            }
        }
        Commands::Export { command } => {
            if !command.allow.is_empty() {
                app.export_allow(&command.allow[0]);
            } else if !command.block.is_empty() {
                app.export_block(&command.block[0]);
            } else if !command.redirect.is_empty() {
                app.export_redirect(&command.redirect[0]);
            } else if !command.sources.is_empty() {
                app.export_sources(&command.sources[0]);
            } else {
                unreachable!()
            }
            std::process::exit(0);
        }
        Commands::Update { command } => {
            if command.sources {
                let urls = app.get_sources().iter().map(|f| f.as_str());
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
                app.clear_host();
                app.add_etc_host(hosts);
                app.save();
            } else if command._self {
                todo!()
            } else {
                unreachable!()
            }
        }
    }
}
