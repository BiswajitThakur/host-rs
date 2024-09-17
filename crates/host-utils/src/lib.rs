#![allow(clippy::result_large_err)]

use std::collections::HashSet;
use std::fmt::{self, Display};
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

use anyhow::Result;
use crossterm::style::Stylize;

mod list;

pub use list::HashList;
pub use list::VecList;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct H<'a>(&'a str);

impl<'a> AsRef<str> for H<'a> {
    #[inline]
    fn as_ref(&self) -> &'a str {
        self.0
    }
}
impl<'a> TryFrom<&'a str> for H<'a> {
    type Error = &'static str;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let err_msg = Err("Empty line");
        if is_comment(value) {
            return err_msg;
        };
        let v = value.split_whitespace().collect::<Vec<&str>>();
        if v.len() == 1 {
            return Ok(Self(v[0]));
        };
        if v.len() > 1 {
            return Ok(Self(v[1]));
        };
        err_msg
    }
}
impl<'a> From<H<'a>> for &'a str {
    #[inline]
    fn from(value: H<'a>) -> Self {
        value.0
    }
}
impl<'a> H<'a> {
    #[inline]
    pub fn new(value: &'a str) -> Self {
        Self(value.trim())
    }
    pub fn as_str(&'a self) -> &'a str {
        self.0
    }
    /*
    pub fn as_bytes(&'a self) -> &[u8] {
        self.0.as_bytes()
    }
    */
}

impl fmt::Display for H<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct R<'a> {
    pub from: &'a str,
    pub to: &'a str,
}

#[allow(dead_code)]
impl<'a> R<'a> {
    pub fn new(to: &'a str, from: &'a str) -> Self {
        Self { from, to }
    }
}

impl fmt::Display for R<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.to, self.from)
    }
}

impl<'a> TryFrom<&'a str> for R<'a> {
    type Error = &'static str;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let err_msg = Err("Invalid input");
        if is_comment(value) {
            return err_msg;
        };
        let v = value.split_whitespace().collect::<Vec<&str>>();
        if v.len() < 2 {
            return err_msg;
        };
        Ok(Self {
            from: v[1],
            to: v[0],
        })
    }
}

pub fn read_file<T: AsRef<Path>>(path: T) -> Result<String> {
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
/*
pub fn write_list<T, I, U>(path: T, value: I) -> Result<()>
where
    T: AsRef<Path>,
    I: IntoIterator<Item = U>,
    U: Display,
{
    let mut f = File::create(path)?;
    for line in value {
        writeln!(&mut f, "{}", line)?;
    }
    Ok(())
}
*/
pub fn write_list<'a, W: io::Write, T: Iterator<Item = &'a [u8]>>(
    w: W,
    value: T,
) -> anyhow::Result<()> {
    let mut stream = BufWriter::new(w);
    for i in value {
        stream.write_all(i)?;
        stream.write_all(b"\n")?;
    }
    stream.flush()?;
    Ok(())
}

#[cfg(test)]
mod test_write_list {
    use std::io::Cursor;

    use super::write_list;

    #[test]
    fn test_1() {
        let mut v: Vec<u8> = Vec::from(b"hello \nworld.com ");
        let input: Vec<&[u8]> = vec![b"example.com", b"hello world", b"abc"];
        let cursor = Cursor::new(&mut v);
        assert!(write_list(cursor, input.into_iter()).is_ok());
        let want = b"example.com\nhello world\nabc\n";
        assert_eq!(v, want);
    }
}

pub fn write_redirect<'a, W: io::Write, T: Iterator<Item = &'a R<'a>>>(
    w: W,
    value: T,
) -> anyhow::Result<()> {
    let mut stream = BufWriter::new(w);
    for i in value {
        stream.write_all(i.to.as_bytes())?;
        stream.write_all(b" ")?;
        stream.write_all(i.from.as_bytes())?;
        stream.write_all(b"\n")?;
    }
    stream.flush()?;
    Ok(())
}

