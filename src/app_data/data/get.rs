use crate::app_data::paths::paths;
use std::io::Write;
use std::{
    collections::HashMap,
    error::Error,
    fs::{self, create_dir_all, File},
    io::BufReader,
    path::PathBuf,
};

use super::{Data, Host, Ops, Url};

#[allow(dead_code)]
pub fn db() -> Result<Data, Box<dyn Error>> {
    #[cfg(not(debug_assertions))]
    if !crate::app_data::usr::is_admin() {
        return Err(Box::new(super::myerr::MyError(
            "Administrator privilages required".into(),
        )));
    };
    let paths: HashMap<String, PathBuf> = paths();
    let db_path: &PathBuf = paths.get("db").unwrap();
    if db_path.exists() {
        return Ok(read_data(db_path)?);
    };
    let allow_path: &PathBuf = paths.get("allow").unwrap();
    let block_path: &PathBuf = paths.get("block").unwrap();
    let redirect_path: &PathBuf = paths.get("redirect").unwrap();
    let ads_path: &PathBuf = paths.get("ads").unwrap();
    let porn_path: &PathBuf = paths.get("porn").unwrap();
    let fakenews_path: &PathBuf = paths.get("fakenews").unwrap();
    let social_path: &PathBuf = paths.get("social").unwrap();
    let gambling_path: &PathBuf = paths.get("gambling").unwrap();
    let data_dir: &PathBuf = paths.get("data_dir").unwrap();
    if !data_dir.exists() {
        create_dir_all(data_dir)?;
    };
    setup_path(db_path)?;
    setup_path(allow_path)?;
    setup_path(block_path)?;
    setup_path(redirect_path)?;
    setup_path(ads_path)?;
    setup_path(porn_path)?;
    setup_path(fakenews_path)?;
    setup_path(social_path)?;
    setup_path(gambling_path)?;
    let data = Data {
        hosts: Ops {
            ads: Host {
                is_enable: false,
                urls: vec![
                    Url {
                        is_enable: true,
                        url: "https://adaway.org/hosts.txt".into(),
                    },
                    Url {
                        is_enable: true,
                        url: "https://pgl.yoyo.org/adservers/serverlist.php?hostformat=hosts&showintro=0&mimetype=plaintext".into(),
                    },
                    Url {
                        is_enable: true,
                        url: "https://raw.githubusercontent.com/StevenBlack/hosts/master/hosts".into(),
                    },
                ],
                path: fs::canonicalize(ads_path).unwrap(),
            },
            porn: Host {
                is_enable: false,
                urls: vec![Url {
                    is_enable: true,
                    url: "http://sbc.io/hosts/alternates/porn-only/hosts".into(),
                }],
                path: fs::canonicalize(porn_path).unwrap(),
            },
            fakenews: Host {
                is_enable: false,
                urls: vec![Url {
                    is_enable: true,
                    url: "http://sbc.io/hosts/alternates/fakenews-only/hosts".into(),
                }],
                path: fs::canonicalize(fakenews_path).unwrap(),
            },
            social: Host {
                is_enable: false,
                urls: vec![Url {
                    is_enable: true,
                    url: "http://sbc.io/hosts/alternates/social-only/hosts".into(),
                }],
                path: fs::canonicalize(social_path).unwrap(),
            },
            gambling: Host {
                is_enable: false,
                urls: vec![Url {
                    is_enable: true,
                    url: "http://sbc.io/hosts/alternates/gambling-only/hosts".into(),
                }],
                path: fs::canonicalize(gambling_path).unwrap(),
            },
        },
        user_hosts: vec![],
        user_host_files: vec![],
        block_path: fs::canonicalize(block_path).unwrap(),
        allow_path: fs::canonicalize(allow_path).unwrap(),
        redirect_path: fs::canonicalize(redirect_path).unwrap(),
        db_path: fs::canonicalize(db_path).unwrap(),
        host_path: fs::canonicalize(paths.get("host_path").unwrap()).unwrap(),
    };
    let mut file: File = File::create(db_path)?;
    let binding: String = serde_json::to_string_pretty(&data)?;
    let to_write: &[u8] = binding.as_bytes();
    file.write_all(to_write)?;
    Ok(data)
}

#[allow(dead_code)]
fn read_data(path: &PathBuf) -> Result<Data, Box<dyn Error>> {
    let file: File = File::open(path)?;
    let reader: BufReader<File> = BufReader::new(file);
    let f: Data = serde_json::from_reader(reader)?;
    Ok(f)
}

#[allow(dead_code)]
fn setup_path(path: &PathBuf) -> Result<(), Box<dyn Error>> {
    if path.exists() {
        return Ok(());
    };
    match File::create(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}


#[cfg(test)]
mod tests {
    use super::setup_path;
    use std::path::PathBuf;

    #[test]
    fn test_setup_path() {
        let p: PathBuf = ["test_files", "tmp_list.csv"].iter().collect();
        let k = setup_path(&p);
        assert!(k.is_ok());
    }
}