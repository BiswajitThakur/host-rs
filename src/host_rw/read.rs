use std::collections::HashSet;
use std::fs::{read_to_string, File};
use std::io;
use std::io::Read;

use regex::Regex;

#[allow(dead_code)]
pub fn list(path: &str) -> Result<HashSet<String>, io::Error> {
    let is_cmt0: Regex = Regex::new(r"^\s*$").unwrap();
    let is_cmt1: Regex = Regex::new(r"^\s*#+[^#]*.*$").unwrap();
    let mut result: HashSet<String> = HashSet::new();
    for line in read_to_string(path)?.lines() {
        if !super::filter::is_comment(line, &is_cmt0, &is_cmt1) {
            result.insert(line.trim().to_string());
        };
    }
    Ok(result)
}

#[allow(dead_code)]
pub fn etc_host(path: &str) -> Result<HashSet<String>, io::Error> {
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

pub fn host(path: &str) -> Result<HashSet<String>, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(super::filter::host(contents))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list() {
        let got = list("./test_files/r0");
        assert!(got.is_ok());
        let got: HashSet<String> = got.unwrap();
        let want: HashSet<String> = HashSet::from([
            "hello.com".to_owned(),
            "www.world.in".to_owned(),
            "test.co.in".to_owned(),
            "fooo.cc".to_owned(),
            "biswajit.com".to_owned(),
            "jjj.com  kkk.com".to_owned(),
            "tmp.in".to_owned(),
        ]);
        assert_eq!(got, want);
        let got = list("path_not_exist");
        assert!(got.is_err());
        let got = list("./test_files/null");
        assert!(got.is_ok());
        let got: HashSet<String> = got.unwrap();
        let want: HashSet<String> = HashSet::new();
        assert_eq!(got, want);
    }

    #[test]
    fn test_host() {
        let got = etc_host("./test_files/h0");
        assert!(got.is_ok());
        let got: HashSet<String> = got.unwrap();
        let want: HashSet<String> = HashSet::from([
            "example.com".to_owned(),
            "google.com".to_owned(),
            //"facebook.com".to_owned(),
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
}