pub fn etc_write<T, V, X>(
    path: T,
    current_content: (Vec<&H>, Vec<&R>),
    previous_content: V,
) -> Result<()>
where
    T: AsRef<Path>,
    X: AsRef<str> + Display,
    V: Iterator<Item = X> + Clone,
{
    let file = File::create(path)?;
    let mut stream = BufWriter::new(file);
    let host_b_e = ("#host-rs-beg#", "#host-rs-end#");
    let redirect_b_e = ("#r-host-rs-beg#", "#r-host-rs-end#");
    let t = b"0.0.0.0 ";
    let (h, r) = current_content;
    let mut iter = previous_content.peekable();
    while let Some(line) = iter.next() {
        let line = line.as_ref().trim();
        if line == host_b_e.0 {
            for ref v in iter.by_ref() {
                if v.as_ref().trim() == host_b_e.1 {
                    break;
                };
            }
            continue;
        };
        if line == redirect_b_e.0 {
            for ref v in iter.by_ref() {
                if v.as_ref().trim() == redirect_b_e.1 {
                    break;
                };
            }
            continue;
        };
        stream.write_all(line.as_bytes())?;
        stream.write_all(b"\n")?
    }
    stream.write_all(host_b_e.0.as_bytes())?;
    stream.write_all(b"\n")?;
    for i in h {
        stream.write_all(t)?;
        stream.write_all(i.as_str().as_bytes())?;
        stream.write_all(b"\n")?;
    }
    stream.write_all(host_b_e.1.as_bytes())?;
    stream.write_all(b"\n")?;
    stream.write_all(redirect_b_e.0.as_bytes())?;
    stream.write_all(b"\n")?;
    for i in r {
        stream.write_all(i.to.as_bytes())?;
        stream.write_all(b" ")?;
        stream.write_all(i.from.as_bytes())?;
        stream.write_all(b"\n")?;
    }
    stream.write_all(redirect_b_e.1.as_bytes())?;
    stream.write_all(b"\n")?;
    stream.flush()?;
    Ok(())
}

pub fn etc_host_reader<'a>(lines: &Vec<&'a str>, h: &mut HashSet<H<'a>>) {
    let mut host_flag = false;
    let start_host = "#host-rs-beg#";
    let end_host = "#host-rs-end#";
    for i in lines {
        let j = i.trim();
        if host_flag {
            if j == end_host {
                host_flag = false;
                continue;
            };
            if let Ok(v) = H::try_from(j) {
                h.insert(v);
            };
            continue;
        };
        if j == start_host {
            host_flag = true;
        };
    }
}

pub fn host_reader<'a>(lines: Vec<&'a str>, h: &mut HashList<H<'a>>) {
    for line in lines.iter() {
        if let Ok(v) = H::try_from(line.trim()) {
            h.push(v);
        };
    }
}

#[derive(Debug)]
pub struct StoragePath {
    allow: PathBuf,
    block: PathBuf,
    redirect: PathBuf,
    sources: PathBuf,
}

impl From<PathBuf> for StoragePath {
    #[allow(unused)]
    fn from(parent: PathBuf) -> Self {
        #[cfg(debug_assertions)]
        let parent = PathBuf::from("tests");
        Self {
            allow: [parent.clone(), "allow".into()].into_iter().collect(),
            block: [parent.clone(), "block".into()].into_iter().collect(),
            redirect: [parent.clone(), "redirect".into()].into_iter().collect(),
            sources: [parent, "soucres".into()].into_iter().collect(),
        }
    }
}

impl Default for StoragePath {
    fn default() -> Self {
        Self::new()
    }
}

impl StoragePath {
    pub fn new() -> Self {
        todo!()
    }
    pub fn get_allow(&self) -> &PathBuf {
        &self.allow
    }
    pub fn get_block(&self) -> &PathBuf {
        &self.block
    }
    pub fn get_redirect(&self) -> &PathBuf {
        &self.redirect
    }
    pub fn get_sources(&self) -> &PathBuf {
        &self.sources
    }
}

pub struct Container<'a> {
    allow: HashSet<H<'a>>,
    block: HashSet<H<'a>>,
    redirect: HashSet<R<'a>>,
    sources: HashSet<H<'a>>,
}

impl<'a> Container<'a> {
    pub fn init(allow: &'a str, block: &'a str, redirect: &'a str, soucres: &'a str) -> Self {
        Self {
            allow: HashList::from(allow).into(),
            block: HashList::from(block).into(),
            redirect: HashList::from(redirect).into(),
            sources: HashList::from(soucres).into(),
        }
    }

