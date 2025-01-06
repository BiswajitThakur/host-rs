use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    fmt, fs,
    io::{self, BufReader, BufWriter},
    ops::Index,
    path::Path,
};

use colored::Colorize;

use crate::{
    db::UserData,
    scanner::HostScanner,
    utils::{filter_etc_hosts, get_host_from_url_or_host, is_valid_host, is_valid_url, sha256},
};

pub struct HostRs<'a, O: io::Write, E: io::Write> {
    pub(crate) block: HashSet<Cow<'a, str>>,
    pub(crate) data: UserData<'a>,
    pub(crate) etc_hosts: Cow<'a, str>,
    pub(crate) stdout: O,
    pub(crate) stderr: E,
}

impl<'a, O: io::Write, E: io::Write> HostRs<'a, O, E> {
    pub fn new<P: AsRef<Path>, R: io::Read>(
        etc_hosts: &'a str,
        db: R,
        stdout: O,
        stderr: E,
    ) -> io::Result<Self> {
        let user_db = UserData::from_read(db)?;
        Ok(Self {
            block: filter_etc_hosts(etc_hosts),
            data: user_db,
            etc_hosts: Cow::Borrowed(etc_hosts),
            stdout,
            stderr,
        })
    }
    fn eprintln_invalid_host_or_url<T: fmt::Display>(&mut self, value: T) {
        let _ = writeln!(
            self.stderr,
            "{}: invalid host or url: {}",
            "ERROR".red().bold(),
            value.to_string().italic().bold().red(),
        );
    }
    fn eprintln_url<T: fmt::Display>(&mut self, value: T) {
        let _ = writeln!(
            self.stderr,
            "{}: invalid url: {}",
            "ERROR".red().bold(),
            value.to_string().italic().bold().red()
        );
    }
    #[inline]
    pub fn get_sources(&self) -> &HashMap<Cow<'a, str>, [u8; 32]> {
        &self.data.sources
    }
    pub fn add_allow<T: AsRef<[&'a str]>>(&mut self, args: T) {
        for &i in args.as_ref() {
            if let Some(v) = get_host_from_url_or_host(i) {
                let val = Cow::Borrowed(v);
                self.block.remove(&val);
                self.data.insert_allow(val);
            } else {
                self.eprintln_invalid_host_or_url(i);
            }
        }
    }
    pub fn add_block<T: AsRef<[&'a str]>>(&mut self, args: T) {
        for &i in args.as_ref() {
            if let Some(v) = get_host_from_url_or_host(i) {
                let val = Cow::Borrowed(v);
                self.data.insert_block(val);
            } else {
                self.eprintln_invalid_host_or_url(i);
            }
        }
    }
    pub fn add_redirect<T: AsRef<[(&'a str, &'a str)]>>(&mut self, args: T) {
        for i in args.as_ref() {
            if let Some(to) = get_host_from_url_or_host(i.0) {
                if let Some(from) = get_host_from_url_or_host(i.1) {
                    let to = Cow::Borrowed(to);
                    let from = Cow::Borrowed(from);
                    self.block.remove(&to);
                    self.block.remove(&from);
                    self.data.insert_redirect((to, from));
                } else {
                    self.eprintln_invalid_host_or_url(i.1);
                }
            } else {
                self.eprintln_invalid_host_or_url(i.0);
            }
        }
    }
    pub fn add_sources<T: AsRef<[&'a str]>>(&mut self, args: T) {
        for &i in args.as_ref() {
            if is_valid_url(i) {
                self.data.insert_sources(Cow::Borrowed(i));
            } else {
                self.eprintln_url(i);
            }
        }
    }
    pub fn rm_sources<T: AsRef<[&'a str]>>(&mut self, args: T) {
        for &i in args.as_ref() {
            if is_valid_url(i) {
                let url = Cow::Borrowed(i);
                self.data.remove_sources(&url);
            }
        }
    }
    fn download<T: AsRef<str>>(url: T) -> Result<String, ureq::Error> {
        Ok(ureq::get(url.as_ref()).call()?.into_string()?)
    }
    pub fn get_update(&mut self) -> Vec<(String, &Cow<'a, str>, [u8; 32])> {
        let mut v = Vec::with_capacity(self.data.sources.len());
        for (url, hash) in self.data.sources.iter() {
            // TODO: print update info
            match Self::download(url) {
                Ok(s) => {
                    let new_hash = sha256(&s);
                    if &new_hash != hash {
                        v.push((s, url, new_hash));
                    }
                }
                Err(e) => {
                    let _ = writeln!(self.stderr, "{e}");
                }
            }
        }
        v
    }
    pub fn get_update_fource(&mut self) -> (Vec<(String, &Cow<'a, str>, [u8; 32])>, usize) {
        let mut v = Vec::with_capacity(self.data.sources.len());
        let mut cap = 0;
        for (url, _) in self.data.sources.iter() {
            // TODO: print update info
            match Self::download(url) {
                Ok(s) => {
                    cap += s.len() / 20;
                    let hash = sha256(&s);
                    v.push((s, url, hash));
                }
                Err(e) => {
                    let _ = writeln!(self.stderr, "{e}");
                }
            }
        }
        (v, cap)
    }
    pub fn apply_update(&mut self, update: &'a Vec<(String, &Cow<'a, str>, [u8; 32])>, cap: usize) {
        let _ = self.block.try_reserve(cap);
        for (data, url, hash) in update.iter() {
            for host in HostScanner::from(data.as_str()) {
                self.block.insert(Cow::Borrowed(host));
            }
            if let Some(h) = self.data.sources.get_mut(*url) {
                *h = *hash;
            }
        }
    }
    pub fn rm_allow<T: AsRef<[&'a str]>>(&mut self, args: T) {
        for i in args.as_ref() {
            if let Some(host) = get_host_from_url_or_host(i) {
                self.data.remove_allow(&Cow::Borrowed(host));
            } else {
                self.eprintln_invalid_host_or_url(i);
            }
        }
    }
    pub fn clear_host(&mut self) {
        self.block.clear();
    }
    pub fn export<W: io::Write>(&mut self, w: &mut W) -> io::Result<()> {
        self.data.write(w)
    }
    pub fn save<W1: io::Write, W2: io::Write>(
        &self,
        etc_hosts: &mut W1,
        data: &mut W2,
    ) -> io::Result<()> {
        let mut block = Vec::with_capacity(self.block.len() + self.data.block.len());
        for i in self.block.iter() {
            if self.data.allow.get(i).is_none() {
                block.push(i.as_bytes());
            }
        }
        for i in self.data.block.iter() {
            block.push(i.as_bytes());
        }
        block.sort();
        let mut redirect = Vec::with_capacity(self.data.redirect.len());
        for i in self.data.redirect.iter() {
            redirect.push((i.1.as_bytes(), i.0.as_bytes()));
        }
        redirect.sort();
        write_etc_host(block, redirect, self.etc_hosts.as_ref(), etc_hosts)?;
        self.data.write(data)?;
        Ok(())
    }
    pub fn restore_etc_hosts<W: io::Write>(&mut self, w: &mut W) -> io::Result<()> {
        todo!()
    }
    pub fn clear_data(&mut self) {
        todo!()
    }
}

