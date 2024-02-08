pub mod filter;
pub mod read;
pub mod write;
pub mod get;
use std::collections::{HashMap, HashSet};
use std::error::Error;

use super::app_data::{my_csv, data::{self, Data}};
//use serde::de::Error;

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

pub struct App {
    pub host: Host,
    pub data: Data,
}

impl App {
    pub fn init() -> Result<App, Box<dyn Error>> {
        let my_data = data::get::db().unwrap();
        let allow_list: HashSet<String> = my_csv::read_list(&my_data.allow_path)?.into_iter().collect();
        let block_list: HashSet<String> = my_csv::read_list(&my_data.block_path)?.into_iter().collect();
        let redirect_list: HashMap<String, String> = my_csv::read_redirect(&my_data.redirect_path)?;
        let ads_list: HashSet<String> = my_csv::read_list(&my_data.hosts.ads.path)?.into_iter().collect();
        let porn_list: HashSet<String> = my_csv::read_list(&my_data.hosts.porn.path)?.into_iter().collect();
        let fakenews_list: HashSet<String> = my_csv::read_list(&my_data.hosts.fakenews.path)?.into_iter().collect();
        let social_list: HashSet<String> = my_csv::read_list(&my_data.hosts.social.path)?.into_iter().collect();
        let gambling_list: HashSet<String> = my_csv::read_list(&my_data.hosts.gambling.path)?.into_iter().collect();
        Ok(
            App {
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
            }
        )
    }
}
