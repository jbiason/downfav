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

use elefren::Data;
use log_derive::logfn;
use log_derive::logfn_inputs;
use serde_derive::Deserialize;
use serde_derive::Serialize;

use super::favourite::Favourite;
use crate::storage::markdown::config::MarkdownConfig;

/// Account configuration
#[derive(Serialize, Deserialize, Debug)]
pub struct AccountConfig {
    favourite: Favourite,
    mastodon: Data,
    markdown: Option<MarkdownConfig>,
    // joplin: Option<JoplinConfig>,
    // org: Option<OrgConfig>,
}

impl AccountConfig {
    /// Create an empty account configuration.
    #[logfn(Trace)]
    pub fn new(configuration: Data) -> Self {
        Self {
            mastodon: configuration,
            favourite: Favourite::default(),
            markdown: None,
        }
    }

    /// Return the top favourite for the account.
    #[logfn(Trace)]
    pub fn top_favourite(&self) -> String {
        self.favourite.last()
    }

    /// The Mastodon configuration for the account.
    pub fn mastodon(&self) -> Data {
        self.mastodon.clone()
    }

    /// Set the Markdown configuration.
    #[logfn_inputs(Trace)]
    pub fn set_markdown(&mut self, config: MarkdownConfig) {
        self.markdown = Some(config);
    }
}
