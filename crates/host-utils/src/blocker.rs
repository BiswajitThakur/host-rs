use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    fmt, io,
};

use colored::Colorize;

use crate::{
    db::UserData,
    scanner::{EtcHostScanner, HostScanner},
    utils::{get_host_from_url_or_host, is_valid_url, sha256},
};

pub struct App<'a, O: io::Write, E: io::Write> {
    pub(crate) block: HashSet<Cow<'a, str>>,
    pub(crate) data: UserData<'a>,
    pub(crate) etc_hosts: Cow<'a, str>,
    pub(crate) stdout: &'a mut O,
    pub(crate) stderr: &'a mut E,
}

impl<'a, O: io::Write, E: io::Write> App<'a, O, E> {
    pub fn new<R: io::Read>(
        etc_hosts: &'a str,
        db: Option<R>,
        stdout: &'a mut O,
        stderr: &'a mut E,
    ) -> io::Result<Self> {
        let user_db = db
            .map(|v| UserData::from_read(v).unwrap_or_default())
            .unwrap_or_default();
        let mut block = HashSet::with_capacity(etc_hosts.len() / 20);
        for i in EtcHostScanner::from(etc_hosts) {
            block.insert(Cow::Borrowed(i));
        }
        Ok(Self {
            block,
            data: user_db,
            etc_hosts: Cow::Borrowed(etc_hosts),
            stdout,
            stderr,
        })
    }
    fn eprintln_invalid_host_or_url<T: fmt::Display>(&mut self, value: T) {
        let _ = writeln!(
            self.stderr,
            "ERROR: invalid host or url: {}",
            value.to_string().italic().bold().red(),
        );
        let _ = self.stderr.flush();
    }
    fn eprintln_url<T: fmt::Display>(&mut self, value: T) {
        let _ = writeln!(
            self.stderr,
            "ERROR: invalid url: {}",
            value.to_string().italic().bold().red()
        );
    }
    #[inline]
    pub fn get_sources(&self) -> &HashMap<Cow<'a, str>, [u8; 32]> {
        &self.data.sources
    }
    pub fn add_allow<T: Iterator<Item = &'a str>>(&mut self, args: T) {
        for i in args {
            if let Some(v) = get_host_from_url_or_host(i) {
                let val = Cow::Borrowed(v);
                self.block.remove(&val);
                self.data.insert_allow(val);
            } else {
                self.eprintln_invalid_host_or_url(i);
            }
        }
    }
    pub fn rm_allow<T: Iterator<Item = &'a str>>(&mut self, args: T) {
        for i in args {
            if let Some(host) = get_host_from_url_or_host(i) {
                self.data.remove_allow(host);
            } else {
                self.eprintln_invalid_host_or_url(i);
            }
        }
    }
    pub fn add_block<T: Iterator<Item = &'a str>>(&mut self, args: T) {
        for i in args {
            if let Some(v) = get_host_from_url_or_host(i) {
                let val = Cow::Borrowed(v);
                self.data.insert_block(val);
            } else {
                self.eprintln_invalid_host_or_url(i);
            }
        }
    }
    pub fn rm_block<T: Iterator<Item = &'a str>>(&mut self, args: T) {
        for i in args {
            if let Some(v) = get_host_from_url_or_host(i) {
                self.data.remove_block(v);
            } else {
                self.eprintln_invalid_host_or_url(i);
            }
        }
    }
    pub fn add_redirect<T: Iterator<Item = (&'a str, &'a str)>>(&mut self, args: T) {
        for i in args {
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
    pub fn rm_redirect<T: Iterator<Item = &'a str>>(&mut self, args: T) {
        for i in args {
            if let Some(v) = get_host_from_url_or_host(i) {
                self.data.redirect.remove(&Cow::Borrowed(v));
            } else {
                self.eprintln_invalid_host_or_url(i);
            }
        }
    }
    pub fn add_sources<T: Iterator<Item = &'a str>>(&mut self, args: T) {
        for i in args {
            if is_valid_url(i) {
                self.data.insert_sources(Cow::Borrowed(i));
            } else {
                self.eprintln_url(i);
            }
        }
    }
    pub fn rm_sources<T: Iterator<Item = &'a str>>(&mut self, args: T) {
        for i in args {
            if is_valid_url(i) {
                self.data.remove_sources(i);
            }
        }
    }
    #[allow(clippy::result_large_err)]
    fn download<T: AsRef<str>>(url: T) -> Result<String, ureq::Error> {
        ureq::get(url.as_ref()).call()?.body_mut().read_to_string()
    }
    pub fn get_update(&mut self) -> Vec<(String, String, [u8; 32])> {
        let mut v = Vec::with_capacity(self.data.sources.len());
        for (url, hash) in self.data.sources.iter() {
            let _ = writeln!(self.stdout, "Checking: {}", url.yellow());
            match Self::download(url) {
                Ok(s) => {
                    let new_hash = sha256(&s);
                    if &new_hash != hash {
                        let _ = writeln!(self.stdout, "...Update Available...\n");
                        v.push((s, url.to_string(), new_hash));
                        continue;
                    } else {
                        let _ = writeln!(self.stdout, "...Update Not Available...\n");
                        v.push((s, url.to_string(), *hash));
                    }
                }
                Err(e) => {
                    let _ = writeln!(self.stderr, "{e}");
                }
            }
        }
        v
    }
    pub fn print_etc_hosts(&mut self) -> io::Result<()> {
        for line in self.etc_hosts.lines() {
            self.stdout.write_all(line.as_bytes())?;
            self.stdout.write_all(b"\n")?;
        }
        self.stdout.flush()
    }
    pub fn get_update_fource(&mut self) -> Vec<(String, String, [u8; 32])> {
        let mut v = Vec::with_capacity(self.data.sources.len());
        for (url, _) in self.data.sources.iter() {
            let _ = writeln!(self.stdout, "Downloading From: {}", url.yellow());
            match Self::download(url) {
                Ok(s) => {
                    let hash = sha256(&s);
                    v.push((s, url.to_string(), hash));
                }
                Err(e) => {
                    let _ = writeln!(self.stderr, "{e}");
                }
            }
        }
        v
    }
    pub fn apply_update(&mut self, update: &'a [(String, String, [u8; 32])]) {
        let mut est_len = 0;
        self.block.clear();
        for (data, _, _) in update.iter() {
            est_len += data.len();
        }
        est_len /= 20;
        if self.block.capacity() < est_len {
            let _ = self.block.try_reserve(est_len - self.block.capacity());
        }
        let mut update_flag = false;
        for (data, url, hash) in update.iter() {
            for host in HostScanner::from(data.as_str()) {
                self.block.insert(Cow::Borrowed(host));
            }
            if let Some(h) = self.data.sources.get_mut(&Cow::Borrowed(url.as_str())) {
                if h != hash {
                    update_flag = true;
                }
                *h = *hash;
            }
        }
        if update_flag {
            let _ = self.stdout.write_all(b".....Update Success.....\n");
        }
    }

    pub fn clear_host(&mut self) {
        self.block.clear();
    }
    pub fn export<W: io::Write>(&mut self, w: &mut W) -> io::Result<()> {
        self.data.write(w)
    }
    // W1: Etc Hosts
    // W2: Data
    pub fn save_1<W1: io::Write, W2: io::Write>(
        &mut self,
        w1: &mut W1,
        w2: &mut W2,
    ) -> io::Result<()> {
        self.save(|| (w1, w2))
    }
    // W1: Etc Hosts
    // W2: Data
    pub fn save<W1: io::Write, W2: io::Write, F: FnOnce() -> (W1, W2)>(
        &mut self,
        f: F,
    ) -> io::Result<()> {
        let mut block = HashSet::with_capacity(self.block.len() + self.data.block.len());
        for i in self.block.iter() {
            block.insert(i.as_bytes());
        }
        for i in self.data.block.iter() {
            block.insert(i.as_bytes());
        }
        for i in self.data.allow.iter() {
            block.remove(i.as_bytes());
        }
        let mut v = Vec::with_capacity(block.len());
        v.extend(block);
        v.sort();
        let mut redirect = Vec::with_capacity(self.data.redirect.len());
        for i in self.data.redirect.iter() {
            redirect.push((i.1.as_bytes(), i.0.as_bytes()));
        }
        redirect.sort();
        let (mut etc, mut data) = f();
        write_etc_host(v, redirect, self.etc_hosts.as_ref(), &mut etc)?;
        self.data.write(&mut data)?;
        self.stdout.write_all(b".....Saved Changes.....\n")?;
        Ok(())
    }
    pub fn restore_etc_hosts<W: io::Write>(etc_hosts: &str, w: &mut W) -> io::Result<()> {
        let mut iter = etc_hosts.lines();
        while let Some(line) = iter.next() {
            match line {
                "#host-rs-beg#" => {
                    for line in iter.by_ref() {
                        if line == "#host-rs-end#" {
                            break;
                        }
                    }
                }
                "#r-host-rs-beg#" => {
                    for line in iter.by_ref() {
                        if line == "#r-host-rs-end#" {
                            break;
                        }
                    }
                }
                v => {
                    w.write_all(v.as_bytes())?;
                    w.write_all(b"\n")?;
                }
            }
        }
        w.flush()
    }
    pub fn clear_data(&mut self) {
        self.data.clear();
    }
    pub fn restore_data<W: io::Write>(w: &mut W) -> io::Result<()> {
        UserData::default().write(w)
    }
}

fn write_etc_host<W>(
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
        match line {
            v if v == block_start => {
                for line in old_etc.by_ref() {
                    if line == block_end {
                        break;
                    }
                }
            }
            v if v == r_start => {
                for line in old_etc.by_ref() {
                    if line == r_end {
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
