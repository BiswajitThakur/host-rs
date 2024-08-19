use std::fmt::Display;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::{error::Error, path::Path};

pub fn write_list<T, I, U>(path: T, value: I) -> Result<(), Box<dyn Error>>
where
    T: AsRef<Path>,
    I: IntoIterator<Item = U>,
    U: Display,
{
    let mut f = File::create(path)?;
    for line in value {
        writeln!(&mut f, "{}", line)?;
    }
    Ok(())
}

#[derive(Debug)]
pub struct StoragePath {
    allow: PathBuf,
    block: PathBuf,
    redirect: PathBuf,
    sources: PathBuf,
}

impl From<PathBuf> for StoragePath {
    #[allow(unused)]
    fn from(parent: PathBuf) -> Self {
        #[cfg(debug_assertions)]
        let parent = PathBuf::from("tests");
        Self {
            allow: [parent.clone(), "allow".into()].into_iter().collect(),
            block: [parent.clone(), "block".into()].into_iter().collect(),
            redirect: [parent.clone(), "redirect".into()].into_iter().collect(),
            sources: [parent, "soucres".into()].into_iter().collect(),
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
/*
#[allow(unused)]
fn get_parent() -> PathBuf {
    #[cfg(not(debug_assertions))]
    return dirs::data_dir().unwrap();
    #[cfg(debug_assertions)]
    return PathBuf::from("tests");
}*/