    pub fn allow_cap(&self) -> usize {
        self.allow.capacity()
    }
    pub fn allow_len(&self) -> usize {
        self.allow.len()
    }
    pub fn block_cap(&self) -> usize {
        self.block.capacity()
    }
    pub fn block_len(&self) -> usize {
        self.block.len()
    }
    pub fn redirect_cap(&self) -> usize {
        self.redirect.capacity()
    }
    pub fn redirect_len(&self) -> usize {
        self.redirect.len()
    }
    pub fn sources_cap(&self) -> usize {
        self.sources.capacity()
    }
    pub fn sources_len(&self) -> usize {
        self.sources.len()
    }

    pub fn get_allow(&self) -> &HashSet<H> {
        &self.allow
    }
    pub fn insert_allow(&mut self, value: &'a str) {
        self.redirect.retain(|r| r.from != value);
        if let Ok(v) = H::try_from(value) {
            self.block.remove(&v);
            self.allow.insert(v);
        };
    }
    pub fn insert_allow_h(&mut self, value: H<'a>) {
        self.redirect.retain(|r| r.from == value.as_str());
        self.block.remove(&value);
        self.allow.insert(value);
    }
    pub fn remove_allow(&mut self, value: &'a str) {
        self.allow.remove(&H::new(value));
    }

    pub fn get_block(&self) -> &HashSet<H> {
        &self.block
    }
    pub fn insert_block(&mut self, value: H<'a>) {
        self.redirect.retain(|r| r.from == value.as_str());
        self.allow.remove(&value);
        self.block.insert(value);
    }
    pub fn remove_block(&mut self, value: &'a str) {
        self.block.remove(&H::new(value));
    }

    pub fn get_redirect(&self) -> &HashSet<R> {
        &self.redirect
    }
    pub fn find_redirect(&self, value: &str) -> Option<&R> {
        self.redirect.iter().find(|r| r.from == value)
    }
    pub fn insert_redirect(&mut self, to: &'a str, from: &'a str) {
        if let Ok(v) = H::try_from(from) {
            self.allow.remove(&v);
            self.block.remove(&v);
        };
        self.redirect.insert(R::new(to, from));
    }
    pub fn push_redirect(&mut self, value: R<'a>) {
        if let Ok(v) = H::try_from(value.from) {
            self.allow.remove(&v);
            self.block.remove(&v);
        };
        self.redirect.insert(value);
    }
    pub fn remove_redirect(&mut self, value: &str) {
        self.redirect.retain(|r| r.from == value);
    }

    pub fn get_sources(&self) -> &HashSet<H> {
        &self.sources
    }
    pub fn insert_sources(&mut self, value: &'a str) {
        self.sources.insert(H::new(value));
    }
    pub fn remove_sources(&mut self, value: &'a str) {
        self.sources.remove(&H::new(value));
    }

    pub fn save(&self, paths: &StoragePath) -> Result<()> {
        let mut allow_bytes_vec = Vec::with_capacity(self.allow.len());
        for i in self.allow.iter() {
            allow_bytes_vec.push(i.as_str().as_bytes());
        }
        allow_bytes_vec.sort();
        let mut block_bytes_vec = Vec::with_capacity(self.block.len());
        for i in self.block.iter() {
            block_bytes_vec.push(i.as_str().as_bytes());
        }
        block_bytes_vec.sort();
        write_list(
            OpenOptions::new()
                .truncate(true)
                .write(true)
                .open(paths.get_allow())?,
            allow_bytes_vec.into_iter(),
        )?;
        write_list(
            OpenOptions::new()
                .truncate(true)
                .write(true)
                .open(paths.get_block())?,
            block_bytes_vec.into_iter(),
        )?;
        let mut rr = Vec::with_capacity(self.redirect.len());
        for i in self.redirect.iter() {
            rr.push(i);
        }
        rr.sort();
        write_redirect(
            OpenOptions::new()
                .truncate(true)
                .write(true)
                .open(paths.get_redirect())?,
            rr.into_iter(),
        )?;
        let mut sources_bytes_vec = Vec::with_capacity(self.sources.len());
        for i in self.sources.iter() {
            sources_bytes_vec.push(i.as_str().as_bytes());
        }
        sources_bytes_vec.sort();
        write_list(
            OpenOptions::new()
                .truncate(true)
                .write(true)
                .open(paths.get_sources())?,
            sources_bytes_vec.into_iter(),
        )?;
        Ok(())
    }
}

