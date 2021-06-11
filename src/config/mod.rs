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

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use elefren::Data;
use serde_derive::Deserialize;
use serde_derive::Serialize;

pub mod config;

#[derive(Serialize, Deserialize, Debug)]
pub struct JoplinConfig {
    pub port: u32,
    pub folder: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrgConfig {
    pub location: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Favourite {
    pub last: String,
}

impl Favourite {
    pub fn new() -> Self {
        Self { last: "".into() }
    }

    pub fn new_with_value(new_last: &str) -> Self {
        Self {
            last: new_last.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    accounts: HashMap<String, AccountConfig>,
}

impl Config {
    pub fn add_account(name: &str, configuration: elefren::data::Data) {
        let config = AccountConfig::from(configuration);
        let mut accounts: HashMap<String, AccountConfig> = HashMap::new();
        accounts.insert(name.into(), config);
        let content = toml::to_string(&accounts).unwrap();
        log::debug!("{}", content);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountConfig {
    pub favourite: Favourite,
    pub mastodon: Data,
    pub joplin: Option<JoplinConfig>,
    pub org: Option<OrgConfig>,
}

/// Errors while loading the configuration file
#[derive(Debug)]
pub enum ConfigError {
    /// There is no configuration file
    NoConfig,
    /// The configuration file format is invalid
    InvalidConfig,
}

impl From<toml::de::Error> for ConfigError {
    fn from(e: toml::de::Error) -> Self {
        // Since we only have one single error so far...
        log::debug!("Toml error: {:?}", e);
        ConfigError::InvalidConfig
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(_: std::io::Error) -> Self {
        // This is not optimal: Some IO errors we can't recover (like
        // PermissionDenied)
        ConfigError::NoConfig
    }
}

impl AccountConfig {
    pub fn get() -> Result<AccountConfig, ConfigError> {
        let mut fp = File::open("downfav.toml")?;
        let mut contents = String::new();
        fp.read_to_string(&mut contents).unwrap();
        Ok(toml::from_str(&contents)?)
    }

    pub fn save(self, most_recent_favourite: &str) -> Self {
        let new_configuration = Self {
            favourite: Favourite::new_with_value(most_recent_favourite),
            ..self
        };
        let content = toml::to_string(&new_configuration).unwrap();

        if let Ok(mut fp) = File::create("downfav.toml") {
            fp.write_all(content.as_bytes()).unwrap();
        }
        new_configuration
    }
}

impl From<elefren::data::Data> for AccountConfig {
    fn from(data: elefren::data::Data) -> Self {
        Self {
            favourite: Favourite::new(),
            mastodon: data,
            joplin: None,
            org: None,
        }
    }
}
