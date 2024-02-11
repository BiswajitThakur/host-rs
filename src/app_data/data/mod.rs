use serde::{Deserialize, Serialize};
use std::{collections::{HashMap, HashSet}, error::Error, fs::File, io::Write, path::PathBuf};

use super::my_csv;

pub mod get;
pub mod myerr;

#[derive(Serialize, Deserialize, Debug)]
pub struct Url {
    pub is_enable: bool,
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Host {
    pub is_enable: bool,
    pub urls: Vec<Url>,
    pub path: PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ops {
    pub ads: Host,
    pub porn: Host,
    pub fakenews: Host,
    pub social: Host,
    pub gambling: Host,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub hosts: Ops,
    pub user_hosts: Vec<Host>,
    pub user_host_files: Vec<PathBuf>,
    pub block_path: PathBuf,
    pub allow_path: PathBuf,
    pub redirect_path: PathBuf,
    pub db_path: PathBuf,
    pub host_path: PathBuf,
}

impl Data {
    pub fn get_block_list(&self) -> Result<HashSet<String>, Box<dyn Error>> {
        Ok(my_csv::read_list(&self.block_path)?)
    }
    pub fn set_block_list(&self, v: &HashSet<String>) -> Result<(), Box<dyn Error>> {
        Ok(my_csv::write_list(&self.block_path, v)?)
    }
}

impl Data {
    pub fn get_allow_list(&self) -> Result<HashSet<String>, Box<dyn Error>> {
        Ok(my_csv::read_list(&self.allow_path)?)
    }
    pub fn set_allow_list(&self, v: &HashSet<String>) -> Result<(), Box<dyn Error>> {
        Ok(my_csv::write_list(&self.allow_path, v)?)
    }
}

impl Data {
    pub fn set_ads_list(&self, v: &HashSet<String>) -> Result<(), Box<dyn Error>> {
        Ok(my_csv::write_list(&self.hosts.ads.path, v)?)
    }
    pub fn get_ads_list(&self) -> Result<HashSet<String>, Box<dyn Error>> {
        Ok(my_csv::read_list(&self.hosts.ads.path)?)
    }
}

impl Data {
    pub fn set_redirect_list(&self, vals: &HashMap<String, String>) -> Result<(), Box<dyn Error>> {
        Ok(my_csv::write_redirect(&self.redirect_path, vals)?)
    }
    pub fn get_redirect_list(&self) -> Result<HashMap<String, String>, Box<dyn Error>> {
        Ok(my_csv::read_redirect(&self.redirect_path)?)
    }
}

impl Data {
    pub fn set_porn_list(&self, v: &HashSet<String>) -> Result<(), Box<dyn Error>> {
        Ok(my_csv::write_list(&self.hosts.porn.path, v)?)
    }
    pub fn get_porn_list(&self) -> Result<HashSet<String>, Box<dyn Error>> {
        Ok(my_csv::read_list(&self.hosts.porn.path)?)
    }
}

impl Data {
    pub fn set_fakenews_list(&self, v: &HashSet<String>) -> Result<(), Box<dyn Error>> {
        Ok(my_csv::write_list(&self.hosts.fakenews.path, v)?)
    }
    pub fn get_fakenews_list(&self) -> Result<HashSet<String>, Box<dyn Error>> {
        Ok(my_csv::read_list(&self.hosts.fakenews.path)?)
    }
}

impl Data {
    pub fn set_social_list(&self, v: &HashSet<String>) -> Result<(), Box<dyn Error>> {
        Ok(my_csv::write_list(&self.hosts.social.path, v)?)
    }
    pub fn get_social_list(&self) -> Result<HashSet<String>, Box<dyn Error>> {
        Ok(my_csv::read_list(&self.hosts.social.path)?)
    }
}

impl Data {
    pub fn set_gambling_list(&self, v: &HashSet<String>) -> Result<(), Box<dyn Error>> {
        Ok(my_csv::write_list(&self.hosts.gambling.path, v)?)
    }
    pub fn get_gambling_list(&self) -> Result<HashSet<String>, Box<dyn Error>> {
        Ok(my_csv::read_list(&self.hosts.gambling.path)?)
    }
}


impl Data {
    pub fn enable_ads_blocker(&mut self) {
        self.hosts.ads.is_enable = true;
    }
    pub fn disable_ads_blocker(&mut self) {
        self.hosts.ads.is_enable = false;
    }
}

impl Data {
    pub fn enable_porn_blocker(&mut self) {
        self.hosts.porn.is_enable = true;
    }
    pub fn disable_porn_blocker(&mut self) {
        self.hosts.porn.is_enable = false;
    }
}

impl Data {
    pub fn enable_fakenews_blocker(&mut self) {
        self.hosts.fakenews.is_enable = true;
    }
    pub fn disable_fakenews_blocker(&mut self) {
        self.hosts.fakenews.is_enable = false;
    }
}

impl Data {
    pub fn enable_social_blocker(&mut self) {
        self.hosts.social.is_enable = true;
    }
    pub fn disable_social_blocker(&mut self) {
        self.hosts.social.is_enable = false;
    }
}

impl Data {
    pub fn enable_gambling_blocker(&mut self) {
        self.hosts.gambling.is_enable = true;
    }
    pub fn disable_gambling_blocker(&mut self) {
        self.hosts.gambling.is_enable = false;
    }
}

impl Data {
    pub fn flush(&self) -> Result<(), Box<dyn Error>> {
        let mut file: File = File::create(&self.db_path)?;
        let binding: String = serde_json::to_string_pretty(self)?;
        let to_write: &[u8] = binding.as_bytes();
        file.write_all(to_write)?;
        Ok(())
    }
}
