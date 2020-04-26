/*
   DOWNFAV - Download Favourites
   Copyright (C) 2020  Julio Biason

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU Affero General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU Affero General Public License for more details.

   You should have received a copy of the GNU Affero General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use std::fs::File;
use std::io::prelude::*;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use toml;

#[derive(Serialize, Deserialize, Debug)]
pub struct JoplinConfig {
    pub port: u32,
    pub folder: String,
    pub token: String,
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
                        token: x.token.to_string(),
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