#[allow(unused)]
pub struct App<'a> {
    parent: StoragePath,
    storage: Container<'a>,
    etc_content_str: Vec<&'a str>,
    etc_content_h: HashSet<H<'a>>,
}

macro_rules! insert_allow_block {
    ($self:ident, <$method:ident<$args:ident>>) => {
        let mut iter = $args.into_iter();
        while let Some(u) = iter.next() {
            if let Some(v) = get_host_from_url(u) {
                $self.storage.$method(H::new(v));
            };
        }
    };
}

fn eprintln_invalid_host_or_url<T: AsRef<str>>(s: T) {
    let e = "ERROR".red().bold().to_owned();
    eprintln!(
        "{}: invalid host or url: {}",
        e,
        s.as_ref().italic().dark_red().to_owned()
    );
}

impl<'a> App<'a> {
    pub fn new(
        parent: &'static str,
        storage: Container<'a>,
        etc_content: Vec<&'a str>,
    ) -> Result<Self> {
        let parent: StoragePath = [dirs::data_dir().unwrap(), parent.into()]
            .into_iter()
            .collect::<PathBuf>()
            .into();
        let mut etc_content_h = HashSet::<H>::with_capacity(etc_content.len());
        etc_host_reader(&etc_content, &mut etc_content_h);
        let etc_content_str = etc_content;
        Ok(Self {
            parent,
            storage,
            etc_content_str,
            etc_content_h,
        })
    }
    pub fn get_sources(&self) -> &HashSet<H<'_>> {
        self.storage.get_sources()
    }
    pub fn add_allow(&mut self, args: &'a Vec<&'a str>) {
        for i in args.iter() {
            if let Some(v) = get_host_from_url(i) {
                self.etc_content_h.remove(&H::new(v));
            } else {
                eprintln_invalid_host_or_url(i);
            };
        }
        insert_allow_block!(self, <insert_allow_h<args>>);
    }
    pub fn add_block(&mut self, args: &'a [&'a str]) {
        for i in args.iter() {
            if let Some(v) = get_host_from_url(i) {
                self.etc_content_h.insert(H::new(v));
            } else {
                eprintln_invalid_host_or_url(i);
            };
        }
        insert_allow_block!(self, <insert_block<args>>);
    }
    pub fn add_redirect(&mut self, args: &'a [(&'a str, &'a str)]) {
        let iter = args.iter();
        for u in iter {
            let to = if is_valid_host(u.0) {
                Some(u.0)
            } else if is_valid_url(u.0) {
                get_host_from_url(u.0)
            } else {
                eprintln_invalid_host_or_url(u.0);
                None
            };
            let from = if is_valid_host(u.1) {
                Some(u.0)
            } else if is_valid_url(u.1) {
                get_host_from_url(u.1)
            } else {
                eprintln_invalid_host_or_url(u.1);
                None
            };
            if let (Some(u), Some(v)) = (to, from) {
                self.etc_content_h.remove(&H::new(u));
                self.etc_content_h.remove(&H::new(v));
                self.storage.insert_redirect(u, v);
            };
        }
    }
    pub fn add_etc_host<T: IntoIterator<Item = H<'a>>>(&mut self, iter: T) {
        self.etc_content_h.extend(iter)
    }
    pub fn add_sources(&mut self, args: &'a [&'a str]) {
        for i in args.iter() {
            if is_valid_url(i) {
                self.storage.insert_sources(i);
            } else {
                eprintln!(
                    "{}: invalid url: {}",
                    "ERROR".red().bold().to_owned(),
                    i.italic().dark_red().to_owned()
                );
            };
        }
    }
    pub fn rm_allow(&mut self, args: &'a [&'a str]) {
        for i in args.iter() {
            if is_valid_host(i) {
                self.storage.remove_allow(i);
            } else if let Some(v) = get_host_from_url(i) {
                self.storage.remove_allow(v);
            } else {
                eprintln_invalid_host_or_url(i);
            }
        }
    }
    pub fn rm_block(&mut self, args: &'a [&'a str]) {
        for i in args.iter() {
            if is_valid_host(i) {
                self.etc_content_h.remove(&H::new(i));
                self.storage.remove_block(i);
            } else if let Some(v) = get_host_from_url(i) {
                self.etc_content_h.remove(&H::new(v));
                self.storage.remove_block(v);
            } else {
                eprintln_invalid_host_or_url(i);
            }
        }
    }
    pub fn rm_redirect(&mut self, args: &'a [&'a str]) {
        for i in args.iter() {
            if is_valid_host(i) {
                self.storage.remove_redirect(i);
            } else if let Some(v) = get_host_from_url(i) {
                self.storage.remove_redirect(v);
            } else {
                eprintln_invalid_host_or_url(i);
            }
        }
    }
    pub fn rm_sources(&mut self, args: &'a [&'a str]) {
        for i in args.iter() {
            if is_valid_url(i) {
                self.storage.remove_sources(i);
            } else {
                eprintln!(
                    "{}: invalid url: {}",
                    "ERROR".red().bold().to_owned(),
                    i.italic().dark_red().to_owned()
                );
            }
        }
    }
    pub fn clear_host(&mut self) {
        self.etc_content_h.clear();
    }
    pub fn impoer_allow(&mut self, args: &'a [&'a str]) {
        for _ in args.iter() {}
        todo!()
    }
    pub fn impoer_block(&mut self, _args: &'a [&'a str]) {
        todo!()
    }
    pub fn impoer_redirect(&mut self, _args: &'a [&'a str]) {
        todo!()
    }
    pub fn impoer_sources(&mut self, _args: &'a [&'a str]) {
        todo!()
    }
    pub fn export_allow<T: AsRef<Path>>(&mut self, _path: T) {
        todo!()
    }
    pub fn export_block<T: AsRef<Path>>(&mut self, _path: T) {
        todo!()
    }
    pub fn export_redirect<T: AsRef<Path>>(&mut self, _path: T) {
        todo!()
    }
    pub fn export_sources<T: AsRef<Path>>(&mut self, _path: T) {
        todo!()
    }
    pub fn save(&self) {
        let e_msg = format!(
            "{}: Faild to save changes...",
            "ERROR".red().bold().to_owned()
        );
        if self.storage.save(&self.parent).is_err() {
            eprintln!("{}", e_msg);
            std::process::exit(1);
        };
        let mut h = Vec::<&H>::with_capacity(self.etc_content_h.len());
        for i in &self.etc_content_h {
            h.push(i);
        }
        h.sort();
        let mut r = Vec::<&R>::with_capacity(self.storage.get_redirect().len());
        for i in self.storage.get_redirect() {
            r.push(i);
        }
        r.sort();
        let b_len = h.len();
        let r_len = r.len();
        if etc_write(host_path(), (h, r), self.etc_content_str.iter()).is_err() {
            eprintln!("{}", e_msg);
            eprintln!(
                "{}: run as administrator privilages.",
                "NOTE".bold().yellow().to_owned()
            );
            std::process::exit(1);
        };
        println!("Total host blocked: {}", b_len + r_len);
        println!("Total Redirect host: {}", r_len);
        println!("Total host, block by you: {}", self.storage.block_len());
        println!("Total host, allowed by you: {}", self.storage.allow_len());
        println!("Total host sources: {}", self.storage.sources_len());
    }
}

