use std::collections::{HashMap, HashSet};
use std::error::Error;

use crate::host_rw::{filter, get};

use super::app_data::{
    data::{self, Data},
    my_csv,
};

#[derive(Debug)]
pub struct Host {
    pub allow_list: HashSet<String>,
    pub block_list: HashSet<String>,
    pub redirect_list: HashMap<String, String>,
    pub ads: HashSet<String>,
    pub porn: HashSet<String>,
    pub fakenews: HashSet<String>,
    pub social: HashSet<String>,
    pub gambling: HashSet<String>,
}

#[derive(Debug)]
pub struct App {
    pub host: Host,
    pub data: Data,
}

impl App {
    pub fn init() -> Result<App, Box<dyn Error>> {
        let my_data = data::get::db().unwrap();
        let allow_list: HashSet<String> = my_csv::read_list(&my_data.allow_path)?
            .into_iter()
            .collect();
        let block_list: HashSet<String> = my_csv::read_list(&my_data.block_path)?
            .into_iter()
            .collect();
        let redirect_list: HashMap<String, String> = my_csv::read_redirect(&my_data.redirect_path)?;
        let ads_list: HashSet<String> = my_csv::read_list(&my_data.hosts.ads.path)?
            .into_iter()
            .collect();
        let porn_list: HashSet<String> = my_csv::read_list(&my_data.hosts.porn.path)?
            .into_iter()
            .collect();
        let fakenews_list: HashSet<String> = my_csv::read_list(&my_data.hosts.fakenews.path)?
            .into_iter()
            .collect();
        let social_list: HashSet<String> = my_csv::read_list(&my_data.hosts.social.path)?
            .into_iter()
            .collect();
        let gambling_list: HashSet<String> = my_csv::read_list(&my_data.hosts.gambling.path)?
            .into_iter()
            .collect();
        Ok(App {
            host: Host {
                allow_list: allow_list,
                block_list: block_list,
                redirect_list: redirect_list,
                ads: ads_list,
                porn: porn_list,
                fakenews: fakenews_list,
                social: social_list,
                gambling: gambling_list,
            },
            data: my_data,
        })
    }
}

impl App {
    pub fn flush(&self) -> Result<(), Box<dyn Error>> {
        let allow_list = &self.host.allow_list;
        let usr_block_list = &self.host.block_list;
        let redirect_list = &self.host.redirect_list;
        let ads_list = &self.host.ads;
        let porn_list = &self.host.porn;
        let fakenews_list = &self.host.fakenews;
        let social_list = &self.host.social;
        let gambling_list = &self.host.gambling;
        let mut block_list: HashSet<&str> = HashSet::with_capacity(
            usr_block_list.len()
                + ads_list.len()
                + porn_list.len()
                + fakenews_list.len()
                + social_list.len()
                + gambling_list.len(),
        );
        for i in usr_block_list {
            block_list.insert(i);
        }
        for i in ads_list {
            block_list.insert(i);
        }
        for i in porn_list {
            block_list.insert(i);
        }
        for i in fakenews_list {
            block_list.insert(i);
        }
        for i in social_list {
            block_list.insert(i);
        }
        for i in gambling_list {
            block_list.insert(i);
        }
        for i in allow_list {
            block_list.remove(i.as_str());
        }
        for (key, _) in redirect_list {
            block_list.remove(key.as_str());
        }
        crate::host_rw::write::host(&self.data.host_path, block_list)?;
        crate::host_rw::write::redirect(&self.data.host_path, redirect_list)?;
        self.data.set_allow_list(allow_list)?;
        self.data.set_block_list(usr_block_list)?;
        self.data.set_redirect_list(redirect_list)?;
        self.data.set_ads_list(ads_list)?;
        self.data.set_porn_list(porn_list)?;
        self.data.set_fakenews_list(fakenews_list)?;
        self.data.set_social_list(social_list)?;
        self.data.set_gambling_list(gambling_list)?;
        self.data.flush()?;
        Ok(())
    }
}

impl App {
    pub fn add_to_allow_list(&mut self, val: Vec<String>) {
        for i in &val {
            self.host.block_list.remove(i);
            self.host.ads.remove(i);
            self.host.fakenews.remove(i);
            self.host.porn.remove(i);
            self.host.gambling.remove(i);
            self.host.redirect_list.remove(i);
        }
        self.host.allow_list.extend(val);
    }
}

impl App {
    pub fn add_to_block_list(&mut self, val: Vec<String>) {
        for i in &val {
            self.host.allow_list.remove(i);
            self.host.ads.remove(i);
            self.host.fakenews.remove(i);
            self.host.porn.remove(i);
            self.host.gambling.remove(i);
            self.host.redirect_list.remove(i);
        }
        self.host.block_list.extend(val);
    }
}

impl App {
    pub fn add_redirect_list(&mut self, vals: HashMap<String, String>) {
        for (key, _) in &vals {
            self.host.allow_list.remove(key);
            self.host.block_list.remove(key);
            self.host.ads.remove(key);
            self.host.fakenews.remove(key);
            self.host.porn.remove(key);
            self.host.gambling.remove(key);
        }
        self.host.redirect_list.extend(vals);
    }

    pub fn rm_redirect(&mut self, keys: Vec<String>) {
        for i in keys {
            self.host.redirect_list.remove(&i);
        }
    }
}

