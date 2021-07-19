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

use std::io::Write;

use log_derive::logfn;
use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::config::errors::ConfigError;
use crate::config::Configurable;

/// Configuration for the Org backend
#[derive(Serialize, Deserialize, Debug)]
pub struct OrgConfig {
    pub path: String,
}

impl Configurable for OrgConfig {
    #[logfn(Trace)]
    fn config() -> Result<Self, ConfigError> {
        print!("Base path for Org files: ");
        std::io::stdout().flush().expect("Failed to flush stdout!");

        let mut path = String::new();
        std::io::stdin().read_line(&mut path)?;
        let fullpath = shellexpand::full(path.trim())?;
        log::debug!("Full path: {:?}", fullpath);
        Ok(Self {
            path: fullpath.into(),
        })
    }
}