fn write_etc_host<'a, W>(
    block: Vec<&[u8]>,
    redirect: Vec<(&[u8], &[u8])>,
    old_etc: &str,
    etc_file: &mut W,
) -> io::Result<()>
where
    W: io::Write,
{
    let block_start = b"#host-rs-beg#";
    let block_end = b"#host-rs-end#";
    let r_start = b"#r-host-rs-beg#";
    let r_end = b"#r-host-rs-end#";
    let mut old_etc = old_etc.lines().map(|v| v.as_bytes());
    while let Some(line) = old_etc.next() {
        match line.as_ref() {
            v if v == block_start => {
                while let Some(line) = old_etc.next() {
                    if line.as_ref() == block_end {
                        break;
                    }
                }
            }
            v if v == r_start => {
                while let Some(line) = old_etc.next() {
                    if line.as_ref() == r_end {
                        break;
                    }
                }
            }
            v => {
                etc_file.write_all(v)?;
                etc_file.write_all(b"\n")?;
            }
        }
    }
    etc_file.write_all(block_start)?;
    etc_file.write_all(b"\n")?;
    for i in block {
        etc_file.write_all(b"0.0.0.0 ")?;
        etc_file.write_all(i.as_ref())?;
        etc_file.write_all(b"\n")?;
    }
    etc_file.write_all(block_end)?;
    etc_file.write_all(b"\n")?;
    etc_file.write_all(r_start)?;
    etc_file.write_all(b"\n")?;
    for i in redirect {
        etc_file.write_all(i.0.as_ref())?;
        etc_file.write_all(b" ")?;
        etc_file.write_all(i.1.as_ref())?;
        etc_file.write_all(b"\n")?;
    }
    etc_file.write_all(r_end)?;
    etc_file.write_all(b"\n")?;
    etc_file.flush()?;
    Ok(())
}
