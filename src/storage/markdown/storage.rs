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
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use log_derive::logfn;

use super::config::MarkdownConfig;
use crate::storage::data::Data;
use crate::storage::helpers::make_markdown;
use crate::storage::storage::Storage;

pub struct Markdown {
    path: String,
}

impl Storage for Markdown {
    fn save(&self, data: &Data) {
        self.create_dirs(data);
        self.save_content(data);
        self.save_attachments(data);
        println!("Saved to {}", self.dir(data).to_string_lossy());
    }
}

impl Markdown {
    pub fn new(config: &MarkdownConfig) -> Self {
        Self {
            path: config.path.to_string(),
        }
    }

    /// The directory in which the data from this toot will be saved.
    #[logfn(Trace)]
    fn dir(&self, data: &Data) -> PathBuf {
        Path::new(&self.path).join(&data.account).join(&data.id)
    }

    /// Make sure the path structure exists for saving the data.
    fn create_dirs(&self, data: &Data) {
        std::fs::create_dir_all(self.dir(data))
            .expect("Failed to create storage directory");
    }

    /// Save the content in the directory.
    fn save_content(&self, data: &Data) {
        let filename = self.dir(data).join("toot.md");
        let mut fp = File::create(filename).expect("Failed to create file");
        fp.write_all(make_markdown(data).as_bytes())
            .expect("Failed to save content");
    }

    /// Save the attachments.
    fn save_attachments(&self, data: &Data) {
        data.attachments.iter().for_each(|attachment| {
            let filename = self.dir(data).join(attachment.filename());
            let mut fp = File::create(filename).expect("Failed to create file");
            attachment.download().copy_to(&mut fp).unwrap();
        })
    }
}
