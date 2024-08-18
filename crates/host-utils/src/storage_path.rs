use std::path::PathBuf;

#[allow(unused)]
pub struct StoragePath {
    allow: PathBuf,
    block: PathBuf,
    redirect: PathBuf,
    sources: PathBuf,
}

impl From<PathBuf> for StoragePath {
    fn from(parent: PathBuf) -> Self {
        let pkg_name = env!("CARGO_PKG_NAME");
        Self {
            allow: [parent.clone(), pkg_name.into(), "allow".into()]
                .into_iter()
                .collect(),
            block: [parent.clone(), pkg_name.into(), "block".into()]
                .into_iter()
                .collect(),
            redirect: [parent.clone(), pkg_name.into(), "redirect".into()]
                .into_iter()
                .collect(),
            sources: [parent, pkg_name.into(), "soucres".into()]
                .into_iter()
                .collect(),
        }
    }
}

impl StoragePath {
    pub fn get_allow(&self) -> &PathBuf {
        &self.allow
    }
    pub fn get_block(&self) -> &PathBuf {
        &self.block
    }
    pub fn get_redirect(&self) -> &PathBuf {
        &self.redirect
    }
    pub fn get_sources(&self) -> &PathBuf {
        &self.sources
    }
}

#[allow(unused)]
fn get_parent() -> PathBuf {
    #[cfg(not(debug_assertions))]
    return dirs::data_dir().unwrap();
    #[cfg(debug_assertions)]
    return PathBuf::from("tests");
}
