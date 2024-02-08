use std::{collections::{HashMap, HashSet}, error::Error, io};

pub mod filter;
pub mod get;
pub mod read;
pub mod write;

#[allow(dead_code)]
#[derive(Clone)]
pub struct DataPath<'a> {
    pub allow_path: &'a str,
    pub block_path: &'a str,
    // pub redirect_path: &'a str,
    pub host_path: &'a str,
}

#[allow(dead_code)]
pub struct AppData<'a> {
    pub allow: HashSet<String>,
    pub block: HashSet<String>,
    pub redirect: HashMap<String, String>,
    pub host: HashSet<String>,
    pub path: DataPath<'a>,
}

#[allow(dead_code)]
impl AppData<'_> {
    pub fn init(path: DataPath) -> Result<AppData, Box<dyn Error>> {
        let mut allow: HashSet<String> = read::list(path.allow_path)?;
        let mut block: HashSet<String> = read::list(path.block_path)?;
        let mut host: HashSet<String> = read::etc_host(path.host_path)?;
        let to_remove: Vec<String> = allow.intersection(&block).cloned().collect();
        for i in &to_remove {
            allow.remove(i);
            block.remove(i);
        }
        for i in &block {
            host.insert(i.clone());
        }
        for i in &allow {
            host.remove(i);
        }
        Ok(AppData {
            allow: allow,
            block: block,
            host: host,
            path: path.clone(),
        })
    }
    pub fn get_allow_list<'a>(&'a self) -> &'a HashSet<String> {
        &self.allow
    }
    pub fn get_block_list<'a>(&'a self) -> &'a HashSet<String> {
        &self.block
    }
    pub fn add_allow_list(&mut self, data: HashSet<String>) {
        for i in data {
            self.block.remove(&i);
            self.host.remove(&i);
            self.allow.insert(i);
        }
    }
    pub fn add_block_list(&mut self, data: HashSet<String>) {
        for i in data {
            self.allow.remove(&i);
            self.block.insert(i.to_owned());
            self.host.insert(i);
        }
    }
    pub fn block(&mut self, data: HashSet<String>) {
        let mut data: HashSet<String> = data;
        for i in &self.allow {
            data.remove(i);
        }
        for i in data {
            self.host.insert(i);
        }
    }
    pub fn remove_from_all(&mut self, v: Vec<String>) {
        for i in &v {
            self.allow.remove(i);
            self.block.remove(i);
            self.host.remove(i);
        }
    }

    pub async fn download(v: Vec<&str>) -> Result<HashSet<String>, reqwest::Error> {
        let mut res_body: HashSet<String> = HashSet::new();
        for url in v {
            res_body.extend(filter::host(get::get(url).await?));
        }
        Ok(res_body)
    }
    pub async fn block_by_url(&mut self, urls: Vec<&str>) -> Result<(), Box<dyn Error>> {
        let h: HashSet<String> = AppData::download(urls).await?;
        self.host.clear();
        self.host = self.block.clone();
        self.block(h);
        Ok(())
    }
    pub fn block_by_file(&mut self, paths: Vec<&str>) -> Result<(), io::Error> {
        for path in paths {
            let h: HashSet<String> = read::host(path)?;
            self.block(h);
        }
        Ok(())
    }
    pub fn write(&self) -> Result<(), Box<dyn Error>> {
        write::list(self.path.allow_path, &self.allow)?;
        write::list(self.path.block_path, &self.block)?;
        write::host(self.path.host_path, &self.host)?;
        Ok(())
    }
}
