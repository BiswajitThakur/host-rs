use crate::{VecList, H, R};
use std::io::Write;
use std::{error::Error, fs::File, path::Path};

pub fn etc_write<T: AsRef<Path>>(
    path: T,
    current_content: (VecList<H>, VecList<R>),
    previous_content: &str,
) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(path)?;
    let start_host = "#host-rs-beg#";
    let end_host = "#host-rs-end#";
    let start_redirect = "#r-host-rs-beg#";
    let end_redirect = "#r-host-rs-end#";
    let (h, r) = current_content;
    let mut lines = previous_content.lines();
    let iter = &mut lines;
    while let Some(line) = iter.next() {
        let line = line.trim();
        if line == start_host {
            loop {
                let v = iter.next();
                if (v.is_some() && (v.unwrap().trim() == end_host)) || v.is_none() {
                    break;
                };
            }
            continue;
        };
        if line == start_redirect {
            loop {
                let v = iter.next();
                if (v.is_some() && (v.unwrap().trim() == end_redirect)) || v.is_none() {
                    break;
                };
            }
            continue;
        };
        writeln!(&mut file, "{}", line)?;
    }
    writeln!(&mut file, "{}", start_host)?;
    for i in h.as_vec().iter() {
        writeln!(&mut file, "{}", i)?;
    }
    writeln!(&mut file, "{}", end_host)?;
    writeln!(&mut file, "{}", start_redirect)?;
    for i in r.as_vec().iter() {
        writeln!(&mut file, "{}", i)?;
    }
    writeln!(&mut file, "{}", end_redirect)?;
    Ok(())
}
