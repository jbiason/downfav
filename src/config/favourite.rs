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

use serde_derive::Deserialize;
use serde_derive::Serialize;

/// The last seen favourite
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Favourite {
    last: Option<String>,
}

impl Favourite {
    /// Return the last favourite
    ///
    /// For example, if there is a favourite ID, returns it.
    ///
    /// ```
    /// let favourite = Favourite { last: Some("123".into()) };
    /// assert_eq!(favourite.last(), "123".into());
    /// ```
    ///
    /// ... but if there isn't, it returns 0.
    ///
    /// ```
    /// let favourite = Favourite { last: None };
    /// assert_eq!(favourite.last(), "0".into());
    /// ```
    pub fn last(&self) -> String {
        match &self.last {
            Some(last) => last.to_string(),
            None => "0".into(),
        }
    }
}
