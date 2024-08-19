use std::collections::HashSet;

use host_utils::{HashList, H, R};
mod utils;
use crate::utils::write_list;
pub use crate::utils::StoragePath;

pub struct Container<'a> {
    allow: HashSet<H<'a>>,
    block: HashSet<H<'a>>,
    redirect: HashSet<R<'a>>,
    sources: HashSet<H<'a>>,
}

impl<'a> Container<'a> {
    pub fn init(allow: &'a str, block: &'a str, redirect: &'a str, soucres: &'a str) -> Self {
        Self {
            allow: HashList::from(allow).into(),
            block: HashList::from(block).into(),
            redirect: HashList::from(redirect).into(),
            sources: HashList::from(soucres).into(),
        }
    }

    pub fn allow_cap(&self) -> usize {
        self.allow.capacity()
    }
    pub fn allow_len(&self) -> usize {
        self.allow.len()
    }
    pub fn block_cap(&self) -> usize {
        self.block.capacity()
    }
    pub fn block_len(&self) -> usize {
        self.block.len()
    }
    pub fn redirect_cap(&self) -> usize {
        self.redirect.capacity()
    }
    pub fn redirect_len(&self) -> usize {
        self.redirect.len()
    }
    pub fn sources_cap(&self) -> usize {
        self.sources.capacity()
    }
    pub fn sources_len(&self) -> usize {
        self.sources.len()
    }

    pub fn get_allow(&self) -> &HashSet<H> {
        &self.allow
    }
    pub fn insert_allow(&mut self, value: &'a str) {
        self.redirect.retain(|r| r.from != value);
        if let Ok(v) = H::try_from(value) {
            self.block.remove(&v);
            self.allow.insert(v);
        };
    }
    pub fn remove_allow(&mut self, value: &'a str) {
        self.allow.remove(&H::new(value));
    }

    pub fn get_block(&self) -> &HashSet<H> {
        &self.block
    }
    pub fn insert_block(&mut self, value: H<'a>) {
        self.redirect.retain(|r| r.from == value.as_str());
        self.allow.remove(&value);
        self.block.insert(value);
    }
    pub fn remove_block(&mut self, value: &'a str) {
        self.block.remove(&H::new(value));
    }

    pub fn get_redirect(&self) -> &HashSet<R> {
        &self.redirect
    }
    pub fn find_redirect(&self, value: &str) -> Option<&R> {
        self.redirect.iter().find(|r| r.from == value)
    }
    pub fn insert_redirect(&mut self, to: &'a str, from: &'a str) {
        if let Ok(v) = H::try_from(from) {
            self.allow.remove(&v);
            self.block.remove(&v);
        };
        self.redirect.insert(R::new(to, from));
    }
    pub fn remove_redirect(&mut self, value: &str) {
        self.redirect.retain(|r| r.from == value);
    }

    pub fn get_sources(&self) -> &HashSet<H> {
        &self.sources
    }
    pub fn insert_sources(&mut self, value: &'a str) {
        self.sources.insert(H::new(value));
    }
    pub fn remove_sources(&mut self, value: &'a str) {
        self.sources.remove(&H::new(value));
    }

    pub fn save(&self, paths: &StoragePath) -> Result<(), Box<dyn std::error::Error>> {
        write_list(paths.get_allow(), &self.allow)?;
        write_list(paths.get_block(), &self.block)?;
        write_list(paths.get_redirect(), &self.redirect)?;
        write_list(paths.get_sources(), &self.sources)?;
        Ok(())
    }
}
