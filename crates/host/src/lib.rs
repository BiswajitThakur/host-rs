use host_utils::{etc_host_reader, HashList, H, R};
use storage::{Container, StoragePath};

#[allow(unused)]
pub struct App<'a> {
    storage: Container<'a>,
    host: HashList<H<'a>>,
    redirect: HashList<R<'a>>,
}

impl<'a> App<'a> {
    pub fn init_add_block(storage: Container<'a>, etc_host: &'a str) -> Self {
        todo!()
    }

    pub fn insert_allow(&mut self, value: &'a H) {
        self.host.remove(&value);
        //self.redirect.remove(&value);
        self.storage.insert_allow(value.as_str());
    }
    pub fn remove_allow(&mut self, value: &'a str) {
        self.storage.remove_allow(&value);
    }
    //pub fn insert_block(&mut se)

    pub fn save(&self, path: StoragePath) -> std::io::Result<()> {
        todo!()
    }
}
