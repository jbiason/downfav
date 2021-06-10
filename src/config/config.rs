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

use directories::ProjectDirs;
use elefren::Data;

/// The last seen favourite
struct Favourite {
    last: String,
}

/// Account configuration
struct AccountConfig {
    favourite: Favourite,
    mastodon: Data,
}

/// Errors from the configuration
pub enum ConfigError {
    /// The system can't figure out the path for the configuration file
    CantFigureConfigPath,
}

/// The main configuration
#[derive(serde_derive::Serialize, serde::derive::Deserialize, Debug)]
pub struct Config(HashMap<String, AccountConfig>);

impl Config {
    /// Figure out the filename for the configuration file.
    fn filename() -> Result<Path, ConfigError> {
        if let Some(proj_dirs) =
            ProjectDirs::from("me", "JulioBiason", "downfav")
        {
            Ok(proj_dirs.config_dir())
        } else {
            Error(ConfigError::CantFigureConfigPath)
        }
    }

    pub fn new() -> Self {}
}
