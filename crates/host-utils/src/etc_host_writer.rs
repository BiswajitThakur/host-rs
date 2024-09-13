use std::fmt::Display;
use std::io::{BufWriter, Write};
use std::{fs::File, path::Path};

use crate::{H, R};

use anyhow::Result;

/*
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
    let mut file = File::create(path)?;
    let host_b_e = ("#host-rs-beg#", "#host-rs-end#");
    let redirect_b_e = ("#r-host-rs-beg#", "#r-host-rs-end#");
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
        writeln!(&mut file, "{}", line)?;
    }
    writeln!(&mut file, "{}", host_b_e.0)?;
    for i in h {
        writeln!(&mut file, "{}", i)?;
    }
    writeln!(&mut file, "{}", host_b_e.1)?;
    writeln!(&mut file, "{}", redirect_b_e.0)?;
    for i in r {
        writeln!(&mut file, "{}", i)?;
    }
    writeln!(&mut file, "{}", redirect_b_e.1)?;
    Ok(())
}

*/

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
    let host_b_e = (b"#host-rs-beg#", b"#host-rs-end#");
    let redirect_b_e = (b"#r-host-rs-beg#", b"#r-host-rs-end#");
    let t = b"0.0.0.0 ";
    let (h, r) = current_content;
    let mut iter = previous_content.peekable();
    while let Some(line) = iter.next() {
        let line = line.as_ref().trim();
        if line.as_bytes() == host_b_e.0 {
            for ref v in iter.by_ref() {
                if v.as_ref().trim().as_bytes() == host_b_e.1 {
                    break;
                };
            }
            continue;
        };
        if line.as_bytes() == redirect_b_e.0 {
            for ref v in iter.by_ref() {
                if v.as_ref().trim().as_bytes() == redirect_b_e.1 {
                    break;
                };
            }
            continue;
        };
        stream.write_all(line.as_bytes())?;
        stream.write_all(b"\n")?
    }
    stream.write_all(host_b_e.0)?;
    stream.write_all(b"\n")?;
    for i in h {
        stream.write_all(t)?;
        stream.write_all(i.as_str().as_bytes())?;
        stream.write_all(b"\n")?;
    }
    stream.write_all(host_b_e.1)?;
    stream.write_all(b"\n")?;
    stream.write_all(redirect_b_e.0)?;
    stream.write_all(b"\n")?;
    for i in r {
        stream.write_all(i.to.as_bytes())?;
        stream.write_all(b" ")?;
        stream.write_all(i.from.as_bytes())?;
        stream.write_all(b"\n")?;
    }
    stream.write_all(redirect_b_e.1)?;
    stream.write_all(b"\n")?;
    stream.flush()?;
    Ok(())
}
