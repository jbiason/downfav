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
use crate::storage::data::Data;

// XXX Easily could be tied to the HTML-to-Org parser.
pub fn make_markdown(status: &Data) -> String {
    let base_content = html2md::parse_html(&status.text);
    let title = &status.title;

    let mut result = String::new();
    if title.len() > 0 {
        result.push_str(title);
        result.push_str("\n\n");
    }

    result.push_str(&base_content);

    if !status.source.is_empty() {
        result.push_str("\n\n");
        result.push_str(&status.source);
    }

    result
}