impl App {
    pub fn clear_all(&mut self) {
        self.host.allow_list.clear();
        self.host.block_list.clear();
        self.host.redirect_list.clear();
        self.host.ads.clear();
        self.host.porn.clear();
        self.host.fakenews.clear();
        self.host.gambling.clear();
        self.host.porn.clear();
    }
}

impl App {
    pub async fn block_ads(&mut self) -> Result<(), reqwest::Error> {
        let mut downloads = Vec::<HashSet<String>>::with_capacity(self.data.hosts.ads.urls.len());
        let mut capacity: usize = 0;
        for url in self.data.hosts.ads.urls.iter() {
            if !url.is_enable {
                continue;
            };
            let tmp: String = get::get(&url.url).await?;
            let f: HashSet<String> = filter::host(tmp);
            capacity += f.len();
            downloads.push(f);
        }
        let mut result = HashSet::<String>::with_capacity(capacity);
        for i in downloads {
            result.extend(i);
        }
        result.remove("127.0.0.1");
        result.remove("0.0.0.0");
        if &result == &self.host.ads {
            return Ok(());
        };
        self.host.ads.clear();
        self.host.ads.extend(result);
        Ok(())
    }

    pub fn unblock_ads(&mut self) {
        self.data.hosts.ads.is_enable = false;
        self.host.ads.clear();
    }
}


impl App {
    pub async fn block_porn(&mut self) -> Result<(), reqwest::Error> {
        let mut downloads = Vec::<HashSet<String>>::with_capacity(self.data.hosts.porn.urls.len());
        let mut capacity: usize = 0;
        for url in self.data.hosts.porn.urls.iter() {
            if !url.is_enable {
                continue;
            };
            let tmp: String = get::get(&url.url).await?;
            let f: HashSet<String> = filter::host(tmp);
            capacity += f.len();
            downloads.push(f);
        }
        let mut result = HashSet::<String>::with_capacity(capacity);
        for i in downloads {
            result.extend(i);
        }
        result.remove("127.0.0.1");
        result.remove("0.0.0.0");
        if &result == &self.host.porn {
            return Ok(());
        };
        self.host.porn.clear();
        self.host.porn.extend(result);
        Ok(())
    }

    pub fn unblock_porn(&mut self) {
        self.data.hosts.porn.is_enable = false;
        self.host.porn.clear();
    }
}

impl App {
    pub async fn block_fakenews(&mut self) -> Result<(), reqwest::Error> {
        let mut downloads = Vec::<HashSet<String>>::with_capacity(self.data.hosts.fakenews.urls.len());
        let mut capacity: usize = 0;
        for url in self.data.hosts.fakenews.urls.iter() {
            if !url.is_enable {
                continue;
            };
            let tmp: String = get::get(&url.url).await?;
            let f: HashSet<String> = filter::host(tmp);
            capacity += f.len();
            downloads.push(f);
        }
        let mut result = HashSet::<String>::with_capacity(capacity);
        for i in downloads {
            result.extend(i);
        }
        result.remove("127.0.0.1");
        result.remove("0.0.0.0");
        if &result == &self.host.fakenews {
            return Ok(());
        };
        self.host.fakenews.clear();
        self.host.fakenews.extend(result);
        Ok(())
    }

    pub fn unblock_fakenews(&mut self) {
        self.data.hosts.fakenews.is_enable = false;
        self.host.fakenews.clear();
    }
}

impl App {
    pub async fn block_social(&mut self) -> Result<(), reqwest::Error> {
        let mut downloads = Vec::<HashSet<String>>::with_capacity(self.data.hosts.social.urls.len());
        let mut capacity: usize = 0;
        for url in self.data.hosts.social.urls.iter() {
            if !url.is_enable {
                continue;
            };
            let tmp: String = get::get(&url.url).await?;
            let f: HashSet<String> = filter::host(tmp);
            capacity += f.len();
            downloads.push(f);
        }
        let mut result = HashSet::<String>::with_capacity(capacity);
        for i in downloads {
            result.extend(i);
        }
        result.remove("127.0.0.1");
        result.remove("0.0.0.0");
        if &result == &self.host.social {
            return Ok(());
        };
        self.host.social.clear();
        self.host.social.extend(result);
        Ok(())
    }

    pub fn unblock_social(&mut self) {
        self.data.hosts.social.is_enable = false;
        self.host.social.clear();
    }
}

impl App {
    pub async fn block_gambling(&mut self) -> Result<(), reqwest::Error> {
        let mut downloads = Vec::<HashSet<String>>::with_capacity(self.data.hosts.gambling.urls.len());
        let mut capacity: usize = 0;
        for url in self.data.hosts.gambling.urls.iter() {
            if !url.is_enable {
                continue;
            };
            let tmp: String = get::get(&url.url).await?;
            let f: HashSet<String> = filter::host(tmp);
            capacity += f.len();
            downloads.push(f);
        }
        let mut result = HashSet::<String>::with_capacity(capacity);
        for i in downloads {
            result.extend(i);
        }
        result.remove("127.0.0.1");
        result.remove("0.0.0.0");
        if &result == &self.host.gambling {
            return Ok(());
        };
        self.host.gambling.clear();
        self.host.gambling.extend(result);
        Ok(())
    }

    pub fn unblock_gambling(&mut self) {
        self.data.hosts.gambling.is_enable = false;
        self.host.gambling.clear();
    }
}
