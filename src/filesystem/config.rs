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

use serde_derive::Deserialize;
use serde_derive::Serialize;

/// Configuration for the Filesystem backend
#[derive(Serialize, Deserialize, Debug)]
pub struct FilesystemConfig {
    /// Path where files will be stored.
    pub path: String,
}
