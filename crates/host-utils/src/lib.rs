mod comment;
mod etc_host_writer;
mod h;
mod host_collections;
mod host_reader;
mod list;
mod r;
mod storage_path;

use std::fs::OpenOptions;
use std::fs::{self, File};
use std::io::ErrorKind;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

pub use comment::is_comment;
pub use etc_host_writer::etc_write;
pub use h::H;
pub use host_reader::{etc_host_reader, host_reader};
pub use list::HashList;
pub use list::VecList;
pub use r::R;

pub enum Cap {
    Capacity(usize),
    None,
}

pub fn read_file<T: AsRef<Path>>(path: T) -> std::io::Result<String> {
    let path = path.as_ref();
    if path.exists() {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    } else {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        };
        File::create_new(path)?;
        Ok(String::new())
    }
}

pub fn host_path() -> PathBuf {
    #[cfg(debug_assertions)]
    let host_path: PathBuf = ["tests", "etc", "hosts"].iter().collect();
    #[cfg(not(debug_assertions))]
    #[cfg(any(target_os = "android", target_os = "linux"))]
    let host_path: PathBuf = PathBuf::from(r"/etc/hosts");
    #[cfg(not(debug_assertions))]
    #[cfg(target_os = "macos")]
    let host_path: PathBuf = PathBuf::from(r"/private/etc/hosts");
    #[cfg(not(debug_assertions))]
    #[cfg(target_os = "windows")]
    let host_path: PathBuf = PathBuf::from(r"c:\Windows\System32\Drivers\etc\hosts");
    #[cfg(not(debug_assertions))]
    #[cfg(not(any(
        target_os = "android",
        target_os = "linux",
        target_os = "macos",
        target_os = "windows",
    )))]
    return unreachable!();
    host_path
}