pub fn get_host_from_url_or_host(value: &str) -> Option<&str> {
    if is_comment(value) {
        return None;
    };
    let v: Vec<_> = value.split_whitespace().collect();
    if v.is_empty() {
        return None;
    };
    let u = if v.len() == 1 { v[0] } else { v[1] };
    if is_valid_host(u) {
        Some(u)
    } else if is_valid_url(u) {
        get_host_from_url(u)
    } else {
        None
    }
}

pub fn is_valid_url<T: AsRef<str>>(value: T) -> bool {
    let mut value = value.as_ref();
    if let Some(v) = value.find("http") {
        if v != 0 {
            return false;
        };
        value = &value[4..];
    };
    if value.is_empty() {
        return false;
    };
    let mut iter = value.chars().peekable();
    if let Some(v) = iter.peek() {
        if v == &'s' {
            iter.next();
        };
    };
    for c in r#"://"#.chars() {
        if let Some(v) = iter.next() {
            if v != c {
                return false;
            };
        } else {
            return false;
        }
    }
    if let Some(v) = iter.next() {
        if !matches!(v, 'a'..='z' | 'A'..='Z' | '0'..='9') {
            return false;
        };
    };
    let mut present_dot = false;
    for v in iter {
        if matches!(v, '/' | '?' | '#') {
            break;
        };
        if v == '.' {
            present_dot = true;
            continue;
        };
        if !matches!(v, 'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_') {
            return false;
        };
    }
    present_dot
}

