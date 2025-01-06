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
        while let Some(line) = self.lines.next() {
            if let Some(host) = line.split_whitespace().skip(1).next() {
                if is_valid_host(host) {
                    return Some(host);
                }
            }
        }
        None
    }
}
