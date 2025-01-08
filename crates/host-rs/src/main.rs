mod cli;
use std::{
    fs,
    io::{self, BufReader},
};

use host_utils::{self, App};
fn main() {
    //cli::run();
    //let hs = HostRs::new("", "db.bin", io::stdout(), io::stderr());
    let db = BufReader::new(fs::File::open("db.bin").unwrap());
}