#[cfg(test)]
mod test_is_valid_url {
    use super::is_valid_url;

    #[test]
    fn test_1() {
        let input = "";
        let got = is_valid_url(input);
        let want = false;
        assert_eq!(got, want);
    }

    #[test]
    fn test_2() {
        let input = "       ";
        let got = is_valid_url(input);
        let want = false;
        assert_eq!(got, want);
    }

    #[test]
    fn test_3() {
        let input = "abcd";
        let got = is_valid_url(input);
        let want = false;
        assert_eq!(got, want);
    }

    #[test]
    fn test_4() {
        let input = "www.google.com";
        let got = is_valid_url(input);
        let want = false;
        assert_eq!(got, want);
    }

    #[test]
    fn test_5() {
        let input = "http";
        let got = is_valid_url(input);
        let want = false;
        assert_eq!(got, want);
    }

    #[test]
    fn test_6() {
        let input = "https";
        let got = is_valid_url(input);
        let want = false;
        assert_eq!(got, want);
    }

    #[test]
    fn test_7() {
        let input = "http://";
        let got = is_valid_url(input);
        let want = false;
        assert_eq!(got, want);
    }

    #[test]
    fn test_8() {
        let input = "https://";
        let got = is_valid_url(input);
        let want = false;
        assert_eq!(got, want);
    }

    #[test]
    fn test_9() {
        let input = "http://www12abc";
        let got = is_valid_url(input);
        let want = false;
        assert_eq!(got, want);
    }

    #[test]
    fn test_10() {
        let input = "https://xyz123";
        let got = is_valid_url(input);
        let want = false;
        assert_eq!(got, want);
    }

    #[test]
    fn test_11() {
        let input = "http://www.example.com";
        let got = is_valid_url(input);
        let want = true;
        assert_eq!(got, want);
    }

    #[test]
    fn test_12() {
        let input = "https://www.examplee.com";
        let got = is_valid_url(input);
        let want = true;
        assert_eq!(got, want);
    }

    #[test]
    fn test_13() {
        let input = "http://www.exam&ple.com";
        let got = is_valid_url(input);
        let want = false;
        assert_eq!(got, want);
    }

    #[test]
    fn test_14() {
        let input = "https://www.ex@mple.com";
        let got = is_valid_url(input);
        let want = false;
        assert_eq!(got, want);
    }

    #[test]
    fn test_15() {
        let input = "http://xyz.123.abc/hello%@?fooo=null";
        let got = is_valid_url(input);
        let want = true;
        assert_eq!(got, want);
    }

    #[test]
    fn test_16() {
        let input = "https://xyz.123.abc?hello%@?fooo=null";
        let got = is_valid_url(input);
        let want = true;
        assert_eq!(got, want);
    }

    #[test]
    fn test_17() {
        let input = "https://xyz.123.abc.?hello%@?fooo=null";
        let got = is_valid_url(input);
        let want = true;
        assert_eq!(got, want);
    }

    #[test]
    fn test_18() {
        let input = "https://.xyz.123.abc?hello%@?fooo=null";
        let got = is_valid_url(input);
        let want = false;
        assert_eq!(got, want);
    }

    #[test]
    fn test_19() {
        let input = "https://...../hello%@?fooo=null";
        let got = is_valid_url(input);
        let want = false;
        assert_eq!(got, want);
    }

    #[test]
    fn test_20() {
        let input = "https://xyz-123.abc/api?key=val";
        let got = is_valid_url(input);
        let want = true;
        assert_eq!(got, want);
    }

    #[test]
    fn test_21() {
        let input = "hellohttps://xyz-123.abc/api?key=val";
        let got = is_valid_url(input);
        let want = false;
        assert_eq!(got, want);
    }
}

