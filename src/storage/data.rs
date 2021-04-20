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

use crate::storage::attachment::Attachment;

/// Our data content.
#[derive(Debug)]
pub struct Data {
    pub id: String,
    pub account: String,
    pub title: String,
    pub text: String,
    pub attachments: Vec<Attachment>,
    pub source: String,
}

/// Convert the incoming Status from Elefren to ours.
impl From<&Status> for Data {
    fn from(origin: &Status) -> Self {
        Self {
            id: origin.id.to_string(),
            account: origin.account.acct.to_string(),
            title: origin.spoiler_text.to_string(),
            text: origin.content.to_string(),
            attachments: origin
                .media_attachments
                .iter()
                .map(|attachment| Attachment::from(attachment))
                .collect(),
            source: origin.url.as_ref().unwrap_or(&String::new()).to_string(),
        }
    }
}
