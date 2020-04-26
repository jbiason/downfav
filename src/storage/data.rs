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

use std::convert::From;

use elefren::entities::status::Status;
use html2md;

use crate::storage::attachment::Attachment;
use crate::storage::storage::Storage;

/// Our data content.
#[derive(Debug)]
pub struct Data {
    pub id: String,
    pub account: String,
    pub text: String,
    pub attachments: Vec<Attachment>,
    pub source: String,
}

/// Convert the incoming Status from Elefren to ours.
impl From<&Status> for Data {
    fn from(origin: &Status) -> Self {
        println!("Downloading ID: {}", origin.id);

        Self {
            id: origin.id.to_string(),
            account: origin.account.acct.to_string(),
            text: dbg!(build_text(origin)),
            attachments: origin
                .media_attachments
                .iter()
                .map(|attachment| Attachment::from(attachment))
                .collect(),
            source: origin.url.as_ref().unwrap_or(&String::new()).to_string(),
        }
    }
}

impl Data {
    pub fn save<T: Storage>(&self, storage: &T) {
        storage.save(self);
    }
}

fn build_text(status: &Status) -> String {
    let base_content = html2md::parse_html(&status.content);
    let source = &status.url;
    let title = &status.spoiler_text;

    let mut result = String::new();
    if title.len() > 0 {
        result.push_str(title);
        result.push_str("\n\n");
    }

    result.push_str(&base_content);

    if let Some(url) = source {
        result.push_str("\n\n");
        result.push_str(&url);
    }

    result
}
