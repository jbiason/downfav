use std::fs::File;
use std::io::prelude::*;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use toml;

#[derive(Serialize, Deserialize, Debug)]
pub struct JoplinConfig {
    port: u32,
    folder: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub last_favorite: String,
    pub joplin: Option<JoplinConfig>,
}

impl Config {
    pub fn get() -> Config {
        if let Ok(mut fp) = File::open("downfav.toml") {
            let mut contents = String::new();
            fp.read_to_string(&mut contents).unwrap();

            let config: Config = toml::from_str(&contents).unwrap_or(Config {
                last_favorite: "".to_string(),
                joplin: None,
            });
            config
        } else {
            Config {
                last_favorite: "".to_string(),
                joplin: None,
            }
        }
    }


    pub fn save(&self, most_recent_favourite: Option<String>) -> () {
        if let Some(id) = most_recent_favourite {
            let new_configuration = Config {
                last_favorite: id,
                joplin: match &self.joplin {
                    None => None,
                    Some(x) => Some(JoplinConfig {
                        folder: x.folder.to_string(),
                        port: x.port,
                    }),
                },
            };
            let content = toml::to_string(&new_configuration).unwrap();

            if let Ok(mut fp) = File::create("downfav.toml") {
                fp.write_all(content.as_bytes()).unwrap();
            }
        }
    }
}
