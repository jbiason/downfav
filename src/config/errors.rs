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

/// Errors from the configuration
#[derive(Debug)]
pub enum ConfigError {
    /// The system can't figure out the path for the configuration file
    CantFigureConfigPath,
    /// The configuration file has an error and can't be parsed
    ConfigFileIsBroken,
    /// There was something broken with the data and we couldn't save it properly
    InvalidConfiguration,
    /// The select path is invalid
    InvalidPath,
}

impl From<toml::de::Error> for ConfigError {
    fn from(e: toml::de::Error) -> Self {
        log::debug!("Toml error: {:?}", e);
        ConfigError::ConfigFileIsBroken
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(e: std::io::Error) -> Self {
        log::debug!("I/O error: {:?}", e);
        ConfigError::ConfigFileIsBroken
    }
}

impl From<toml::ser::Error> for ConfigError {
    fn from(e: toml::ser::Error) -> Self {
        log::debug!("TOML error: {:?}", e);
        ConfigError::InvalidConfiguration
    }
}

impl From<shellexpand::LookupError<std::env::VarError>> for ConfigError {
    fn from(e: shellexpand::LookupError<std::env::VarError>) -> Self {
        log::debug!("Shellexpand error: {:?}", e);
        ConfigError::InvalidPath
    }
}
