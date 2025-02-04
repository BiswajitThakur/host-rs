use std::str::Lines;

use crate::utils::is_valid_host;

pub struct HostScanner<'a> {
    lines: Lines<'a>,
}

impl<'a> From<&'a str> for HostScanner<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            lines: value.lines(),
        }
    }
}

impl<'a> Iterator for HostScanner<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        for line in self.lines.by_ref() {
            let line = line.trim_start();
            if line.starts_with('#') {
                continue;
            }
            if let Some(host) = line.split_whitespace().nth(1) {
                if is_valid_host(host) {
                    return Some(host);
                }
            }
        }
        None
    }
}

pub struct EtcHostScanner<'a> {
    flag: bool,
    lines: Lines<'a>,
}

impl<'a> From<&'a str> for EtcHostScanner<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            flag: false,
            lines: value.lines(),
        }
    }
}

impl<'a> Iterator for EtcHostScanner<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(line) = self.lines.next() {
            let line = line.trim_start();
            match (self.flag, line) {
                (true, "#host-rs-end#") => {
                    self.flag = false;
                    continue;
                }
                (true, line) => {
                    if line.starts_with('#') {
                        continue;
                    }
                    if let Some(host) = line.split_whitespace().nth(1) {
                        if is_valid_host(host) {
                            return Some(host);
                        }
                    }
                    continue;
                }
                (false, "#host-rs-beg#") => {
                    self.flag = true;
                    continue;
                }
                (false, "#r-host-rs-beg#") => {
                    for line in self.lines.by_ref() {
                        if line == "#r-host-rs-end#" {
                            break;
                        }
                    }
                }
                _ => {}
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{EtcHostScanner, HostScanner};

    const INPUT: &str = r#"

127.0.0.1   localhost
0.0.0.0  example.com
#0.0.0.0   www.google.com
github.com facebook.com

1.1.1.1 4.4.4.4


99.99.99.99  invalidhost

99.99.99.99  validhost.co.in

#host-rs-beg#
120.10.11.78  abc.com

#0.0.0.0 adfuture.cn
0.0.0.0 adgear.com
xyz.com 70.70.70.70

10.10.10.10 20.20.20.20
0.0.0.0 bidgear.com
#  bidr.io
www.abcxyz.com www.xyzabc.com
#host-rs-end#
www.hello.com  www.world.com

#r-host-rs-beg#
0.0.0.0 bidr.io
#host-rs-beg#
0.0.0.0 content.ad
#r-host-rs-end#
#host-rs-beg#
0.0.0.0 gdn.socdm.com
   0.0.0.0   innity.net
#host-rs-end#
0.0.0.0 llink.site
            "#;

    #[test]
    fn test_host_scanner() {
        let mut sc = HostScanner::from(INPUT);
        assert_eq!(sc.next(), Some("example.com"));
        assert_eq!(sc.next(), Some("facebook.com"));
        assert_eq!(sc.next(), Some("4.4.4.4"));
        assert_eq!(sc.next(), Some("validhost.co.in"));
        assert_eq!(sc.next(), Some("abc.com"));
        assert_eq!(sc.next(), Some("adgear.com"));
        assert_eq!(sc.next(), Some("70.70.70.70"));
        assert_eq!(sc.next(), Some("20.20.20.20"));
        assert_eq!(sc.next(), Some("bidgear.com"));
        assert_eq!(sc.next(), Some("www.xyzabc.com"));
        assert_eq!(sc.next(), Some("www.world.com"));
        assert_eq!(sc.next(), Some("bidr.io"));
        assert_eq!(sc.next(), Some("content.ad"));
        assert_eq!(sc.next(), Some("gdn.socdm.com"));
        assert_eq!(sc.next(), Some("innity.net"));
        assert_eq!(sc.next(), Some("llink.site"));
        assert_eq!(sc.next(), None);
        assert_eq!(sc.next(), None);
    }
    #[test]
    fn test_etc_host_scanner() {
        let mut sc = EtcHostScanner::from(INPUT);
        assert_eq!(sc.next(), Some("abc.com"));
        assert_eq!(sc.next(), Some("adgear.com"));
        assert_eq!(sc.next(), Some("70.70.70.70"));
        assert_eq!(sc.next(), Some("20.20.20.20"));
        assert_eq!(sc.next(), Some("bidgear.com"));
        assert_eq!(sc.next(), Some("www.xyzabc.com"));
        assert_eq!(sc.next(), Some("gdn.socdm.com"));
        assert_eq!(sc.next(), Some("innity.net"));
        assert_eq!(sc.next(), None);
        assert_eq!(sc.next(), None);
    }
}
