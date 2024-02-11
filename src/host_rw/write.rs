use std::collections::{HashMap, HashSet};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::PathBuf;

use regex::Regex;

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

pub fn redirect(path: &PathBuf, data: &HashMap<String, String>) -> io::Result<()> {
    let mut file: File = File::open(path)?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    let mut f: File = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)?;
    let cap = data.len();
    let mut tmp = Vec::with_capacity(cap);
    for (key, val) in data {
        tmp.push(format!("{val} {key}"));
    };
    tmp.sort();
    let mut d = Vec::with_capacity(cap + 2);
    d.push("\n# BT-redirect-start #".to_string());
    for i in tmp {
        d.push(i);
    }
    d.push("# BT-redirect-end #\n".to_string());
    let con: String = d.join("\n");
    let reg: Regex =
        Regex::new(r"(?s)\s*#+\s*BT\-redirect\-start\s*#*(.*?)\#\s*BT\-redirect\-end\s*#*\s*\n?")
            .unwrap();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_host() {
        let path = &PathBuf::from("./test_files/wh");
        let input: HashSet<&str> =
            HashSet::from(["example.com", "hello.in", "gooooogle.com", "abcxyz.com"]);
        let got = host(path, input);
        assert!(got.is_ok());
    }

    #[test]
    fn test_redirect() {
        let path = &PathBuf::from("./test_files/wh");
        let input = HashMap::from([
            ("localbara.com".into(), "127.0.0.1".into()),
            ("bal6era.com".into(), "lawrachoda.com".into()),
            ("google.com".into(), "120.88.99.1".into()),
            ("globallawra.xyz".into(), "0.0.0.0".into()),
            ("99.0.0.100".into(), "khanki.in".into()),
            ("khankichoda.com".into(), "lawrachoda.com".into()),
        ]);
        let got = redirect(path, &input);
        assert!(got.is_ok());
    }
}
