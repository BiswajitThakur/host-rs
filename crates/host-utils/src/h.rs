use std::fmt;

use crate::is_comment;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct H<'a>(&'a str);

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
    fn from(value: H<'a>) -> Self {
        value.0
    }
}

impl<'a> H<'a> {
    pub fn new(value: &'a str) -> Self {
        Self(value.trim())
    }
    pub fn as_str(&'a self) -> &'a str {
        self.0
    }
}

impl fmt::Display for H<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0.0.0.0 {}", self.as_str())
    }
}
