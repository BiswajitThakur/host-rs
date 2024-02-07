use std::collections::{HashMap, HashSet};
use std::error::Error;

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
        };
        for i in ads_list {
            block_list.insert(i);
        };
        for i in porn_list {
            block_list.insert(i);
        };
        for i in fakenews_list {
            block_list.insert(i);
        };
        for i in social_list {
            block_list.insert(i);
        };
        for i in gambling_list {
            block_list.insert(i);
        };
        for i in allow_list {
            block_list.remove(i.as_str());
        };
        crate::host_rw::write::host(&self.data.host_path, block_list)?;
        self.data.set_allow_list(allow_list)?;
        self.data.set_block_list(usr_block_list)?;
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