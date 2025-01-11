use std::{
    fs,
    io::{self, BufReader, BufWriter, ErrorKind, Read, Write},
    path::{Path, PathBuf},
    process,
    time::Duration,
};

use clap::{Args, Parser, Subcommand};
use colored::Colorize;

use host_utils::App;

#[derive(Debug, Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Add host to allow list & removed from block list.
    #[arg(long, num_args = 1..)]
    allow: Vec<String>,

    /// Add to block list & remove from allow list.
    #[arg(long, num_args = 1..)]
    block: Vec<String>,

    /// Add to redirect list & remove from allow and block.
    #[arg(long, num_args = 1..)]
    redirect: Vec<String>,

    /// delete all host, host sources and restore /etc/hosts file
    #[arg(long)]
    restore: bool,

    /// Expoer user data (you can import it later).
    #[arg(long)]
    export: Option<PathBuf>,

    /// Import data
    #[arg(long)]
    import: Option<PathBuf>,
}

#[derive(Debug, Subcommand)]
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

    /// Print allow, block, redirect, etc host and host sources
    #[command(aliases = ["echo", "show"], arg_required_else_help = true)]
    Print {
        #[command(flatten)]
        command: PrintOps,
    },

    /// Update sources or self.
    #[command(arg_required_else_help = true)]
    Update {
        #[command(flatten)]
        command: UpdateOps,
    },
}

#[derive(Debug, Args)]
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

#[derive(Debug, Args)]
#[group(required = true, multiple = false)]
struct PrintOps {
    /// print allow list
    #[arg(long, required = true)]
    allow: bool,

    /// print block list
    #[arg(long, required = true)]
    block: bool,

    /// print redirect list
    #[arg(long, required = true)]
    redirect: bool,

    /// print sources
    #[arg(long, required = true)]
    sources: bool,

    /// print etc hosts
    #[arg(long, alias = "etc-host", required = true)]
    etc_hosts: bool,
}

#[derive(Debug, Args)]
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

#[derive(Debug, Args)]
#[group(required = true, multiple = false)]
struct UpdateOps {
    /// Update self
    #[arg(long = "self", required = true)]
    _self: bool,

    /// Update host sources
    #[arg(long, required = true)]
    sources: bool,
}

pub fn run() -> io::Result<()> {
    let cli = Cli::parse();
    let mut stdout = io::stdout().lock();
    let mut stderr = io::stderr().lock();

    let etc_str = etc_file_string().unwrap();
    let mut app = App::new(etc_str.as_str(), db_file(), &mut stdout, &mut stderr)?;
    app.add_allow(cli.allow.iter().map(|v| v.as_str()));
    app.add_block(cli.block.iter().map(|v| v.as_str()));
    app.add_redirect(filter_redirect(&cli.redirect).into_iter());
    let (mut app, updates) = main2(app, cli.command.as_ref());
    if let Some(update) = updates.as_ref() {
        let mut total_len = 0;
        for (v, _, _) in update {
            total_len += v.len();
        }
        app.apply_update(update, total_len / 20);
    }
    match app.save(|| {
        let etc_hosts_file = match fs::File::create(etc_hosts_path()) {
            Ok(f) => BufWriter::new(f),
            Err(e) => {
                eprintln!("{e}");
                std::process::exit(1);
            }
        };
        let db_file = match fs::File::create(db_path()) {
            Ok(f) => BufWriter::new(f),
            Err(e) => {
                eprintln!("{e}");
                std::process::exit(1);
            }
        };
        (etc_hosts_file, db_file)
    }) {
        Ok(()) => {}
        Err(e) => writeln!(stderr, "{}", e)?,
    }
    Ok(())
}

#[inline(always)]
fn etc_hosts_path() -> &'static Path {
    if cfg!(debug_assertions) {
        return Path::new("hosts");
    } else if cfg!(any(
        target_os = "linux",
        target_os = "aix",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "fuchsia",
        target_os = "hurd",
        target_os = "illumos",
        target_os = "ios",
        target_os = "netbsd",
        target_os = "nto",
        target_os = "openbsd",
        target_os = "redox",
        target_os = "tvos",
        target_os = "vxworks",
    )) {
        Path::new("/etc/hosts")
    } else if cfg!(target_os = "macos") {
        Path::new("/private/etc/hosts")
    } else if cfg!(target_os = "windows") {
        Path::new(r"C:\Windows\System32\drivers\etc\hosts")
    } else if cfg!(target_os = "android") {
        Path::new("/system/etc/hosts")
    } else if cfg!(target_os = "solaris") {
        Path::new("/etc/inet/hosts")
    } else {
        eprintln!("ERROR: Unknown OS.");
        std::process::exit(1);
    }
}

