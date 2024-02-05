#[allow(dead_code)]
pub fn is_admin() -> bool {
    let host_path: std::path::PathBuf = super::paths::host_path().unwrap();
    let f = std::fs::File::options()
        .read(true)
        .write(true)
        .open(host_path);
    f.is_ok()
}
