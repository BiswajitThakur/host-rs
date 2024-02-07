#[cfg(not(debug_assertions))]
use dirs;
use std::{collections::HashMap, path::PathBuf};

#[allow(dead_code)]
pub fn host_path() -> Result<PathBuf, String> {
    #[cfg(debug_assertions)]
    let host_path: PathBuf = ["test_files", "hosts"].iter().collect();
    #[cfg(not(debug_assertions))]
    #[cfg(any(target_os = "android", target_os = "linux"))]
    let host_path: PathBuf = PathBuf::from(r"/etc/hosts");
    #[cfg(not(debug_assertions))]
    #[cfg(target_os = "macos")]
    let host_path: PathBuf = PathBuf::from(r"/private/etc/hosts");
    #[cfg(not(debug_assertions))]
    #[cfg(target_os = "windows")]
    let host_path: PathBuf = PathBuf::from(r"c:\Windows\System32\Drivers\etc\hosts");
    return Ok(host_path);
    #[cfg(not(debug_assertions))]
    #[cfg(not(any(
        target_os = "android",
        target_os = "linux",
        target_os = "macos",
        target_os = "windows",
    )))]
    Err(String::from("Faild to detect OS"))
}

pub fn paths() -> HashMap<String, PathBuf> {
    let mut pths: HashMap<String, PathBuf> = HashMap::with_capacity(10);
    #[cfg(not(debug_assertions))]
    let home: PathBuf = dirs::home_dir().unwrap();
    #[cfg(debug_assertions)]
    let home: PathBuf = PathBuf::from("test_files");
    #[cfg(not(debug_assertions))]
    let pkg_data = PathBuf::from(r".host-rs");
    #[cfg(debug_assertions)]
    let pkg_data = PathBuf::from(r"tmp_files");
    let data_dir: &PathBuf = &[home, pkg_data].iter().collect();
    pths.insert("host_path".to_string(), host_path().unwrap());
    pths.insert("data_dir".to_string(), data_dir.clone());
    pths.insert(
        "allow".to_string(),
        [data_dir, &"allow.csv".into()].iter().collect(),
    );
    pths.insert(
        "block".to_string(),
        [data_dir, &"block.csv".into()].iter().collect(),
    );
    pths.insert(
        "redirect".to_string(),
        [data_dir, &"redirect.csv".into()].iter().collect(),
    );
    pths.insert(
        "ads".to_string(),
        [data_dir, &"ads.csv".into()].iter().collect(),
    );
    pths.insert(
        "porn".to_string(),
        [data_dir, &"porn.csv".into()].iter().collect(),
    );
    pths.insert(
        "fakenews".to_string(),
        [data_dir, &"fakenews.csv".into()].iter().collect(),
    );
    pths.insert(
        "social".to_string(),
        [data_dir, &"social.csv".into()].iter().collect(),
    );
    pths.insert(
        "gambling".to_string(),
        [data_dir, &"gambling.csv".into()].iter().collect(),
    );
    pths.insert(
        "db".to_string(),
        [data_dir, &"db.json".into()].iter().collect(),
    );
    pths
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(not(debug_assertions))]
    use dirs;

    #[test]
    fn test_host_path() {
        #[cfg(not(debug_assertions))]
        #[cfg(target_os = "macos")]
        assert_eq!(host_path().unwrap(), PathBuf::from(r"/private/etc/hosts"));
        #[cfg(not(debug_assertions))]
        #[cfg(target_os = "linux")]
        assert_eq!(host_path().unwrap(), PathBuf::from("/etc/hosts"));
        #[cfg(not(debug_assertions))]
        #[cfg(target_os = "android")]
        assert_eq!(host_path().unwrap(), PathBuf::from("/etc/hosts"));
        #[cfg(not(debug_assertions))]
        #[cfg(target_os = "windows")]
        assert_eq!(host_path().unwrap(), PathBuf::from(r"/private/etc/hosts"));
    }

    #[test]
    fn test_paths() {
        let p: HashMap<String, PathBuf> = paths();
        let paths_got = vec![
            p.get("data_dir").unwrap().clone(),
            p.get("allow").unwrap().clone(),
            p.get("block").unwrap().clone(),
            p.get("redirect").unwrap().clone(),
            p.get("ads").unwrap().clone(),
            p.get("porn").unwrap().clone(),
            p.get("fakenews").unwrap().clone(),
            p.get("social").unwrap().clone(),
            p.get("gambling").unwrap().clone(),
            p.get("db").unwrap().clone(),
        ];
        #[cfg(debug_assertions)]
        let data_dir_want: PathBuf = ["test_files", "tmp_files"].iter().collect();
        #[cfg(not(debug_assertions))]
        let data_dir_want: PathBuf = [dirs::home_dir().unwrap(), ".host-rs".into()]
            .iter()
            .collect();
        let paths_want = vec![
            data_dir_want.clone(),
            [&data_dir_want, &"allow.csv".into()].iter().collect(),
            [&data_dir_want, &"block.csv".into()].iter().collect(),
            [&data_dir_want, &"redirect.csv".into()].iter().collect(),
            [&data_dir_want, &"ads.csv".into()].iter().collect(),
            [&data_dir_want, &"porn.csv".into()].iter().collect(),
            [&data_dir_want, &"fakenews.csv".into()].iter().collect(),
            [&data_dir_want, &"social.csv".into()].iter().collect(),
            [&data_dir_want, &"gambling.csv".into()].iter().collect(),
            [&data_dir_want, &"db.json".into()].iter().collect(),
        ];
        assert_eq!(paths_got, paths_want);
    }
}