fn db_path() -> PathBuf {
    if cfg!(debug_assertions) {
        PathBuf::from("db.bin")
    } else {
        if let Some(path) = dirs::data_dir() {
            [
                &path,
                Path::new(env!("CARGO_PKG_NAME")),
                Path::new("db.bin"),
            ]
            .into_iter()
            .collect()
        } else {
            PathBuf::from("db.bin")
        }
    }
}

fn etc_file_string() -> io::Result<String> {
    let mut hosts_file = match fs::File::open(etc_hosts_path()) {
        Ok(f) => BufReader::new(f),
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                eprintln!("ERROR: file '{}' not found.", etc_hosts_path().display());
            }
            eprintln!("{e}");
            std::process::exit(1);
        }
    };
    let mut s = String::new();
    hosts_file.read_to_string(&mut s)?;
    Ok(s)
}

fn db_file<'a>() -> Option<BufReader<fs::File>> {
    let path = db_path();
    match fs::File::open(path) {
        Ok(file) => Some(BufReader::new(file)),
        Err(_) => None,
    }
}

fn filter_redirect<'a>(args: &'a Vec<String>) -> Vec<(&'a str, &'a str)> {
    if args.len() % 2 != 0 {
        eprintln!("ERROR: Envalid Arguments");
        std::process::exit(1);
    }
    let mut r = Vec::with_capacity(args.len() / 2);
    let mut iter = args.iter();
    while let (Some(u), Some(v)) = (iter.next(), iter.next()) {
        r.push((u.as_str(), v.as_str()));
    }
    r
}

fn main2<'a, O: io::Write, E: io::Write>(
    mut app: App<'a, O, E>,
    cli: Option<&'a Commands>,
) -> (App<'a, O, E>, Option<Vec<(String, String, [u8; 32])>>) {
    match cli {
        Some(cmd) => match cmd {
            Commands::Insert { command } => {
                let InsertOps {
                    allow,
                    block,
                    redirect,
                    sources,
                } = command;
                app.add_allow(allow.iter().map(|v| v.as_str()));
                app.add_block(block.iter().map(|v| v.as_str()));
                app.add_redirect(filter_redirect(&redirect).into_iter());
                if !sources.is_empty() {
                    app.add_sources(sources.iter().map(|v| v.as_str()));
                    let updates = app.get_update();
                    return (app, Some(updates));
                }
            }
            Commands::Remove { command } => {
                let RemoveOps {
                    allow,
                    block,
                    redirect,
                    sources,
                    _self,
                } = command;
                app.rm_allow(allow.iter().map(|v| v.as_str()));
                app.rm_block(block.iter().map(|v| v.as_str()));
                app.rm_redirect(redirect.iter().map(|v| v.as_str()));
                app.rm_sources(sources.iter().map(|v| v.as_str()));
                if *_self {
                    todo!()
                }
            }
            Commands::Print { command } => {
                let PrintOps {
                    allow,
                    block,
                    redirect,
                    sources,
                    etc_hosts,
                } = command;
                let ep = || eprintln!("{}", "ERROR: Faild to print.".red().bold());
                if *allow && app.print_allow().is_err() {
                    ep();
                }
                if *block && app.print_block().is_err() {
                    ep();
                }
                if *redirect && app.print_redirect().is_err() {
                    ep();
                }
                if *sources && app.print_sources().is_err() {
                    ep();
                }
                if *etc_hosts && app.print_etc_hosts().is_err() {
                    ep();
                }
            }
            Commands::Update { command } => {
                let UpdateOps { _self, sources } = command;
                if *_self {
                    let child = process::Command::new("cargo")
                        .args(["install", env!("CARGO_PKG_NAME")])
                        .stdout(io::stdout())
                        .stderr(io::stderr())
                        .status();
                    match child {
                        Ok(status) if status.success() => {
                            println!("{}", ".....Update Success.....".green().bold())
                        }
                        Err(e) if e.kind() == io::ErrorKind::NotFound => {
                            eprintln!("{}", ".....Rust not Found.....".yellow().bold());
                            std::thread::sleep(Duration::from_secs(2));
                            let _ = webbrowser::open("https://www.rust-lang.org/tools/install");
                        }
                        _ => eprintln!("{}", ".....Faild to Update.....".red().bold()),
                    }
                }
                if *sources {
                    let updates = app.get_update();
                    return (app, Some(updates));
                }
            }
        },
        None => {}
    }
    (app, None)
}
