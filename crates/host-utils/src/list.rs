use std::collections::HashSet;
use std::hash::Hash;

use crate::{H, R};

#[allow(unused)]
#[derive(Debug, PartialEq)]
pub struct HashList<T: Eq + Hash>(HashSet<T>);

#[allow(unused)]
impl<T> HashList<T>
where
    T: Eq + Hash,
{
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self(HashSet::with_capacity(capacity))
    }
    pub fn push(&mut self, value: T) {
        self.0.insert(value);
    }
    pub fn clear(&mut self) {
        self.0.clear();
    }
    pub fn remove(&mut self, value: &T) -> bool {
        self.0.remove(value)
    }
}

/*
impl<'a> HashList<H<'a>> {
    pub fn remove(&mut self, value: &'a str) {
        self.0.remove(&H::new(value));
    }
}

impl HashList<R<'_>> {
    pub fn remove(&mut self, value: &str) {
        self.0.retain(|r| r.from == value);
    }
}
*/
impl<T: Eq + Hash> From<HashList<T>> for HashSet<T> {
    fn from(value: HashList<T>) -> Self {
        value.0
    }
}
#[test]
fn from_hoshlist_hashset() {
    let input = HashList(HashSet::from([H::new("hello"), H::new("world")]));
    let got: HashSet<H> = input.into();
    let want = HashSet::from([H::new("hello"), H::new("world")]);
    assert_eq!(got, want);
}

impl<T: Eq + Hash> From<HashList<T>> for Vec<T> {
    fn from(value: HashList<T>) -> Self {
        let mut v = Vec::with_capacity(value.capacity());
        for i in value.0.into_iter() {
            v.push(i);
        }
        v
    }
}

impl<T: Eq + Hash> From<HashSet<T>> for HashList<T> {
    fn from(value: HashSet<T>) -> Self {
        Self(value)
    }
}
#[test]
fn from_hashset_hashlist() {
    let want = HashList(HashSet::from([H::new("hello"), H::new("world")]));
    let got: HashList<H> = HashSet::from([H::new("hello"), H::new("world")]).into();
    assert_eq!(got, want);
    let got: HashList<&str> = HashSet::from(["hello", "world"]).into();
    let want = HashList::from(HashSet::from(["hello", "world"]));
    assert_eq!(got, want);
}

impl<T: Eq + Hash> From<Vec<T>> for HashList<T> {
    fn from(value: Vec<T>) -> Self {
        let mut v = HashList::with_capacity(value.len());
        for i in value.into_iter() {
            v.push(i);
        }
        v
    }
}

impl<'a, T: Eq + Hash + TryFrom<&'a str>> From<&'a str> for HashList<T> {
    fn from(value: &'a str) -> Self {
        let lines: Vec<&str> = value.lines().collect();
        let mut r = HashSet::with_capacity(lines.len());
        for line in lines.into_iter() {
            if let Ok(v) = T::try_from(line) {
                r.insert(v);
            };
        }
        Self(r)
    }
}
#[test]
fn test_from_str_hashlist_t() {
    let input = r#"
example.com

127.0.0.1     github.com
www.google.com
            "#;
    let got = HashList::from(input);
    let want = HashList::from(HashSet::from([
        H::new("example.com"),
        H::new("github.com"),
        H::new("www.google.com"),
    ]));
    assert_eq!(got, want);
}

#[derive(Debug, PartialEq)]
pub struct VecList<T>(Vec<T>);

impl<T> VecList<T> {
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }
    pub fn push(&mut self, value: T) {
        self.0.push(value);
    }
    pub fn as_vec(self) -> Vec<T> {
        self.0
    }
}

impl<'a, T> From<&'a str> for VecList<T>
where
    T: TryFrom<&'a str>,
{
    fn from(value: &'a str) -> Self {
        let lines = value.lines();
        let mut result: Vec<T> = Vec::with_capacity(1024);
        for line in lines {
            if let Ok(v) = T::try_from(line) {
                result.push(v);
            };
        }
        Self(result)
    }
}

impl<'a> From<Vec<&'a str>> for VecList<&'a str> {
    fn from(value: Vec<&'a str>) -> Self {
        Self(value)
    }
}

impl<'a> From<Vec<H<'a>>> for VecList<H<'a>> {
    fn from(value: Vec<H<'a>>) -> Self {
        Self(value)
    }
}

impl<'a> From<Vec<R<'a>>> for VecList<R<'a>> {
    fn from(value: Vec<R<'a>>) -> Self {
        Self(value)
    }
}

impl<'a> From<HashSet<&'a str>> for VecList<&'a str> {
    fn from(value: HashSet<&'a str>) -> Self {
        let mut v = Vec::with_capacity(value.capacity());
        for i in value.iter() {
            v.push(*i);
        }
        Self(v)
    }
}

impl<T> From<VecList<T>> for Vec<T> {
    fn from(value: VecList<T>) -> Self {
        value.0
    }
}

impl<T> From<VecList<T>> for HashSet<T>
where
    T: PartialEq + Eq + Hash,
{
    fn from(value: VecList<T>) -> Self {
        let mut v: HashSet<T> = HashSet::with_capacity(value.capacity());
        for i in value.0.into_iter() {
            v.insert(i);
        }
        v
    }
}

#[cfg(test)]
mod tests {

    use crate::H;
    use crate::R;

    use super::*;

    #[test]
    fn test_list_vec_rdr() {
        let input = r#"
example.com

github.com
www.google.com
            "#;
        let rdr: VecList<H> = VecList::from(input);
        let got: Vec<H> = rdr.into();
        let want = vec![
            H::new("example.com"),
            H::new("github.com"),
            H::new("www.google.com"),
        ];
        assert_eq!(got, want);
    }

    #[test]
    fn test_list_set_rdr() {
        let input = r#"
example.com

github.com
www.google.com
            "#;
        let rdr: VecList<H> = VecList::from(input);
        let got: HashSet<H> = rdr.into();
        let want = HashSet::from([
            H::new("example.com"),
            H::new("github.com"),
            H::new("www.google.com"),
        ]);
        assert_eq!(got, want);
    }

    #[test]
    fn test_redirect_vec_rdr() {
        let input = r#"
hello   hiiii

   abcd efg  
 xyz      zzz
        "#;
        let rdr: VecList<R> = VecList::from(input);
        let got: HashSet<R> = rdr.into();
        let want: HashSet<R> = HashSet::from([
            R::new("hello", "hiiii"),
            R::new("abcd", "efg"),
            R::new("xyz", "zzz"),
        ]);
        assert_eq!(got, want);
    }
}