#[inline]
fn is_valid_host<T: AsRef<str>>(value: T) -> bool {
    let value = value.as_ref();
    if value.is_empty()
        // || value.len() > 63
        || value.starts_with(' ')
        || value.ends_with(' ')
        || value.starts_with('.')
        || value.ends_with('.')
        || !value.contains('.')
    {
        return false;
    };
    for c in value.chars() {
        if c.is_ascii_uppercase()
            || c.is_ascii_lowercase()
            || c.is_ascii_digit()
            || (c == '.')
            || (c == '-')
            || (c == '_')
        {
            continue;
        } else {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test_is_valid_host {
    use super::is_valid_host;

    #[test]
    fn test_1() {
        let input = "";
        let got = is_valid_host(input);
        let want = false;
        assert_eq!(got, want);
    }

    #[test]
    fn test_2() {
        let input = "    ";
        let got = is_valid_host(input);
        let want = false;
        assert_eq!(got, want);
    }

    #[test]
    fn test_3() {
        let input = "  www.google.com";
        let got = is_valid_host(input);
        let want = false;
        assert_eq!(got, want);
    }

    #[test]
    fn test_4() {
        let input = "www.google.com";
        let got = is_valid_host(input);
        let want = true;
        assert_eq!(got, want);
    }

    #[test]
    fn test_5() {
        let input = "wwwgooglecom";
        let got = is_valid_host(input);
        let want = false;
        assert_eq!(got, want);
    }

    #[test]
    fn test_6() {
        let input = "exam.p.le.com";
        let got = is_valid_host(input);
        let want = true;
        assert_eq!(got, want);
    }

    #[test]
    fn test_7() {
        let input = "example-123.com";
        let got = is_valid_host(input);
        let want = true;
        assert_eq!(got, want);
    }

    #[test]
    fn test_8() {
        let input = ".example.com";
        let got = is_valid_host(input);
        let want = false;
        assert_eq!(got, want);
    }

    #[test]
    fn test_9() {
        let input = "example.com.";
        let got = is_valid_host(input);
        let want = false;
        assert_eq!(got, want);
    }

    #[test]
    fn test_10() {
        let input = "www.exam/ple.com";
        let got = is_valid_host(input);
        let want = false;
        assert_eq!(got, want);
    }

    #[test]
    fn test_11() {
        let input = "http://example.com";
        let got = is_valid_host(input);
        let want = false;
        assert_eq!(got, want);
    }

    #[test]
    fn test_12() {
        let input = "https://example.com";
        let got = is_valid_host(input);
        let want = false;
        assert_eq!(got, want);
    }
}

#[allow(unused)]
pub fn is_comment<T: AsRef<str>>(value: T) -> bool {
    let v = value.as_ref().trim_start();
    if v.is_empty() || v.starts_with('#') {
        return true;
    };
    false
}

#[cfg(test)]
mod test_is_commit {
    use super::is_comment;

    #[test]
    fn test_1() {
        assert!(is_comment(""));
        assert!(is_comment("     "));
        assert!(is_comment("#"));
        assert!(is_comment("####"));
        assert!(is_comment("   #"));
        assert!(is_comment("# "));
        assert!(is_comment("#    "));
        assert!(is_comment("##    "));
        assert!(is_comment("   #   "));
        assert!(is_comment("   ####    "));
        assert!(is_comment("# hel54..-+lo"));
        assert!(is_comment("    # hi..iiii"));
        assert!(is_comment("#### testing..."));
        assert!(is_comment("# te..st # ++test #"));
        assert!(is_comment("   ##3 hlo"));
        assert!(!is_comment("hii #"));
        assert!(!is_comment("    testing # testing"));
        assert!(!is_comment("    rust   #"));
        assert!(!is_comment("// hello.::+-"));
        assert!(!is_comment("<!--html#css#js#-->"));
        assert!(!is_comment("/////"));
        assert!(!is_comment("//// #### //// ####"));
    }
}

pub fn download_from_url<T: AsRef<str>>(url: T) -> Result<String, ureq::Error> {
    let url = url.as_ref();
    println!("Downloading from: {}", url.yellow().to_owned());
    let body = ureq::get(url).call()?.into_string()?;
    println!("Download success");
    Ok(body)
}

pub fn filter_host_from_vec_str(value: Vec<&str>, result_cap: usize) -> Vec<H> {
    let mut hosts = Vec::with_capacity(result_cap);
    for i in value {
        for line in i.lines() {
            if is_comment(line) {
                continue;
            };
            if let Some(ht) = get_host_from_url_or_host(line) {
                hosts.push(H::new(ht));
            } else {
                let e: Vec<_> = line.split_whitespace().collect();
                if e.len() == 1 {
                    eprintln!(
                        "Invalid host or url: {}",
                        line.italic().bold().dark_red().to_owned()
                    );
                } else {
                    let left = line.find(e[1]).unwrap();
                    let right = left + e[1].len();
                    eprintln!(
                        "Invalid host or url: {}{}{}",
                        line[0..left].red().to_owned(),
                        line[left..right].italic().bold().dark_red().to_owned(),
                        line[right..].red().to_owned()
                    );
                }
            }
        }
    }
    hosts
}

pub fn get_host_from_url<T: AsRef<str> + ?Sized>(webs: &T) -> Option<&str> {
    let mut webs = webs.as_ref().trim();
    if let Some(v) = webs.find("http://") {
        if v == 0 {
            webs = &webs[7..];
        };
    };
    if let Some(v) = webs.find("https://") {
        if v == 0 {
            webs = &webs[8..];
        };
    };
    let mut end = webs.len();
    if let Some(v) = webs.find('/') {
        if v < end {
            end = v;
        };
    };
    if let Some(v) = webs.find('?') {
        if v < end {
            end = v;
        };
    };
    if let Some(v) = webs.find(':') {
        if v < end {
            end = v;
        };
    };
    let v = &webs[..end];
    if is_valid_host(v) {
        return Some(v);
    };
    None
}

#[cfg(test)]
mod test_get_host_from_url {
    use crate::get_host_from_url;
    #[test]
    fn test_1() {
        assert_eq!(get_host_from_url(""), None);
    }
    #[test]
    fn test_2() {
        assert_eq!(get_host_from_url("   "), None);
    }
    #[test]
    fn test_3() {
        assert_eq!(get_host_from_url("hhff?lk=89"), None);
    }
    #[test]
    fn test_4() {
        assert_eq!(get_host_from_url("/q123.com?name=BT"), None);
    }
    #[test]
    fn test_5() {
        assert_eq!(get_host_from_url("example.com"), Some("example.com"));
    }
    #[test]
    fn test_6() {
        assert_eq!(get_host_from_url("http://example.com"), Some("example.com"));
    }
    #[test]
    fn test_7() {
        assert_eq!(
            get_host_from_url("https://example.com"),
            Some("example.com")
        );
    }
    #[test]
    fn test_8() {
        assert_eq!(
            get_host_from_url("https://example.com"),
            Some("example.com")
        );
    }
    #[test]
    fn test_9() {
        assert_eq!(
            get_host_from_url("http://example.com/about"),
            Some("example.com")
        );
    }
    #[test]
    fn test_10() {
        assert_eq!(
            get_host_from_url("   https://example.com/url?qq=123&hello=testing"),
            Some("example.com")
        );
    }
    #[test]
    fn test_11() {
        assert_eq!(
            get_host_from_url("   example.com/url?qq=123&hello=testing"),
            Some("example.com")
        );
    }
    #[test]
    fn test_12() {
        assert_eq!(
            get_host_from_url("https://123h.in?kk=99&m=rr"),
            Some("123h.in")
        );
        assert_eq!(get_host_from_url("123h.in?kk=99&m=rr"), Some("123h.in"));
        assert_eq!(get_host_from_url("127.0.0.1:8080"), Some("127.0.0.1"));
        assert_eq!(get_host_from_url("127.0.0.1:8080/"), Some("127.0.0.1"));
        assert_eq!(get_host_from_url("127.0.0.1:8080?"), Some("127.0.0.1"));
        assert_eq!(get_host_from_url("127.0.0.1"), Some("127.0.0.1"));
        assert_eq!(
            get_host_from_url("https://1.1.1.1:8080/home?val=http://www.example.com"),
            Some("1.1.1.1")
        );
        assert_eq!(
            get_host_from_url("hellohttps://1.1.1.1:8080/home?val=http://www.example.com"),
            None
        );
        assert_eq!(
            get_host_from_url("hello https://1.1.1.1:8080/home?val=http://www.example.com"),
            None
        );
        assert_eq!(
            get_host_from_url("  http://127.0.0.1/login?uname=test_user&pass=12345678"),
            Some("127.0.0.1")
        );
    }
    #[test]
    #[ignore = "reason"]
    fn test_13() {
        assert_eq!(
            get_host_from_url("http://https://97.7.54.10/login?uname=test_user&pass=12345678"),
            None
        );
    }
}
