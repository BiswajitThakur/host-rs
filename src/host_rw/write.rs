use std::collections::HashMap;
use std::io::Read;
use std::path::PathBuf;
use std::{
    collections::HashSet,
    fs::{File, OpenOptions},
    io::{self, Write},
};

use regex::Regex;

#[allow(dead_code)]
pub fn list(path: &PathBuf, data: &HashSet<String>) -> Result<(), io::Error> {
    let mut f = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)?;
    let mut v: Vec<&str> = data.iter().map(|i| i.as_str()).collect();
    v.sort();
    f.write_all(v.join("\n").as_bytes())?;
    f.flush()?;
    Ok(())
}

#[allow(dead_code)]
pub fn host(path: &PathBuf, data: HashSet<&str>) -> Result<(), io::Error> {
    let cap: usize = data.len();
    let mut v1: Vec<&str> = Vec::with_capacity(cap + 2);
    let mut v2: Vec<String> = data.into_iter().map(|f| format!("0.0.0.0 {f}")).collect();
    v2.sort();
    v1.push("\n# BT-start #");
    for i in &v2 {
        v1.push(i.as_str());
    }
    v1.push("# BT-end #\n");
    let con: String = v1.join("\n");
    let mut file: File = File::open(path)?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    let reg: Regex = Regex::new(r"(?s)\s*#+\s*BT\-start\s*#*(.*?)\#\s*BT\-end\s*#*\s*\n?").unwrap();
    let mut f: File = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)?;
    if !reg.is_match(&contents) {
        let new_data = format!("{}\n{}\n", contents, con);
        f.write_all(new_data.as_bytes())?;
        f.flush()?;
        return Ok(());
    };
    let new_data = reg.replace(&contents, con);
    f.write_all(new_data.as_bytes())?;
    f.flush()?;
    Ok(())
}

pub fn redirect(path: &PathBuf, data: HashMap<String, String>) -> Result<(), io::Error> {

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list() {
        let input: HashSet<String> =
            HashSet::from(["example.com".to_owned(), "abcxyz.com".to_owned()]);
        let path = &PathBuf::from("./test_files/w0");
        let got = list(path, &input);
        assert!(got.is_ok());
        let mut file = File::open(path).unwrap();
        let mut got = String::new();
        file.read_to_string(&mut got).unwrap();
        let want = String::from(
            "abcxyz.com
example.com",
        );
        assert_eq!(got, want);

        let input: HashSet<String> = HashSet::from([
            "example.com".to_owned(),
            "hello.in".to_owned(),
            "gooooogle.com".to_owned(),
            "abcxyz.com".to_owned(),
        ]);
        let path = &PathBuf::from("./test_files/w0");
        let got = list(path, &input);
        assert!(got.is_ok());
        let mut file = File::open(path).unwrap();
        let mut got = String::new();
        file.read_to_string(&mut got).unwrap();
        let want = String::from(
            "abcxyz.com
example.com
gooooogle.com
hello.in",
        );
        assert_eq!(got, want);
    }

    #[test]
    fn test_host() {
        let path = &PathBuf::from("./test_files/wh");
        let input: HashSet<&str> = HashSet::from([
            "example.com",
            "hello.in",
            "gooooogle.com",
            "abcxyz.com",
        ]);
        let got = host(path, input);
        assert!(got.is_ok());
    }
}
