use csv;
use serde::{Deserialize, Serialize};
use std::{collections::{HashMap, HashSet}, error::Error, fs::File, path::PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct List {
    #[serde(rename = "List")]
    pub list: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Redirect {
    #[serde(rename = "C0")]
    pub c0: String,
    #[serde(rename = "C1")]
    pub c1: String,
}

pub fn read_list(path: &PathBuf) -> Result<HashSet<String>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);
    let iter = rdr.deserialize();
    let t: Vec<_> = iter.into_iter().collect();
    let mut result = HashSet::<String>::with_capacity(t.len());
    for i in t {
        let record: List = i?;
        result.insert(record.list);
    }
    Ok(result)
}

pub fn read_redirect(path: &PathBuf) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);
    let iter = rdr.deserialize();
    let mut result = HashMap::<String, String>::new();
    for i in iter {
        let record: Redirect = i?;
        result.insert(record.c1, record.c0);
    }
    Ok(result)
}

pub fn write_list(path: &PathBuf, vals: &HashSet<String>) -> Result<(), Box<dyn Error>> {
    let mut list: Vec<&String> = vals.into_iter().collect();
    list.sort();
    let mut wtr = csv::Writer::from_path(path)?;
    for i in list {
        wtr.serialize(List { list: i.clone() })?;
    }
    wtr.flush()?;
    Ok(())
}

pub fn write_redirect(
    path: &PathBuf,
    vals: &HashMap<String, String>,
) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(path)?;
    for (i, j) in vals {
        wtr.serialize(Redirect {
            c0: j.clone(),
            c1: i.clone(),
        })?;
    }
    wtr.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_read_write() {
        let path: PathBuf = ["test_files", "list.read.write.csv"].iter().collect();
        let vals = HashSet::new();
        let w = write_list(&path, &vals);
        assert!(w.is_ok());
        let r = read_list(&path);
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), HashSet::<String>::new());
        let vals = [
            "example.com".to_string(),
            "google.com".into(),
            "facebook.com".into(),
            "gcect.ac.in".into(),
        ].into();
        let w = write_list(&path, &vals);
        assert!(w.is_ok());
        let r = read_list(&path);
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), vals);
    }

    #[test]
    fn test_read_write_redirect() {
        let path: PathBuf = ["test_files", "redirect.read.write.csv"].iter().collect();
        let vals = HashMap::<String, String>::new();
        let w = write_redirect(&path, &vals);
        assert!(w.is_ok());
        let r = read_redirect(&path);
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), vals);
        let vals = HashMap::from([
            ("facebook.com".to_string(), "google.com".to_string()),
            ("youtube.com".into(), "google.com".into()),
            ("example.com".into(), "gcect.ac.in".into()),
        ]);
        let w = write_redirect(&path, &vals);
        assert!(w.is_ok());
        let r = read_redirect(&path);
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), vals);
    }
}
