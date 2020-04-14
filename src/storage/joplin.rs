use crate::config::Config;
use crate::config::JoplinConfig;

use reqwest::Error;
use reqwest::Url;
use serde_derive::Deserialize;

/// This is the folder structured returned by Joplin. It is here so Reqwests can
/// unjson the data (there are more fields, but these are the only ones we need
/// right now).
#[allow(dead_code)]
#[derive(Deserialize)]
struct Folder {
    id: String,
    title: String,
}

/// Connection to Joplin.
pub struct JoplinConnection {
    port: u32,
    token: String,
    folder_id: String,
}

pub fn validate(config: &Config) -> Option<JoplinConnection> {
    if let Some(joplin_config) = &config.joplin {
        let folder_id = dbg!(get_folder_id(&joplin_config));

        if let Some(folder) = folder_id {
            Some(JoplinConnection {
                port: joplin_config.port,
                token: joplin_config.token.to_string(),
                folder_id: folder,
            })
        } else {
            println!("No folder named {}", joplin_config.folder);
            None
        }
    } else {
        println!("Joplin not set up");
        None
    }
}

fn build_url(config: &JoplinConfig, resource: &String) -> Url {
    let base_url = format!(
        "http://localhost:{port}/{resource}?token={token}",
        port = config.port,
        resource = resource,
        token = config.token
    );
    let url = Url::parse(&base_url);
    url.unwrap()
}

fn get_folder_id(config: &JoplinConfig) -> Option<String> {
    let request = get_folder_list(config);
    if let Ok(folders) = request {
        for folder in folders {
            if folder.title == *config.folder {
                return Some(folder.id);
            }
        }
    }
    None
}

fn get_folder_list(config: &JoplinConfig) -> Result<Vec<Folder>, Error> {
    let folders: Vec<Folder> =
        reqwest::get(&build_url(config, &String::from("folders")).into_string())?.json()?;
    Ok(folders)
}
