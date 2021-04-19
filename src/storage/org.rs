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

use chrono::prelude::*;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;

use crate::config::OrgConfig;
use crate::storage::data::Data;
use crate::storage::storage::Storage;

pub struct Org {
    file: PathBuf,
    date: String,
}

impl Org {
    pub(crate) fn new_from_config(config: &OrgConfig) -> Org {
        let now = Utc::now();
        let filename = format!("{}{}{}.org", now.year(), now.month(), now.day());
        let date = format!("{}-{}-{}", now.year(), now.month(), now.day());
        let full_path = Path::new(&config.location).join(&filename);
        log::debug!("Org file: {}", full_path.to_string_lossy());

        Org {
            file: full_path,
            date,
        }
    }
}

impl Storage for Org {
    fn save(&self, record: &Data) {
        let mut fp = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&self.file)
            .unwrap_or_else(|_| {
                // Let's assume here that the problem is that the file doesn't exist.
                log::debug!(
                    "Creating {filename}",
                    filename = &self.file.to_string_lossy()
                );
                OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(&self.file)
                    .map(|mut fp| {
                        let text =
                            format!("#+title: Favourites from {date}\n\n", date = &self.date);
                        fp.write_all(text.as_bytes()).unwrap();
                        fp
                    })
                    .unwrap()
            });
        fp.write_all(
            format!(
                "* {user}/{id}\n  {message}\n",
                user = record.account,
                id = record.id,
                message = record.text,
            )
            .as_bytes(),
        )
        .unwrap();
    }
}
