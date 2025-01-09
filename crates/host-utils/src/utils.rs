use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
};

use sha2::Digest;

pub(crate) fn is_valid_host<T: AsRef<str>>(value: T) -> bool {
    let value = value.as_ref();
    if value.is_empty()
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
pub(crate) fn get_host_from_url<'a, T: AsRef<str> + ?Sized>(webs: &'a T) -> Option<&'a str> {
    let url = webs.as_ref().trim_start();
    let f = |v: &'a str| {
        if let Some((host, _)) = v.split_once(['/', '?', '#', ':']) {
            if is_valid_host(host) {
                Some(host)
            } else {
                None
            }
        } else if is_valid_host(v) {
            Some(v)
        } else {
            None
        }
    };
    if url.starts_with("http") {
        url.split_once("://").and_then(|(left, right)| {
            if !matches!(left, "http" | "https") {
                None
            } else {
                f(right)
            }
        })
    } else {
        f(url)
    }
}

pub(crate) fn is_valid_url<T: AsRef<str>>(value: T) -> bool {
    value
        .as_ref()
        .split_once(r"://")
        .and_then(|(l, r)| {
            if !matches!(l, "http" | "https") {
                None
            } else {
                r.split_once('.')
            }
        })
        .and_then(|(l, r)| {
            if r.len() < 2
                || l.is_empty()
                || l.chars().any(|c| {
                    !matches!(&c,
                        'a'..='z' | 'A'..='Z' |
                        '0'..='9' | '_' | '-'
                    )
                })
            {
                return None;
            }
            if let Some((i, c)) = r.chars().enumerate().find(|(_, c)| {
                matches!(c, '/' | '?' | ':' | '#')
                    || !matches!(c,
                      'a'..='z' | 'A'..='Z' |
                      '0'..='9' | '.' | '_' | '-'
                    )
            }) {
                match (i, c) {
                    (_, 'a'..='z' | 'A'..='Z' | '0'..='9' | '.' | '_' | '-' | '/' | '?' | '#') => {
                        Some(())
                    }
                    (index, ':') => {
                        let mut port: u16 = 0;
                        let zero = '0' as u16;
                        for i in r.chars().skip(index + 1) {
                            match i {
                                '0'..='9' => {
                                    let v = i as u16 - zero;
                                    if let Some(v) = port.checked_mul(10) {
                                        port = v;
                                    } else {
                                        return None;
                                    }
                                    if let Some(u) = port.checked_add(v) {
                                        port = u;
                                    } else {
                                        return None;
                                    }
                                }
                                '/' | '?' | '#' => break,
                                _ => return None,
                            }
                        }
                        Some(())
                    }
                    _ => None,
                }
            } else {
                Some(())
            }
        })
        .is_some()
}

pub(crate) fn get_host_from_url_or_host(value: &str) -> Option<&str> {
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

pub(crate) fn is_comment<T: AsRef<str>>(value: T) -> bool {
    let v = value.as_ref().trim_start();
    if v.is_empty() || v.starts_with('#') {
        return true;
    };
    false
}

pub(crate) fn filter_etc_hosts<'a>(value: &'a str) -> HashSet<Cow<'a, str>> {
    let mut hosts = HashSet::with_capacity(value.len() / 20);
    // let mut redirect = HashMap::with_capacity(100);
    let mut iter = value.lines();
    while let Some(line) = iter.next() {
        match line.trim() {
            "#host-rs-beg#" => {
                while let Some(v) = iter.next() {
                    match v.trim() {
                        "#host-rs-end#" => break,
                        u if is_comment(u) => continue,
                        u => {
                            if let Some(host) = u.split_whitespace().skip(1).next() {
                                if is_valid_host(host) {
                                    hosts.insert(Cow::Borrowed(host));
                                }
                            }
                        }
                    }
                }
            }
            /*
            "#r-host-rs-beg#" => {
                while let Some(v) = iter.next() {
                    match v.trim() {
                        "#r-host-rs-end#" => break,
                        u if is_comment(u) => continue,
                        u => {
                            let mut i = u.split_whitespace();
                            if let (Some(to), Some(from)) = (i.next(), i.next()) {
                                if is_valid_host(to) && is_valid_host(from) {
                                    redirect.insert(Cow::Borrowed(from), Cow::Borrowed(to));
                                }
                            }
                        }
                    }
                }
            }
            */
            _ => {}
        }
    }
    hosts
}

pub(crate) fn sha256<T: AsRef<str>>(value: T) -> [u8; 32] {
    let mut hasher = sha2::Sha256::new();
    hasher.update(value.as_ref().as_bytes());
    hasher.finalize().into()
}

#[cfg(test)]
mod tests {}
