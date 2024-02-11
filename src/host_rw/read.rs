use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

use regex::Regex;

pub fn etc_host(path: &str) -> io::Result<HashSet<String>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let re = Regex::new(r"(?s)\s*#+\s*BT\-start\s*#*(.*?)\#\s*BT\-end\s*#*\s*\n?").unwrap();
    if let Some(captures) = re.captures(&contents) {
        if let Some(m) = captures.get(1) {
            return Ok(super::filter::host(m.as_str().to_string()));
        };
    };
    Ok(HashSet::new())
}

pub fn host(path: &str) -> io::Result<HashSet<String>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(super::filter::host(contents))
}

pub fn etc_redirect(path: &PathBuf) -> io::Result<HashMap<String, String>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let re =
        Regex::new(r"(?s)\s*#+\s*BT\-redirect\-start\s*#*(.*?)\#\s*BT\-redirect\-end\s*#*\s*\n?")
            .unwrap();
    if let Some(captures) = re.captures(&contents) {
        if let Some(m) = captures.get(1) {
            return Ok(super::filter::redirect(m.as_str().to_string()));
        };
    };
    Ok(HashMap::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_host() {
        let got = etc_host("./test_files/h0");
        assert!(got.is_ok());
        let got: HashSet<String> = got.unwrap();
        let want: HashSet<String> = HashSet::from([
            "example.com".to_owned(),
            "google.com".to_owned(),
            "facebook.com".to_owned(),
            "testing.in".to_owned(),
            "m.com".to_owned(),
        ]);
        assert_eq!(got, want);
        let got = etc_host("./test_files/null");
        assert!(got.is_ok());
        let got: HashSet<String> = got.unwrap();
        let want: HashSet<String> = HashSet::new();
        assert_eq!(got, want);
        let got = etc_host("path_not_exist");
        assert!(got.is_err());
    }

    #[test]
    fn test_redirect() {
        let got = etc_redirect(&PathBuf::from("./test_files/h0"));
        assert!(got.is_ok());
        let got = got.unwrap();
        let want = HashMap::from([
            ("localbara.com".into(), "127.0.0.1".into()),
            ("bal6era.com".into(), "lawrachoda.com".into()),
            ("google.com".into(), "120.88.99.1".into()),
            ("globallawra.xyz".into(), "0.0.0.0".into()),
            ("99.0.0.100".into(), "khanki.in".into()),
            ("khankichoda.com".into(), "lawrachoda.com".into()),
        ]);
        assert_eq!(got, want);
        let got = etc_redirect(&PathBuf::from("./test_files/null"));
        assert!(got.is_ok());
        let got = got.unwrap();
        let want = HashMap::new();
        assert_eq!(got, want);
    }
}
