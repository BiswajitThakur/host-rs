use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    io::{self, ErrorKind},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct UserData<'a> {
    pub(crate) allow: HashSet<Cow<'a, str>>,
    pub(crate) block: HashSet<Cow<'a, str>>,
    pub(crate) redirect: HashMap<Cow<'a, str>, Cow<'a, str>>,
    pub(crate) sources: HashMap<Cow<'a, str>, [u8; 32]>,
}

impl<'a> UserData<'a> {
    pub(crate) fn insert_allow(&mut self, value: Cow<'a, str>) {
        self.block.remove(&value);
        self.redirect.remove(&value);
        self.allow.insert(value);
    }
    pub(crate) fn remove_allow(&mut self, value: &'a str) {
        self.allow.remove(value);
    }
    pub(crate) fn insert_block(&mut self, value: Cow<'a, str>) {
        self.redirect.remove(&value);
        self.allow.remove(&value);
        self.block.insert(value);
    }
    pub(crate) fn remove_block(&mut self, value: &'a str) {
        self.block.remove(value);
    }
    pub(crate) fn insert_redirect(&mut self, (to, from): (Cow<'a, str>, Cow<'a, str>)) {
        self.allow.remove(&from);
        self.block.remove(&from);
        self.redirect.insert(from, to);
    }
    pub(crate) fn insert_sources(&mut self, value: Cow<'a, str>) {
        self.sources.insert(value, [0; 32]);
    }
    pub(crate) fn remove_sources(&mut self, value: &'a str) {
        self.sources.remove(value);
    }
    pub(crate) fn clear(&mut self) {
        self.allow.clear();
        self.block.clear();
        self.redirect.clear();
        self.sources.clear();
    }
    pub(crate) fn write<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        if let Err(e) = rmp_serde::encode::write(w, self) {
            return Err(io::Error::new(ErrorKind::Other, format!("{e}")));
        }
        w.flush()
    }
    pub(crate) fn from_read<R: io::Read>(r: R) -> io::Result<Self> {
        let v: Result<UserData, rmp_serde::decode::Error> = rmp_serde::decode::from_read(r);
        match v {
            Ok(v) => Ok(v),
            Err(e) => Err(io::Error::new(ErrorKind::InvalidData, format!("{e}"))),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        borrow::Cow,
        collections::{HashMap, HashSet},
        io::Cursor,
    };

    use super::UserData;

    fn get_test_db() -> UserData<'static> {
        UserData {
            allow: ["example.com", "abc", "00000", "11111", "\"abc#//kk"]
                .into_iter()
                .map(|v| Cow::Borrowed(v))
                .collect::<HashSet<Cow<'_, str>>>(),
            block: ["???", "###", "qqqq", "zzzzaaa", "kknk", "{{{{{{{}}}}}}}"]
                .into_iter()
                .map(|v| Cow::Borrowed(v))
                .collect::<HashSet<Cow<'_, str>>>(),
            redirect: [
                ("localhost", "127.0.0.1"),
                ("abc", "xyz"),
                ("(((())))", "[[[[[[[[]]]]]]]"),
                ("-------------", "+++++++++"),
            ]
            .into_iter()
            .map(|(a, b)| (Cow::Borrowed(a), Cow::Borrowed(b)))
            .collect::<HashMap<Cow<'_, str>, Cow<'_, str>>>(),
            sources: [
                ("sources", [1; 32]),
                ("fff", [0; 32]),
                ("vvvvv", [10; 32]),
                (
                    "1234567890",
                    (0..32).into_iter().collect::<Vec<u8>>().try_into().unwrap(),
                ),
            ]
            .into_iter()
            .map(|(a, b)| (Cow::Borrowed(a), b))
            .collect::<HashMap<Cow<'_, str>, [u8; 32]>>(),
        }
    }

    #[test]
    fn test_user_data_clear() {
        let mut db = get_test_db();
        db.clear();
        assert_eq!(db, UserData::default());
    }

    #[test]
    fn test_user_data_read_write() {
        let db = get_test_db();
        let mut buffer = Cursor::new(Vec::new());
        db.write(&mut buffer).unwrap();
        buffer.set_position(0);
        let deserialized_db = UserData::from_read(&mut buffer).unwrap();
        assert_eq!(db, deserialized_db);
    }
    #[test]
    fn test_user_date_insert_allow_block_redirect_sources() {
        let mut db = get_test_db();
        db.insert_allow(Cow::Borrowed("hello"));
        assert_eq!(
            db.allow,
            [
                "example.com",
                "abc",
                "00000",
                "11111",
                "\"abc#//kk",
                "hello"
            ]
            .into_iter()
            .map(|v| Cow::Borrowed(v))
            .collect::<HashSet<Cow<'_, str>>>()
        );
    }
}
