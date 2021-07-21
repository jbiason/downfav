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
use std::time::Duration;

use reqwest::Response;

#[derive(Debug)]
pub struct Attachment {
    url: String,
}

impl From<&elefren::entities::attachment::Attachment> for Attachment {
    fn from(origin: &elefren::entities::attachment::Attachment) -> Self {
        // XXX basename of the origin.url here
        println!("Found attachment: {}", origin.url);
        Self {
            url: origin.url.to_string(),
        }
    }
}

impl Attachment {
    pub fn filename(&self) -> String {
        let mut frags = self.url.rsplitn(2, '/');

        if let Some(path_part) = frags.next() {
            path_part.split('?').next().unwrap_or(&self.url).to_string()
        } else {
            // this is, most of the time, bad (due special characters -- like '?' -- and path)
            self.url.to_string()
        }
    }

    // XXX unwrap
    pub fn download(&self) -> Response {
        reqwest::Client::builder()
            .timeout(Duration::from_secs(600))
            .build()
            .unwrap()
            .get(&self.url)
            .send()
            .unwrap()
    }
}
