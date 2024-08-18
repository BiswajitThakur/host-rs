use std::fmt;

use crate::is_comment;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
