/*
   DOWNFAV - Download Favourites
   Copyright (C) 2020-2021  Julio Biason

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
use std::path::PathBuf;

use directories::ProjectDirs;
use elefren::Data;
use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::config::errors::ConfigError;
use crate::filesystem::config::FilesystemConfig;

/// The last seen favourite
#[derive(Serialize, Deserialize, Debug)]
struct Favourite {
    last: String,
}

/// Account configuration
#[derive(Serialize, Deserialize, Debug)]
struct AccountConfig {
    favourite: Option<Favourite>,
    mastodon: Data,
    filesystem: Option<FilesystemConfig>,
    // joplin: Option<JoplinConfig>,
    // org: Option<OrgConfig>,
}

/// The main configuration
#[derive(Serialize, Deserialize, Debug)]
pub struct Config(HashMap<String, AccountConfig>);

impl Config {
    /// Figure out the filename for the configuration file.
    fn filename() -> Result<PathBuf, ConfigError> {
        match ProjectDirs::from("me", "JulioBiason", "downfav.toml") {
            Some(proj_dirs) => Ok(proj_dirs.config_dir().into()),
            None => Err(ConfigError::CantFigureConfigPath),
        }
    }

    /// Open the configuration file; if it doesn't exist, returns an empty set.
    pub fn open() -> Result<Self, ConfigError> {
        let filename = Config::filename()?;
        log::debug!("Trying to open file \"{:?}\"", filename);
        match File::open(filename) {
            Ok(mut fp) => {
                let mut contents = String::new();
                fp.read_to_string(&mut contents)?;
                let parsed = toml::from_str(&contents)?;
                Ok(Self(parsed))
            }
            Err(_) => Ok(Self(HashMap::new())),
        }
    }

    /// Add a new account to the configuration file
    pub fn add_account(&mut self, name: &str, configuration: Data) {
        let account_data = AccountConfig {
            favourite: None,
            mastodon: configuration,
            filesystem: None,
        };
        self.0.insert(name.into(), account_data);
    }

    /// Remove account
    pub fn remove_account(&mut self, name: &str) {
        self.0.remove(name);
    }

    /// Save the current configuration file.
    pub fn save(&self) -> Result<(), ConfigError> {
        let content = toml::to_string(&self.0)?;
        let filename = Config::filename()?;
        log::debug!("Saving configuration to file \"{:?}\"", filename);
        let mut fp = File::create(filename)?;
        fp.write_all(content.as_bytes())?;
        Ok(())
    }
}
